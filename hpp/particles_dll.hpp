// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-24 17:35:37.601127800 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: particles.dll
        // Class count: 338
        // Enum count: 77
        namespace particles_dll {
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
            // Member count: 7
            enum class Detail2Combo_t : uint32_t {
                DETAIL_2_COMBO_UNINITIALIZED = 0xFFFFFFFF,
                DETAIL_2_COMBO_OFF = 0x0,
                DETAIL_2_COMBO_ADD = 0x1,
                DETAIL_2_COMBO_ADD_SELF_ILLUM = 0x2,
                DETAIL_2_COMBO_MOD2X = 0x3,
                DETAIL_2_COMBO_MUL = 0x4,
                DETAIL_2_COMBO_CROSSFADE = 0x5
            };
            // Alignment: 4
            // Member count: 4
            enum class MissingParentInheritBehavior_t : uint32_t {
                MISSING_PARENT_DO_NOTHING = 0xFFFFFFFF,
                MISSING_PARENT_KILL = 0x0,
                MISSING_PARENT_FIND_NEW = 0x1,
                MISSING_PARENT_SAME_INDEX = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleTraceMissBehavior_t : uint32_t {
                PARTICLE_TRACE_MISS_BEHAVIOR_NONE = 0x0,
                PARTICLE_TRACE_MISS_BEHAVIOR_KILL = 0x1,
                PARTICLE_TRACE_MISS_BEHAVIOR_TRACE_END = 0x2
            };
            // Alignment: 4
            // Member count: 7
            enum class PFuncVisualizationType_t : uint32_t {
                PFUNC_VISUALIZATION_SPHERE_WIREFRAME = 0x0,
                PFUNC_VISUALIZATION_SPHERE_SOLID = 0x1,
                PFUNC_VISUALIZATION_BOX = 0x2,
                PFUNC_VISUALIZATION_RING = 0x3,
                PFUNC_VISUALIZATION_PLANE = 0x4,
                PFUNC_VISUALIZATION_LINE = 0x5,
                PFUNC_VISUALIZATION_CYLINDER = 0x6
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleVRHandChoiceList_t : uint32_t {
                PARTICLE_VRHAND_LEFT = 0x0,
                PARTICLE_VRHAND_RIGHT = 0x1,
                PARTICLE_VRHAND_CP = 0x2,
                PARTICLE_VRHAND_CP_OBJECT = 0x3
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleReplicationMode_t : uint32_t {
                PARTICLE_REPLICATIONMODE_NONE = 0x0,
                PARTICLE_REPLICATIONMODE_REPLICATE_FOR_EACH_PARENT_PARTICLE = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleEntityPos_t : uint32_t {
                PARTICLE_ABS_ORIGIN = 0x0,
                PARTICLE_WORLDSPACE_CENTER = 0x1,
                PARTICLE_EYES = 0x2,
                PARTICLE_FLASHLIGHT = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleFanType_t : uint32_t {
                PARTICLE_FAN_TYPE_FAN = 0x0,
                PARTICLE_FAN_TYPE_ROTOR_WASH = 0x1,
                PARTICLE_FAN_TYPE_RADIAL = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class PetGroundType_t : uint32_t {
                PET_GROUND_NONE = 0x0,
                PET_GROUND_GRID = 0x1,
                PET_GROUND_PLANE = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class InheritableBoolType_t : uint32_t {
                INHERITABLE_BOOL_INHERIT = 0x0,
                INHERITABLE_BOOL_FALSE = 0x1,
                INHERITABLE_BOOL_TRUE = 0x2
            };
            // Alignment: 4
            // Member count: 6
            enum class ParticlePostProcessPriorityGroup_t : uint32_t {
                PARTICLE_POST_PROCESS_PRIORITY_LEVEL_VOLUME = 0x0,
                PARTICLE_POST_PROCESS_PRIORITY_LEVEL_OVERRIDE = 0x1,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_EFFECT = 0x2,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_LOW = 0x3,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_HIGH = 0x4,
                PARTICLE_POST_PROCESS_PRIORITY_GLOBAL_UI = 0x5
            };
            // Alignment: 4
            // Member count: 7
            enum class ParticleCollisionGroup_t : uint32_t {
                PARTICLE_COLLISION_GROUP_DEFAULT = 0x4,
                PARTICLE_COLLISION_GROUP_DEBRIS = 0x5,
                PARTICLE_COLLISION_GROUP_INTERACTIVE = 0x7,
                PARTICLE_COLLISION_GROUP_PLAYER = 0x8,
                PARTICLE_COLLISION_GROUP_VEHICLE = 0xA,
                PARTICLE_COLLISION_GROUP_NPC = 0xC,
                PARTICLE_COLLISION_GROUP_PROPS = 0x18
            };
            // Alignment: 4
            // Member count: 4
            enum class DetailCombo_t : uint32_t {
                DETAIL_COMBO_OFF = 0x0,
                DETAIL_COMBO_ADD = 0x1,
                DETAIL_COMBO_ADD_SELF_ILLUM = 0x2,
                DETAIL_COMBO_MOD2X = 0x3
            };
            // Alignment: 4
            // Member count: 12
            enum class ScalarExpressionType_t : uint32_t {
                SCALAR_EXPRESSION_UNINITIALIZED = 0xFFFFFFFF,
                SCALAR_EXPRESSION_ADD = 0x0,
                SCALAR_EXPRESSION_SUBTRACT = 0x1,
                SCALAR_EXPRESSION_MUL = 0x2,
                SCALAR_EXPRESSION_DIVIDE = 0x3,
                SCALAR_EXPRESSION_INPUT_1 = 0x4,
                SCALAR_EXPRESSION_MIN = 0x5,
                SCALAR_EXPRESSION_MAX = 0x6,
                SCALAR_EXPRESSION_MOD = 0x7,
                SCALAR_EXPRESSION_EQUAL = 0x8,
                SCALAR_EXPRESSION_GT = 0x9,
                SCALAR_EXPRESSION_LT = 0xA
            };
            // Alignment: 4
            // Member count: 14
            enum class SpriteCardPerParticleScale_t : uint32_t {
                SPRITECARD_TEXTURE_PP_SCALE_NONE = 0x0,
                SPRITECARD_TEXTURE_PP_SCALE_PARTICLE_AGE = 0x1,
                SPRITECARD_TEXTURE_PP_SCALE_ANIMATION_FRAME = 0x2,
                SPRITECARD_TEXTURE_PP_SCALE_SHADER_EXTRA_DATA1 = 0x3,
                SPRITECARD_TEXTURE_PP_SCALE_SHADER_EXTRA_DATA2 = 0x4,
                SPRITECARD_TEXTURE_PP_SCALE_PARTICLE_ALPHA = 0x5,
                SPRITECARD_TEXTURE_PP_SCALE_SHADER_RADIUS = 0x6,
                SPRITECARD_TEXTURE_PP_SCALE_ROLL = 0x7,
                SPRITECARD_TEXTURE_PP_SCALE_YAW = 0x8,
                SPRITECARD_TEXTURE_PP_SCALE_PITCH = 0x9,
                SPRITECARD_TEXTURE_PP_SCALE_RANDOM = 0xA,
                SPRITECARD_TEXTURE_PP_SCALE_NEG_RANDOM = 0xB,
                SPRITECARD_TEXTURE_PP_SCALE_RANDOM_TIME = 0xC,
                SPRITECARD_TEXTURE_PP_SCALE_NEG_RANDOM_TIME = 0xD
            };
            // Alignment: 4
            // Member count: 2
            enum class BlurFilterType_t : uint32_t {
                BLURFILTER_GAUSSIAN = 0x0,
                BLURFILTER_BOX = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class StandardLightingAttenuationStyle_t : uint32_t {
                LIGHT_STYLE_OLD = 0x0,
                LIGHT_STYLE_NEW = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleParentSetMode_t : uint32_t {
                PARTICLE_SET_PARENT_NO = 0x0,
                PARTICLE_SET_PARENT_IMMEDIATE = 0x1,
                PARTICLE_SET_PARENT_ROOT = 0x2
            };
            // Alignment: 4
            // Member count: 6
            enum class ParticleLightingQuality_t : uint32_t {
                PARTICLE_LIGHTING_PER_PARTICLE = 0x0,
                PARTICLE_LIGHTING_PER_VERTEX = 0x1,
                PARTICLE_LIGHTING_PER_PIXEL = 0xFFFFFFFF,
                PARTICLE_LIGHTING_OVERRIDE_POSITION = 0x2,
                PARTICLE_LIGHTING_OVERRIDE_COLOR = 0x3,
                PARTICLE_LIGHTING_ADD_EXTRA_LIGHT = 0x4
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleVolumetricSmokeCreationType_t : uint32_t {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_CONTINUOUS = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_IMPULSE = 0x1
            };
            // Alignment: 4
            // Member count: 8
            enum class SetStatisticExpressionType_t : uint32_t {
                SET_EXPRESSION_UNINITIALIZED = 0xFFFFFFFF,
                SET_EXPRESSION_SUM = 0x0,
                SET_EXPRESSION_MEAN = 0x1,
                SET_EXPRESSION_MEDIAN = 0x2,
                SET_EXPRESSION_MODE = 0x3,
                SET_EXPRESSION_STANDARD_DEVIATION = 0x4,
                SET_EXPRESSION_MIN = 0x5,
                SET_EXPRESSION_MAX = 0x6
            };
            // Alignment: 4
            // Member count: 13
            enum class EventTypeSelection_t : uint32_t {
                PARTICLE_EVENT_TYPE_MASK_NONE = 0x0,
                PARTICLE_EVENT_TYPE_MASK_SPAWNED = 0x1,
                PARTICLE_EVENT_TYPE_MASK_KILLED = 0x2,
                PARTICLE_EVENT_TYPE_MASK_COLLISION = 0x4,
                PARTICLE_EVENT_TYPE_MASK_FIRST_COLLISION = 0x8,
                PARTICLE_EVENT_TYPE_MASK_COLLISION_STOPPED = 0x10,
                PARTICLE_EVENT_TYPE_MASK_KILLED_ON_COLLISION = 0x20,
                PARTICLE_EVENT_TYPE_MASK_KILLED_ON_CULL = 0x400,
                PARTICLE_EVENT_TYPE_MASK_CULLED_ON_SPAWN = 0x800,
                PARTICLE_EVENT_TYPE_MASK_USER_1 = 0x40,
                PARTICLE_EVENT_TYPE_MASK_USER_2 = 0x80,
                PARTICLE_EVENT_TYPE_MASK_USER_3 = 0x100,
                PARTICLE_EVENT_TYPE_MASK_USER_4 = 0x200
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleMassMode_t : uint32_t {
                PARTICLE_MASSMODE_RADIUS_CUBED = 0x0,
                PARTICLE_MASSMODE_RADIUS_SQUARED = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleHitboxBiasType_t : uint32_t {
                PARTICLE_HITBOX_BIAS_ENTITY = 0x0,
                PARTICLE_HITBOX_BIAS_HITBOX = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class ParticleControlPointAxis_t : uint32_t {
                PARTICLE_CP_AXIS_X = 0x0,
                PARTICLE_CP_AXIS_Y = 0x1,
                PARTICLE_CP_AXIS_Z = 0x2,
                PARTICLE_CP_AXIS_NEGATIVE_X = 0x3,
                PARTICLE_CP_AXIS_NEGATIVE_Y = 0x4,
                PARTICLE_CP_AXIS_NEGATIVE_Z = 0x5
            };
            // Alignment: 4
            // Member count: 12
            enum class ParticlePinDistance_t : uint32_t {
                PARTICLE_PIN_DISTANCE_NONE = 0xFFFFFFFF,
                PARTICLE_PIN_DISTANCE_NEIGHBOR = 0x0,
                PARTICLE_PIN_DISTANCE_FARTHEST = 0x1,
                PARTICLE_PIN_DISTANCE_FIRST = 0x2,
                PARTICLE_PIN_DISTANCE_LAST = 0x3,
                PARTICLE_PIN_DISTANCE_CENTER = 0x5,
                PARTICLE_PIN_DISTANCE_CP = 0x6,
                PARTICLE_PIN_DISTANCE_CP_PAIR_EITHER = 0x7,
                PARTICLE_PIN_DISTANCE_CP_PAIR_BOTH = 0x8,
                PARTICLE_PIN_SPEED = 0x9,
                PARTICLE_PIN_COLLECTION_AGE = 0xA,
                PARTICLE_PIN_FLOAT_VALUE = 0xB
            };
            // Alignment: 4
            // Member count: 7
            enum class VectorFloatExpressionType_t : uint32_t {
                VECTOR_FLOAT_EXPRESSION_UNINITIALIZED = 0xFFFFFFFF,
                VECTOR_FLOAT_EXPRESSION_DOTPRODUCT = 0x0,
                VECTOR_FLOAT_EXPRESSION_DISTANCE = 0x1,
                VECTOR_FLOAT_EXPRESSION_DISTANCESQR = 0x2,
                VECTOR_FLOAT_EXPRESSION_INPUT1_LENGTH = 0x3,
                VECTOR_FLOAT_EXPRESSION_INPUT1_LENGTHSQR = 0x4,
                VECTOR_FLOAT_EXPRESSION_INPUT1_NOISE = 0x5
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleFogType_t : uint32_t {
                PARTICLE_FOG_GAME_DEFAULT = 0x0,
                PARTICLE_FOG_ENABLED = 0x1,
                PARTICLE_FOG_DISABLED = 0x2
            };
            // Alignment: 4
            // Member count: 10
            enum class VectorExpressionType_t : uint32_t {
                VECTOR_EXPRESSION_UNINITIALIZED = 0xFFFFFFFF,
                VECTOR_EXPRESSION_ADD = 0x0,
                VECTOR_EXPRESSION_SUBTRACT = 0x1,
                VECTOR_EXPRESSION_MUL = 0x2,
                VECTOR_EXPRESSION_DIVIDE = 0x3,
                VECTOR_EXPRESSION_INPUT_1 = 0x4,
                VECTOR_EXPRESSION_MIN = 0x5,
                VECTOR_EXPRESSION_MAX = 0x6,
                VECTOR_EXPRESSION_CROSSPRODUCT = 0x7,
                VECTOR_EXPRESSION_LERP = 0x8
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleMultiSegmentInputSelection_t : uint32_t {
                PARTICLE_MULTISEGMENT_SELECTION_FLOAT = 0x0,
                PARTICLE_MULTISEGMENT_SELECTION_STRING = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleRotationLockType_t : uint32_t {
                PARTICLE_ROTATION_LOCK_NONE = 0x0,
                PARTICLE_ROTATION_LOCK_ROTATIONS = 0x1,
                PARTICLE_ROTATION_LOCK_NORMAL = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class HitboxLerpType_t : uint32_t {
                HITBOX_LERP_LIFETIME = 0x0,
                HITBOX_LERP_CONSTANT = 0x1
            };
            // Alignment: 4
            // Member count: 11
            enum class ParticleAttrBoxFlags_t : uint32_t {
                PARTICLE_ATTR_BOX_FLAGS_NONE = 0x0,
                PARTICLE_ATTR_BOX_FLAGS_WATER = 0x1,
                PARTICLE_ATTR_BOX_FLAGS_ON_FIRE = 0x2,
                PARTICLE_ATTR_BOX_FLAGS_ELECTRIFIED = 0x4,
                PARTICLE_ATTR_BOX_FLAGS_ASLEEP = 0x8,
                PARTICLE_ATTR_BOX_FLAGS_FROZEN = 0x10,
                PARTICLE_ATTR_BOX_FLAGS_TIMED_DECAY = 0x20,
                PARTICLE_ATTR_BOX_FLAGS_DISABLE_NONSTATIC_COLLISION = 0x40,
                PARTICLE_ATTR_BOX_FLAGS_WAKE_DECAY = 0x80,
                PARTICLE_ATTR_BOX_FLAGS_MOTION_DISABLED = 0x100,
                PARTICLE_ATTR_BOX_FLAGS_ZERO_GRAVITY = 0x200
            };
            // Alignment: 4
            // Member count: 5
            enum class ParticleTopology_t : uint32_t {
                PARTICLE_TOPOLOGY_POINTS = 0x0,
                PARTICLE_TOPOLOGY_LINES = 0x1,
                PARTICLE_TOPOLOGY_TRIS = 0x2,
                PARTICLE_TOPOLOGY_QUADS = 0x3,
                PARTICLE_TOPOLOGY_CUBES = 0x4
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleLightBehaviorChoiceList_t : uint32_t {
                PARTICLE_LIGHT_BEHAVIOR_FOLLOW_DIRECTION = 0x0,
                PARTICLE_LIGHT_BEHAVIOR_ROPE = 0x1,
                PARTICLE_LIGHT_BEHAVIOR_TRAILS = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class ModelHitboxType_t : uint32_t {
                MODEL_HITBOX_TYPE_STANDARD = 0x0,
                MODEL_HITBOX_TYPE_RAW_BONES = 0x1,
                MODEL_HITBOX_TYPE_RENDERBOUNDS = 0x2,
                MODEL_HITBOX_TYPE_SNAPSHOT = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleMultiSegmentCountSelection_t : uint32_t {
                PARTICLE_MULTISEGMENT_SEG_COUNT_7 = 0x7,
                PARTICLE_MULTISEGMENT_SEG_COUNT_14 = 0xE,
                PARTICLE_MULTISEGMENT_SEG_COUNT_16 = 0x10
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleOrientationType_t : uint32_t {
                PARTICLE_ORIENTATION_NONE = 0x0,
                PARTICLE_ORIENTATION_VELOCITY = 0x1,
                PARTICLE_ORIENTATION_NORMAL = 0x2,
                PARTICLE_ORIENTATION_ROTATION = 0x4
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleTraceSet_t : uint32_t {
                PARTICLE_TRACE_SET_ALL = 0x0,
                PARTICLE_TRACE_SET_STATIC = 0x1,
                PARTICLE_TRACE_SET_STATIC_AND_KEYFRAMED = 0x2,
                PARTICLE_TRACE_SET_DYNAMIC = 0x3
            };
            // Alignment: 4
            // Member count: 7
            enum class ParticleTextureLayerBlendType_t : uint32_t {
                SPRITECARD_TEXTURE_BLEND_MULTIPLY = 0x0,
                SPRITECARD_TEXTURE_BLEND_MOD2X = 0x1,
                SPRITECARD_TEXTURE_BLEND_REPLACE = 0x2,
                SPRITECARD_TEXTURE_BLEND_ADD = 0x3,
                SPRITECARD_TEXTURE_BLEND_SUBTRACT = 0x4,
                SPRITECARD_TEXTURE_BLEND_AVERAGE = 0x5,
                SPRITECARD_TEXTURE_BLEND_LUMINANCE = 0x6
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleSelection_t : uint32_t {
                PARTICLE_SELECTION_FIRST = 0x0,
                PARTICLE_SELECTION_LAST = 0x1,
                PARTICLE_SELECTION_NUMBER = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleToolsState_t : uint32_t {
                PARTICLE_TOOLS_STATE_ALWAYS_ON = 0xFFFFFFFF,
                PARTICLE_TOOLS_STATE_TOOLS_ONLY = 0x0,
                PARTICLE_TOOLS_STATE_GAME_ONLY = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class SnapshotIndexType_t : uint32_t {
                SNAPSHOT_INDEX_INCREMENT = 0x0,
                SNAPSHOT_INDEX_DIRECT = 0x1
            };
            // Alignment: 4
            // Member count: 7
            enum class ParticleOutputBlendMode_t : uint32_t {
                PARTICLE_OUTPUT_BLEND_MODE_ALPHA = 0x0,
                PARTICLE_OUTPUT_BLEND_MODE_ADD = 0x1,
                PARTICLE_OUTPUT_BLEND_MODE_BLEND_ADD = 0x2,
                PARTICLE_OUTPUT_BLEND_MODE_HALF_BLEND_ADD = 0x3,
                PARTICLE_OUTPUT_BLEND_MODE_NEG_HALF_BLEND_ADD = 0x4,
                PARTICLE_OUTPUT_BLEND_MODE_MOD2X = 0x5,
                PARTICLE_OUTPUT_BLEND_MODE_LIGHTEN = 0x6
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleLightnintBranchBehavior_t : uint32_t {
                PARTICLE_LIGHTNING_BRANCH_CURRENT_DIR = 0x0,
                PARTICLE_LIGHTNING_BRANCH_ENDPOINT_DIR = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class MaterialProxyType_t : uint32_t {
                MATERIAL_PROXY_STATUS_EFFECT = 0x0,
                MATERIAL_PROXY_TINT = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleDepthFeatheringMode_t : uint32_t {
                PARTICLE_DEPTH_FEATHERING_OFF = 0x0,
                PARTICLE_DEPTH_FEATHERING_ON_OPTIONAL = 0x1,
                PARTICLE_DEPTH_FEATHERING_ON_REQUIRED = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleLightUnitChoiceList_t : uint32_t {
                PARTICLE_LIGHT_UNIT_CANDELAS = 0x0,
                PARTICLE_LIGHT_UNIT_LUMENS = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleMultiSegmentSpecialCharacter_t : uint32_t {
                PARTICLE_MULTISEGMENT_SPECIAL_NONE = 0xFFFFFFFF,
                PARTICLE_MULTISEGMENT_SPECIAL_DECIMAL = 0x0,
                PARTICLE_MULTISEGMENT_SPECIAL_COLON = 0x1,
                PARTICLE_MULTISEGMENT_SPECIAL_DEGREES = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleFalloffFunction_t : uint32_t {
                PARTICLE_FALLOFF_CONSTANT = 0x0,
                PARTICLE_FALLOFF_LINEAR = 0x1,
                PARTICLE_FALLOFF_EXPONENTIAL = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleSequenceCropOverride_t : uint32_t {
                PARTICLE_SEQUENCE_CROP_OVERRIDE_DEFAULT = 0xFFFFFFFF,
                PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_OFF = 0x0,
                PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_ON = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleDetailLevel_t : uint32_t {
                PARTICLEDETAIL_LOW = 0x0,
                PARTICLEDETAIL_MEDIUM = 0x1,
                PARTICLEDETAIL_HIGH = 0x2,
                PARTICLEDETAIL_ULTRA = 0x3
            };
            // Alignment: 4
            // Member count: 4
            enum class BBoxVolumeType_t : uint32_t {
                BBOX_VOLUME = 0x0,
                BBOX_DIMENSIONS = 0x1,
                BBOX_MINS_MAXS = 0x2,
                BBOX_RADIUS = 0x3
            };
            // Alignment: 4
            // Member count: 12
            enum class SpriteCardTextureType_t : uint32_t {
                SPRITECARD_TEXTURE_DIFFUSE = 0x0,
                SPRITECARD_TEXTURE_ZOOM = 0x1,
                SPRITECARD_TEXTURE_1D_COLOR_LOOKUP = 0x2,
                SPRITECARD_TEXTURE_UVDISTORTION = 0x3,
                SPRITECARD_TEXTURE_UVDISTORTION_ZOOM = 0x4,
                SPRITECARD_TEXTURE_NORMALMAP = 0x5,
                SPRITECARD_TEXTURE_ANIMMOTIONVEC = 0x6,
                SPRITECARD_TEXTURE_SPHERICAL_HARMONICS_A = 0x7,
                SPRITECARD_TEXTURE_SPHERICAL_HARMONICS_B = 0x8,
                SPRITECARD_TEXTURE_SPHERICAL_HARMONICS_C = 0x9,
                SPRITECARD_TEXTURE_DEPTH = 0xA,
                SPRITECARD_TEXTURE_ILLUMINATION_GRADIENT = 0xB
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleAlphaReferenceType_t : uint32_t {
                PARTICLE_ALPHA_REFERENCE_ALPHA_ALPHA = 0x0,
                PARTICLE_ALPHA_REFERENCE_OPAQUE_ALPHA = 0x1,
                PARTICLE_ALPHA_REFERENCE_ALPHA_OPAQUE = 0x2,
                PARTICLE_ALPHA_REFERENCE_OPAQUE_OPAQUE = 0x3
            };
            // Alignment: 4
            // Member count: 15
            enum class SpriteCardTextureChannel_t : uint32_t {
                SPRITECARD_TEXTURE_CHANNEL_MIX_RGB = 0x0,
                SPRITECARD_TEXTURE_CHANNEL_MIX_RGBA = 0x1,
                SPRITECARD_TEXTURE_CHANNEL_MIX_A = 0x2,
                SPRITECARD_TEXTURE_CHANNEL_MIX_RGB_A = 0x3,
                SPRITECARD_TEXTURE_CHANNEL_MIX_RGB_ALPHAMASK = 0x4,
                SPRITECARD_TEXTURE_CHANNEL_MIX_RGB_RGBMASK = 0x5,
                SPRITECARD_TEXTURE_CHANNEL_MIX_RGBA_RGBALPHA = 0x6,
                SPRITECARD_TEXTURE_CHANNEL_MIX_A_RGBALPHA = 0x7,
                SPRITECARD_TEXTURE_CHANNEL_MIX_RGB_A_RGBALPHA = 0x8,
                SPRITECARD_TEXTURE_CHANNEL_MIX_R = 0x9,
                SPRITECARD_TEXTURE_CHANNEL_MIX_G = 0xA,
                SPRITECARD_TEXTURE_CHANNEL_MIX_B = 0xB,
                SPRITECARD_TEXTURE_CHANNEL_MIX_RALPHA = 0xC,
                SPRITECARD_TEXTURE_CHANNEL_MIX_GALPHA = 0xD,
                SPRITECARD_TEXTURE_CHANNEL_MIX_BALPHA = 0xE
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleVolumetricSmokeType_t : uint32_t {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_EMISSION = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_SINK = 0x1,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_REPEL = 0x2,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_TRACE = 0x3
            };
            // Alignment: 4
            // Member count: 4
            enum class RenderModelSubModelFieldType_t : uint32_t {
                SUBMODEL_AS_BODYGROUP_SUBMODEL = 0x0,
                SUBMODEL_AS_MESHGROUP_INDEX = 0x1,
                SUBMODEL_AS_MESHGROUP_MASK = 0x2,
                SUBMODEL_IGNORED_USE_MODEL_DEFAULT_MESHGROUP_MASK = 0x3
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleHitboxDataSelection_t : uint32_t {
                PARTICLE_HITBOX_AVERAGE_SPEED = 0x0,
                PARTICLE_HITBOX_COUNT = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class ParticleOrientationChoiceList_t : uint32_t {
                PARTICLE_ORIENTATION_SCREEN_ALIGNED = 0x0,
                PARTICLE_ORIENTATION_SCREEN_Z_ALIGNED = 0x1,
                PARTICLE_ORIENTATION_WORLD_Z_ALIGNED = 0x2,
                PARTICLE_ORIENTATION_ALIGN_TO_PARTICLE_NORMAL = 0x3,
                PARTICLE_ORIENTATION_SCREENALIGN_TO_PARTICLE_NORMAL = 0x4,
                PARTICLE_ORIENTATION_FULL_3AXIS_ROTATION = 0x5
            };
            // Alignment: 4
            // Member count: 5
            enum class ParticleCollisionMode_t : uint32_t {
                COLLISION_MODE_PER_PARTICLE_TRACE = 0x3,
                COLLISION_MODE_USE_NEAREST_TRACE = 0x2,
                COLLISION_MODE_PER_FRAME_PLANESET = 0x1,
                COLLISION_MODE_INITIAL_TRACE_DOWN = 0x0,
                COLLISION_MODE_DISABLED = 0xFFFFFFFF
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleSortingChoiceList_t : uint32_t {
                PARTICLE_SORTING_NEAREST = 0x0,
                PARTICLE_SORTING_CREATION_TIME = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleEndcapMode_t : uint32_t {
                PARTICLE_ENDCAP_ALWAYS_ON = 0xFFFFFFFF,
                PARTICLE_ENDCAP_ENDCAP_OFF = 0x0,
                PARTICLE_ENDCAP_ENDCAP_ON = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class ClosestPointTestType_t : uint32_t {
                PARTICLE_CLOSEST_TYPE_BOX = 0x0,
                PARTICLE_CLOSEST_TYPE_CAPSULE = 0x1,
                PARTICLE_CLOSEST_TYPE_HYBRID = 0x2
            };
            // Alignment: 4
            // Member count: 6
            enum class ParticleImpulseType_t : uint32_t {
                IMPULSE_TYPE_NONE = 0x0,
                IMPULSE_TYPE_GENERIC = 0x1,
                IMPULSE_TYPE_ROPE = 0x2,
                IMPULSE_TYPE_EXPLOSION = 0x4,
                IMPULSE_TYPE_EXPLOSION_UNDERWATER = 0x8,
                IMPULSE_TYPE_PARTICLE_SYSTEM = 0x10
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleLiquidContents_t : uint32_t {
                PARTICLE_LIQUID_NONE = 0x0,
                PARTICLE_LIQUID_OIL = 0x1,
                PARTICLE_LIQUID_WATER = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class SpriteCardShaderType_t : uint32_t {
                SPRITECARD_SHADER_BASE = 0x0,
                SPRITECARD_SHADER_CUSTOM = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class ParticleOmni2LightTypeChoiceList_t : uint32_t {
                PARTICLE_OMNI2_LIGHT_TYPE_POINT = 0x0,
                PARTICLE_OMNI2_LIGHT_TYPE_SPHERE = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleLightFogLightingMode_t : uint32_t {
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_NONE = 0x0,
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC = 0x2,
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC_NOSHADOWS = 0x4
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleLightTypeChoiceList_t : uint32_t {
                PARTICLE_LIGHT_TYPE_POINT = 0x0,
                PARTICLE_LIGHT_TYPE_SPOT = 0x1,
                PARTICLE_LIGHT_TYPE_FX = 0x2,
                PARTICLE_LIGHT_TYPE_CAPSULE = 0x3
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleOrientationSetMode_t : uint32_t {
                PARTICLE_ORIENTATION_SET_NONE = 0xFFFFFFFF,
                PARTICLE_ORIENTATION_SET_FROM_VELOCITY = 0x0,
                PARTICLE_ORIENTATION_SET_FROM_NORMAL = 0x1,
                PARTICLE_ORIENTATION_SET_FROM_ROTATIONS = 0x2
            };
            // Alignment: 8
            // Member count: 10
            enum class ParticleCollisionMask_t : uint64_t {
                PARTICLE_MASK_ALL = 0xFFFFFFFFFFFFFFFF,
                PARTICLE_MASK_SOLID = 0xC3001,
                PARTICLE_MASK_WATER = 0x18000,
                PARTICLE_MASK_SOLID_WATER = 0xDB001,
                PARTICLE_MASK_SHOT = 0x1C1003,
                PARTICLE_MASK_SHOT_BRUSHONLY = 0x101001,
                PARTICLE_MASK_SHOT_HULL = 0x1C3001,
                PARTICLE_MASK_OPAQUE = 0x80,
                PARTICLE_MASK_DEFAULTPLAYERSOLID = 0xC3011,
                PARTICLE_MASK_NPCSOLID = 0xC3021
            };
            // Alignment: 4
            // Member count: 2
            enum class TextureRepetitionMode_t : uint32_t {
                TEXTURE_REPETITION_PARTICLE = 0x0,
                TEXTURE_REPETITION_PATH = 0x1
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
            // Field count: 0
            namespace IParticleCollection {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_Decay {
                constexpr std::ptrdiff_t m_bRopeDecay = 0x1D8; // bool
                constexpr std::ptrdiff_t m_bForcePreserveParticleOrder = 0x1D9; // bool
            }
            // Parent: None
            // Field count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertySortPriority
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RenderDeferredLight {
                constexpr std::ptrdiff_t m_bUseAlphaTestWindow = 0x228; // bool
                constexpr std::ptrdiff_t m_bUseTexture = 0x229; // bool
                constexpr std::ptrdiff_t m_flRadiusScale = 0x22C; // float32
                constexpr std::ptrdiff_t m_flAlphaScale = 0x230; // float32
                constexpr std::ptrdiff_t m_nAlpha2Field = 0x234; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecColorScale = 0x238; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nColorBlendType = 0x8F0; // ParticleColorBlendType_t
                constexpr std::ptrdiff_t m_flLightDistance = 0x8F4; // float32
                constexpr std::ptrdiff_t m_flStartFalloff = 0x8F8; // float32
                constexpr std::ptrdiff_t m_flDistanceFalloff = 0x8FC; // float32
                constexpr std::ptrdiff_t m_flSpotFoV = 0x900; // float32
                constexpr std::ptrdiff_t m_nAlphaTestPointField = 0x904; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nAlphaTestRangeField = 0x908; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nAlphaTestSharpnessField = 0x90C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_hTexture = 0x910; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_nHSVShiftControlPoint = 0x918; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapTransformToVelocity {
                constexpr std::ptrdiff_t m_TransformInput = 0x1D8; // CParticleTransformInput
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace CollisionGroupContext_t {
                constexpr std::ptrdiff_t m_nCollisionGroupNumber = 0x0; // int32
            }
            // Parent: None
            // Field count: 19
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_INIT_StatusEffectCitadel {
                constexpr std::ptrdiff_t m_flSFXColorWarpAmount = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flSFXNormalAmount = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flSFXMetalnessAmount = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flSFXRoughnessAmount = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flSFXSelfIllumAmount = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flSFXSScale = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flSFXSScrollX = 0x1F8; // float32
                constexpr std::ptrdiff_t m_flSFXSScrollY = 0x1FC; // float32
                constexpr std::ptrdiff_t m_flSFXSScrollZ = 0x200; // float32
                constexpr std::ptrdiff_t m_flSFXSOffsetX = 0x204; // float32
                constexpr std::ptrdiff_t m_flSFXSOffsetY = 0x208; // float32
                constexpr std::ptrdiff_t m_flSFXSOffsetZ = 0x20C; // float32
                constexpr std::ptrdiff_t m_nDetailCombo = 0x210; // DetailCombo_t
                constexpr std::ptrdiff_t m_flSFXSDetailAmount = 0x214; // float32
                constexpr std::ptrdiff_t m_flSFXSDetailScale = 0x218; // float32
                constexpr std::ptrdiff_t m_flSFXSDetailScrollX = 0x21C; // float32
                constexpr std::ptrdiff_t m_flSFXSDetailScrollY = 0x220; // float32
                constexpr std::ptrdiff_t m_flSFXSDetailScrollZ = 0x224; // float32
                constexpr std::ptrdiff_t m_flSFXSUseModelUVs = 0x228; // float32
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // UNUSED
            // F_NORMAL_MAP
            // F_TEXTURE_LAYERS
            // F_REFRACT_BLUR
            // F_PARTICLE_SHADOWS
            // F_DIRECTIONAL_LIGHTING
            // F_REVERSE_DEPTH
            // F_LIGHTEN_MODE
            // F_MIXED_RENDERING
            // F_PARTICLE_ORIENTATION
            // F_SATURATE_COLOR_PRE_ALPHA_BLEND
            // F_FOG_PARTICLES
            // F_SELF_ILLUM_OVER_1
            // F_PER_PIXEL_LIGHTING
            // F_HAS_NO_NORMAL_FOR_LIGHTING
            // F_NUM_SEQUENCES_PER_PARTICLE
            // F_SELF_ILLUM_PER_PARTICLE
            // F_REFRACT_SOLID
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertySortPriority
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            namespace C_OP_RenderSound {
                constexpr std::ptrdiff_t m_flDurationScale = 0x228; // float32
                constexpr std::ptrdiff_t m_flSndLvlScale = 0x22C; // float32
                constexpr std::ptrdiff_t m_flPitchScale = 0x230; // float32
                constexpr std::ptrdiff_t m_flVolumeScale = 0x234; // float32
                constexpr std::ptrdiff_t m_nSndLvlField = 0x238; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nDurationField = 0x23C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nPitchField = 0x240; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nVolumeField = 0x244; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nChannel = 0x248; // int32
                constexpr std::ptrdiff_t m_nCPReference = 0x24C; // int32
                constexpr std::ptrdiff_t m_pszSoundName = 0x250; // char[256]
                constexpr std::ptrdiff_t m_bSuppressStopSoundEvent = 0x350; // bool
            }
            // Parent: None
            // Field count: 19
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // COLLISION_MODE_PER_PARTICLE_TRACE
            // COLLISION_MODE_USE_NEAREST_TRACE
            namespace CParticleVisibilityInputs {
                constexpr std::ptrdiff_t m_flCameraBias = 0x0; // float32
                constexpr std::ptrdiff_t m_nCPin = 0x4; // int32
                constexpr std::ptrdiff_t m_flProxyRadius = 0x8; // float32
                constexpr std::ptrdiff_t m_flInputMin = 0xC; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x10; // float32
                constexpr std::ptrdiff_t m_flInputPixelVisFade = 0x14; // float32
                constexpr std::ptrdiff_t m_flNoPixelVisibilityFallback = 0x18; // float32
                constexpr std::ptrdiff_t m_flDistanceInputMin = 0x1C; // float32
                constexpr std::ptrdiff_t m_flDistanceInputMax = 0x20; // float32
                constexpr std::ptrdiff_t m_flDotInputMin = 0x24; // float32
                constexpr std::ptrdiff_t m_flDotInputMax = 0x28; // float32
                constexpr std::ptrdiff_t m_bDotCPAngles = 0x2C; // bool
                constexpr std::ptrdiff_t m_bDotCameraAngles = 0x2D; // bool
                constexpr std::ptrdiff_t m_flAlphaScaleMin = 0x30; // float32
                constexpr std::ptrdiff_t m_flAlphaScaleMax = 0x34; // float32
                constexpr std::ptrdiff_t m_flRadiusScaleMin = 0x38; // float32
                constexpr std::ptrdiff_t m_flRadiusScaleMax = 0x3C; // float32
                constexpr std::ptrdiff_t m_flRadiusScaleFOVBase = 0x40; // float32
                constexpr std::ptrdiff_t m_bRightEye = 0x44; // bool
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointsToParticle {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFirstControlPoint = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nNumControlPoints = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nFirstSourcePoint = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bReverse = 0x1E8; // bool
                constexpr std::ptrdiff_t m_bSetOrientation = 0x1E9; // bool
                constexpr std::ptrdiff_t m_nOrientationMode = 0x1EC; // ParticleOrientationSetMode_t
                constexpr std::ptrdiff_t m_nSetParent = 0x1F0; // ParticleParentSetMode_t
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapCPVelocityToVector {
                constexpr std::ptrdiff_t m_nControlPoint = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0x1E0; // float32
                constexpr std::ptrdiff_t m_bNormalize = 0x1E4; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_PointVectorAtNextParticle {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInterpolation = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bPrevious = 0x350; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // PARTICLE_WORLDSPACE_CENTER
            // PARTICLE_EYES
            // PARTICLE_FLASHLIGHT
            namespace ParticlePreviewBodyGroup_t {
                constexpr std::ptrdiff_t m_bodyGroupName = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_nValue = 0x8; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_OscillateScalarSimple {
                constexpr std::ptrdiff_t m_Rate = 0x1D8; // float32
                constexpr std::ptrdiff_t m_Frequency = 0x1DC; // float32
                constexpr std::ptrdiff_t m_nField = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOscMult = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOscAdd = 0x1E8; // float32
            }
            // Parent: None
            // Field count: 18
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_INIT_StatusEffect {
                constexpr std::ptrdiff_t m_nDetail2Combo = 0x1E0; // Detail2Combo_t
                constexpr std::ptrdiff_t m_flDetail2Rotation = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flDetail2Scale = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flDetail2BlendFactor = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flColorWarpIntensity = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flDiffuseWarpBlendToFull = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flEnvMapIntensity = 0x1F8; // float32
                constexpr std::ptrdiff_t m_flAmbientScale = 0x1FC; // float32
                constexpr std::ptrdiff_t m_specularColor = 0x200; // Color
                constexpr std::ptrdiff_t m_flSpecularScale = 0x204; // float32
                constexpr std::ptrdiff_t m_flSpecularExponent = 0x208; // float32
                constexpr std::ptrdiff_t m_flSpecularExponentBlendToFull = 0x20C; // float32
                constexpr std::ptrdiff_t m_flSpecularBlendToFull = 0x210; // float32
                constexpr std::ptrdiff_t m_rimLightColor = 0x214; // Color
                constexpr std::ptrdiff_t m_flRimLightScale = 0x218; // float32
                constexpr std::ptrdiff_t m_flReflectionsTintByBaseBlendToNone = 0x21C; // float32
                constexpr std::ptrdiff_t m_flMetalnessBlendToFull = 0x220; // float32
                constexpr std::ptrdiff_t m_flSelfIllumBlendToFull = 0x224; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_ConstrainDistance {
                constexpr std::ptrdiff_t m_fMinDistance = 0x1D8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_fMaxDistance = 0x348; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x4B8; // int32
                constexpr std::ptrdiff_t m_CenterOffset = 0x4BC; // Vector
                constexpr std::ptrdiff_t m_bGlobalCenter = 0x4C8; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_RandomVector {
                constexpr std::ptrdiff_t m_vecMin = 0x1E0; // Vector
                constexpr std::ptrdiff_t m_vecMax = 0x1EC; // Vector
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1F8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_randomnessParameters = 0x1FC; // CRandomNumberGeneratorParameters
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_INIT_InitialVelocityNoise {
                constexpr std::ptrdiff_t m_vecAbsVal = 0x1E0; // Vector
                constexpr std::ptrdiff_t m_vecAbsValInv = 0x1EC; // Vector
                constexpr std::ptrdiff_t m_vecOffsetLoc = 0x1F8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flOffset = 0x8B0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_vecOutputMin = 0xA20; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecOutputMax = 0x10D8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flNoiseScale = 0x1790; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flNoiseScaleLoc = 0x1900; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_TransformInput = 0x1A70; // CParticleTransformInput
                constexpr std::ptrdiff_t m_bIgnoreDt = 0x1AD8; // bool
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapScalarOnceTimed {
                constexpr std::ptrdiff_t m_bProportional = 0x1D8; // bool
                constexpr std::ptrdiff_t m_nFieldInput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flRemapTime = 0x1F4; // float32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_INIT_RandomNamedModelSequence {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_PlaneCull {
                constexpr std::ptrdiff_t m_nPlaneControlPoint = 0x1D8; // int32
                constexpr std::ptrdiff_t m_vecPlaneDirection = 0x1E0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_bLocalSpace = 0x898; // bool
                constexpr std::ptrdiff_t m_flPlaneOffset = 0x89C; // float32
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_VelocityRandom {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_fSpeedMin = 0x1E8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fSpeedMax = 0x358; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_LocalCoordinateSystemSpeedMin = 0x4C8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_LocalCoordinateSystemSpeedMax = 0xB80; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_bIgnoreDT = 0x1238; // bool
                constexpr std::ptrdiff_t m_randomnessParameters = 0x123C; // CRandomNumberGeneratorParameters
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ModelDampenMovement {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_bBoundBox = 0x1DC; // bool
                constexpr std::ptrdiff_t m_bOutside = 0x1DD; // bool
                constexpr std::ptrdiff_t m_bUseBones = 0x1DE; // bool
                constexpr std::ptrdiff_t m_HitboxSetName = 0x1DF; // char[128]
                constexpr std::ptrdiff_t m_vecPosOffset = 0x260; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_fDrag = 0x918; // float32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace CSpinUpdateBase {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_OrientTo2dDirection {
                constexpr std::ptrdiff_t m_vecInput = 0x1D8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flRotOffset = 0x890; // float32
                constexpr std::ptrdiff_t m_flSpinStrength = 0x894; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x898; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapDotProductToCP {
                constexpr std::ptrdiff_t m_nInputCP1 = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nInputCP2 = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nOutputCP = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nOutVectorField = 0x1EC; // int32
                constexpr std::ptrdiff_t m_flInputMin = 0x1F0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x360; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x4D0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x640; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_RemapParticleCountToNamedModelElementScalar {
                constexpr std::ptrdiff_t m_hModel = 0x210; // CStrongHandle<InfoForResourceTypeCModel>
                constexpr std::ptrdiff_t m_outputMinName = 0x218; // CUtlString
                constexpr std::ptrdiff_t m_outputMaxName = 0x220; // CUtlString
                constexpr std::ptrdiff_t m_bModelFromRenderer = 0x228; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointPositionToTimeOfDayValue {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_pszTimeOfDayParameter = 0x1E4; // char[128]
                constexpr std::ptrdiff_t m_vecDefaultValue = 0x264; // Vector
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_DecayMaintainCount {
                constexpr std::ptrdiff_t m_nParticlesToMaintain = 0x1D8; // int32
                constexpr std::ptrdiff_t m_flDecayDelay = 0x1DC; // float32
                constexpr std::ptrdiff_t m_nSnapshotControlPoint = 0x1E0; // int32
                constexpr std::ptrdiff_t m_strSnapshotSubset = 0x1E8; // CUtlString
                constexpr std::ptrdiff_t m_bLifespanDecay = 0x1F0; // bool
                constexpr std::ptrdiff_t m_flScale = 0x1F8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bKillNewest = 0x368; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_INIT_RandomModelSequence {
                constexpr std::ptrdiff_t m_ActivityName = 0x1E0; // char[256]
                constexpr std::ptrdiff_t m_SequenceName = 0x2E0; // char[256]
                constexpr std::ptrdiff_t m_hModel = 0x3E0; // CStrongHandle<InfoForResourceTypeCModel>
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ExternalGameImpulseForce {
                constexpr std::ptrdiff_t m_flForceScale = 0x1E8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bRopes = 0x358; // bool
                constexpr std::ptrdiff_t m_bRopesZOnly = 0x359; // bool
                constexpr std::ptrdiff_t m_bExplosions = 0x35A; // bool
                constexpr std::ptrdiff_t m_bParticles = 0x35B; // bool
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_RemapAverageHitboxSpeedtoCP {
                constexpr std::ptrdiff_t m_nInControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nOutControlPointNumber = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nField = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nHitboxDataType = 0x1EC; // ParticleHitboxDataSelection_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1F0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x360; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x4D0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x640; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nHeightControlPointNumber = 0x7B0; // int32
                constexpr std::ptrdiff_t m_vecComparisonVelocity = 0x7B8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_HitboxSetName = 0xE70; // char[128]
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomAlpha {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nAlphaMin = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nAlphaMax = 0x1E8; // int32
                constexpr std::ptrdiff_t m_flAlphaRandExponent = 0x1F4; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_NormalizeVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0x1DC; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RepeatedTriggerChildGroup {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1E0; // int32
                constexpr std::ptrdiff_t m_flClusterRefireTime = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flClusterSize = 0x358; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flClusterCooldown = 0x4C8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bLimitChildCount = 0x638; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapVelocityToVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0x1DC; // float32
                constexpr std::ptrdiff_t m_bNormalize = 0x1E0; // bool
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            namespace C_INIT_SetHitboxToClosest {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nDesiredHitbox = 0x1E4; // int32
                constexpr std::ptrdiff_t m_vecHitBoxScale = 0x1E8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_HitboxSetName = 0x8A0; // char[128]
                constexpr std::ptrdiff_t m_bUseBones = 0x920; // bool
                constexpr std::ptrdiff_t m_bUseClosestPointOnHitbox = 0x921; // bool
                constexpr std::ptrdiff_t m_nTestType = 0x924; // ClosestPointTestType_t
                constexpr std::ptrdiff_t m_flHybridRatio = 0x928; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bUpdatePosition = 0xA98; // bool
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_RingWave {
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flParticlesPerOrbit = 0x248; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInitialRadius = 0x3B8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flThickness = 0x528; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInitialSpeedMin = 0x698; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInitialSpeedMax = 0x808; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flRoll = 0x978; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flPitch = 0xAE8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flYaw = 0xC58; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bEvenDistribution = 0xDC8; // bool
                constexpr std::ptrdiff_t m_bXYVelocityOnly = 0xDC9; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomTrailLength {
                constexpr std::ptrdiff_t m_flMinLength = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flMaxLength = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flLengthRandExponent = 0x1E8; // float32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapScalar {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_bOldCode = 0x1F0; // bool
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_DistanceBetweenTransforms {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_TransformStart = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_TransformEnd = 0x248; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flInputMin = 0x2B0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x420; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x590; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x700; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flMaxTraceLength = 0x870; // float32
                constexpr std::ptrdiff_t m_flLOSScale = 0x874; // float32
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x878; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x8F8; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_bLOS = 0x8FC; // bool
                constexpr std::ptrdiff_t m_nSetMethod = 0x900; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_DecayOffscreen {
                constexpr std::ptrdiff_t m_flOffscreenTime = 0x1D8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // PARTICLE_CP_AXIS_Y
            // PARTICLE_CP_AXIS_Z
            // PARTICLE_CP_AXIS_NEGATIVE_X
            // PARTICLE_CP_AXIS_NEGATIVE_Y
            // PARTICLE_CP_AXIS_NEGATIVE_Z
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_CreateSequentialPath {
                constexpr std::ptrdiff_t m_fMaxDistance = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flNumToAssign = 0x1E4; // float32
                constexpr std::ptrdiff_t m_bLoop = 0x1E8; // bool
                constexpr std::ptrdiff_t m_bCPPairs = 0x1E9; // bool
                constexpr std::ptrdiff_t m_bSaveOffset = 0x1EA; // bool
                constexpr std::ptrdiff_t m_PathParams = 0x1F0; // CPathParameters
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            namespace C_OP_EndCapTimedDecay {
                constexpr std::ptrdiff_t m_flDecayTime = 0x1D8; // float32
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ContinuousEmitter {
                constexpr std::ptrdiff_t m_flEmissionDuration = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flStartTime = 0x350; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flEmitRate = 0x4C0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flEmissionScale = 0x630; // float32
                constexpr std::ptrdiff_t m_flScalePerParentParticle = 0x634; // float32
                constexpr std::ptrdiff_t m_bInitFromKilledParentParticles = 0x638; // bool
                constexpr std::ptrdiff_t m_nEventType = 0x63C; // EventTypeSelection_t
                constexpr std::ptrdiff_t m_nSnapshotControlPoint = 0x640; // int32
                constexpr std::ptrdiff_t m_strSnapshotSubset = 0x648; // CUtlString
                constexpr std::ptrdiff_t m_nLimitPerUpdate = 0x650; // int32
                constexpr std::ptrdiff_t m_bForceEmitOnFirstUpdate = 0x654; // bool
                constexpr std::ptrdiff_t m_bForceEmitOnLastUpdate = 0x655; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_OscillateVectorSimple {
                constexpr std::ptrdiff_t m_Rate = 0x1D8; // Vector
                constexpr std::ptrdiff_t m_Frequency = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_nField = 0x1F0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOscMult = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flOscAdd = 0x1F8; // float32
                constexpr std::ptrdiff_t m_bOffset = 0x1FC; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_INIT_SequenceLifeTime {
                constexpr std::ptrdiff_t m_flFramerate = 0x1E0; // float32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_MoveBetweenPoints {
                constexpr std::ptrdiff_t m_flSpeedMin = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flSpeedMax = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flEndSpread = 0x4C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flStartOffset = 0x630; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flEndOffset = 0x7A0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nEndControlPointNumber = 0x910; // int32
                constexpr std::ptrdiff_t m_bTrailBias = 0x914; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetUserEvent {
                constexpr std::ptrdiff_t m_flInput = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flRisingEdge = 0x348; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nRisingEventType = 0x4B8; // EventTypeSelection_t
                constexpr std::ptrdiff_t m_flFallingEdge = 0x4C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nFallingEventType = 0x630; // EventTypeSelection_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // p
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_QuantizeFloat {
                constexpr std::ptrdiff_t m_InputValue = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x348; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            namespace C_OP_BasicMovement {
                constexpr std::ptrdiff_t m_Gravity = 0x1D8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_fDrag = 0x890; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_massControls = 0xA00; // CParticleMassCalculationParameters
                constexpr std::ptrdiff_t m_nMaxConstraintPasses = 0xE58; // int32
                constexpr std::ptrdiff_t m_bUseNewCode = 0xE5C; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomNamedModelElement {
                constexpr std::ptrdiff_t m_hModel = 0x1E0; // CStrongHandle<InfoForResourceTypeCModel>
                constexpr std::ptrdiff_t m_names = 0x1E8; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_bShuffle = 0x200; // bool
                constexpr std::ptrdiff_t m_bLinear = 0x201; // bool
                constexpr std::ptrdiff_t m_bModelFromRenderer = 0x202; // bool
                constexpr std::ptrdiff_t m_nFieldOutput = 0x204; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_INIT_InitFromParentKilled {
                constexpr std::ptrdiff_t m_nAttributeToCopy = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nEventType = 0x1E4; // EventTypeSelection_t
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_Callback {
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_GlobalLight {
                constexpr std::ptrdiff_t m_flScale = 0x1D8; // float32
                constexpr std::ptrdiff_t m_bClampLowerRange = 0x1DC; // bool
                constexpr std::ptrdiff_t m_bClampUpperRange = 0x1DD; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_OffsetVectorToVector {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecOutputMin = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_vecOutputMax = 0x1F4; // Vector
                constexpr std::ptrdiff_t m_randomnessParameters = 0x200; // CRandomNumberGeneratorParameters
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_SetPerChildControlPointFromAttribute {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFirstControlPoint = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nNumControlPoints = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nParticleIncrement = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nFirstSourcePoint = 0x1E8; // int32
                constexpr std::ptrdiff_t m_bNumBasedOnParticleCount = 0x1EC; // bool
                constexpr std::ptrdiff_t m_nAttributeToRead = 0x1F0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nCPField = 0x1F4; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetParentControlPointsToChildCP {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nChildControlPoint = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nNumControlPoints = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nFirstSourcePoint = 0x1EC; // int32
                constexpr std::ptrdiff_t m_bSetOrientation = 0x1F0; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_BoxConstraint {
                constexpr std::ptrdiff_t m_vecMin = 0x1D8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_vecMax = 0x890; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nCP = 0xF48; // int32
                constexpr std::ptrdiff_t m_bLocalSpace = 0xF4C; // bool
                constexpr std::ptrdiff_t m_bAccountForRadius = 0xF4D; // bool
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_INIT_CreatePhyllotaxis {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nScaleCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nComponent = 0x1E8; // int32
                constexpr std::ptrdiff_t m_fRadCentCore = 0x1EC; // float32
                constexpr std::ptrdiff_t m_fRadPerPoint = 0x1F0; // float32
                constexpr std::ptrdiff_t m_fRadPerPointTo = 0x1F4; // float32
                constexpr std::ptrdiff_t m_fpointAngle = 0x1F8; // float32
                constexpr std::ptrdiff_t m_fsizeOverall = 0x1FC; // float32
                constexpr std::ptrdiff_t m_fRadBias = 0x200; // float32
                constexpr std::ptrdiff_t m_fMinRad = 0x204; // float32
                constexpr std::ptrdiff_t m_fDistBias = 0x208; // float32
                constexpr std::ptrdiff_t m_bUseLocalCoords = 0x20C; // bool
                constexpr std::ptrdiff_t m_bUseWithContEmit = 0x20D; // bool
                constexpr std::ptrdiff_t m_bUseOrigRadius = 0x20E; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_AttractToControlPoint {
                constexpr std::ptrdiff_t m_vecComponentScale = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_fForceAmount = 0x1F8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fMinimumDistance = 0x368; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fFalloffPower = 0x4D8; // float32
                constexpr std::ptrdiff_t m_TransformInput = 0x4E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_fForceAmountMin = 0x548; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bApplyMinForce = 0x6B8; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomLifeTime {
                constexpr std::ptrdiff_t m_fLifetimeMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_fLifetimeMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_fLifetimeRandExponent = 0x1E8; // float32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            namespace C_INIT_RemapParticleCountToNamedModelSequenceScalar {
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_VelocityRadialRandom {
                constexpr std::ptrdiff_t m_bPerParticleCenter = 0x1E0; // bool
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E4; // int32
                constexpr std::ptrdiff_t m_vecPosition = 0x1E8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecFwd = 0x8A0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_fSpeedMin = 0xF58; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fSpeedMax = 0x10C8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_vecLocalCoordinateSystemSpeedScale = 0x1238; // Vector
                constexpr std::ptrdiff_t m_bIgnoreDelta = 0x1245; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomRadius {
                constexpr std::ptrdiff_t m_flRadiusMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flRadiusMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flRadiusRandExponent = 0x1E8; // float32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_Orient2DRelToCP {
                constexpr std::ptrdiff_t m_flRotOffset = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flSpinStrength = 0x1DC; // float32
                constexpr std::ptrdiff_t m_nCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // PARTICLE_TOPOLOGY_LINES
            namespace ControlPointReference_t {
                constexpr std::ptrdiff_t m_controlPointNameString = 0x0; // int32
                constexpr std::ptrdiff_t m_vOffsetFromControlPoint = 0x4; // Vector
                constexpr std::ptrdiff_t m_bOffsetInLocalSpace = 0x10; // bool
            }
            // Parent: None
            // Field count: 20
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_LightningSnapshotGenerator {
                constexpr std::ptrdiff_t m_nCPSnapshot = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nCPStartPnt = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nCPEndPnt = 0x1E8; // int32
                constexpr std::ptrdiff_t m_flSegments = 0x1F0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOffset = 0x360; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOffsetDecay = 0x4D0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flRecalcRate = 0x640; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flUVScale = 0x7B0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flUVOffset = 0x920; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flSplitRate = 0xA90; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flRecursionSplitScale = 0xC00; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bScaleBranchDistance = 0xD70; // bool
                constexpr std::ptrdiff_t m_flBranchDistanceScale = 0xD78; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bScaleBranchOffset = 0xEE8; // bool
                constexpr std::ptrdiff_t m_flBranchOffsetScale = 0xEF0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flBranchTwist = 0x1060; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nBranchBehavior = 0x11D0; // ParticleLightnintBranchBehavior_t
                constexpr std::ptrdiff_t m_flRadiusStart = 0x11D8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flRadiusEnd = 0x1348; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flDedicatedPool = 0x14B8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_RemapNamedModelMeshGroupOnceTimed {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapQAnglesToRotation {
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_PositionWarp {
                constexpr std::ptrdiff_t m_vecWarpMin = 0x1E0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_vecWarpMax = 0x898; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nScaleControlPointNumber = 0xF50; // int32
                constexpr std::ptrdiff_t m_nControlPointNumber = 0xF54; // int32
                constexpr std::ptrdiff_t m_nRadiusComponent = 0xF58; // int32
                constexpr std::ptrdiff_t m_flWarpTime = 0xF5C; // float32
                constexpr std::ptrdiff_t m_flWarpStartTime = 0xF60; // float32
                constexpr std::ptrdiff_t m_flPrevPosScale = 0xF64; // float32
                constexpr std::ptrdiff_t m_bInvertWarp = 0xF68; // bool
                constexpr std::ptrdiff_t m_bUseCount = 0xF69; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_SetControlPointFieldToScalarExpression {
                constexpr std::ptrdiff_t m_nExpression = 0x1E0; // ScalarExpressionType_t
                constexpr std::ptrdiff_t m_flInput1 = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInput2 = 0x358; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOutputRemap = 0x4C8; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nOutputCP = 0x638; // int32
                constexpr std::ptrdiff_t m_nOutVectorField = 0x63C; // int32
                constexpr std::ptrdiff_t m_flInterpolation = 0x640; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySortPriority
            namespace C_OP_CreateParticleSystemRenderer {
                constexpr std::ptrdiff_t m_hEffect = 0x228; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                constexpr std::ptrdiff_t m_nEventType = 0x230; // EventTypeSelection_t
                constexpr std::ptrdiff_t m_vecCPs = 0x238; // CUtlLeanVector<CPAssignment_t>
                constexpr std::ptrdiff_t m_szParticleConfig = 0x248; // CUtlString
                constexpr std::ptrdiff_t m_AggregationPos = 0x250; // CPerParticleVecInput
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomVectorComponent {
                constexpr std::ptrdiff_t m_flMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nComponent = 0x1EC; // int32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // PARTICLE_CP_AXIS_Y
            // PARTICLE_CP_AXIS_Z
            // PARTICLE_CP_AXIS_NEGATIVE_X
            // PARTICLE_CP_AXIS_NEGATIVE_Y
            // PARTICLE_CP_AXIS_NEGATIVE_Z
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_SetVectorAttributeToVectorExpression {
                constexpr std::ptrdiff_t m_nExpression = 0x1E0; // VectorExpressionType_t
                constexpr std::ptrdiff_t m_vInput1 = 0x1E8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vInput2 = 0x8A0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flLerp = 0xF58; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x10C8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x10CC; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bNormalizedOutput = 0x10D0; // bool
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            namespace C_OP_RemapTransformVisibilityToVector {
                constexpr std::ptrdiff_t m_nSetMethod = 0x1D8; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x248; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x24C; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x250; // float32
                constexpr std::ptrdiff_t m_vecOutputMin = 0x254; // Vector
                constexpr std::ptrdiff_t m_vecOutputMax = 0x260; // Vector
                constexpr std::ptrdiff_t m_flRadius = 0x26C; // float32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_MovementLoopInsideSphere {
                constexpr std::ptrdiff_t m_nCP = 0x1D8; // int32
                constexpr std::ptrdiff_t m_flDistance = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_vecScale = 0x350; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nDistSqrAttr = 0xA08; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // F_DRAW_AS_TRAIL
            // F_MOTION_VECTORS
            // F_REFRACT
            // F_HSV_SHIFT
            // F_SPHERICAL_HARMONIC_LIGHTING
            // F_DEPTH_MAP
            // F_DISABLE_DEPTHTEST
            // F_OPAQUE
            // F_MOD2X
            // F_ADDSELF
            // F_TINT_BY_FOW
            // F_GLOBAL_LIGHT
            // F_BLENDFRAMES_SEQ0
            // F_ANIMATE_IN_FPS
            namespace C_OP_RenderSimpleModelCollection {
                constexpr std::ptrdiff_t m_bCenterOffset = 0x228; // bool
                constexpr std::ptrdiff_t m_hModel = 0x230; // CStrongHandle<InfoForResourceTypeCModel>
                constexpr std::ptrdiff_t m_modelInput = 0x238; // CParticleModelInput
                constexpr std::ptrdiff_t m_fSizeCullScale = 0x298; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bDisableShadows = 0x408; // bool
                constexpr std::ptrdiff_t m_bDisableMotionBlur = 0x409; // bool
                constexpr std::ptrdiff_t m_bAcceptsDecals = 0x40A; // bool
                constexpr std::ptrdiff_t m_fDrawFilter = 0x410; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nAngularVelocityField = 0x580; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_InitFloatCollection {
                constexpr std::ptrdiff_t m_InputValue = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x350; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // PARTICLE_TOPOLOGY_LINES
            // PARTICLE_TOPOLOGY_TRIS
            // PARTICLE_TOPOLOGY_QUADS
            // PARTICLE_TOPOLOGY_CUBES
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace CPathParameters {
                constexpr std::ptrdiff_t m_nStartControlPointNumber = 0x0; // int32
                constexpr std::ptrdiff_t m_nEndControlPointNumber = 0x4; // int32
                constexpr std::ptrdiff_t m_nBulgeControl = 0x8; // int32
                constexpr std::ptrdiff_t m_flBulge = 0xC; // float32
                constexpr std::ptrdiff_t m_flMidPoint = 0x10; // float32
                constexpr std::ptrdiff_t m_vStartPointOffset = 0x14; // Vector
                constexpr std::ptrdiff_t m_vMidPointOffset = 0x20; // Vector
                constexpr std::ptrdiff_t m_vEndOffset = 0x2C; // Vector
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapScalarEndCap {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1EC; // float32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_CreateFromPlaneCache {
                constexpr std::ptrdiff_t m_vecOffsetMin = 0x1E0; // Vector
                constexpr std::ptrdiff_t m_vecOffsetMax = 0x1EC; // Vector
                constexpr std::ptrdiff_t m_bUseNormal = 0x1F9; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LazyCullCompareFloat {
                constexpr std::ptrdiff_t m_flComparsion1 = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flComparsion2 = 0x348; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flCullTime = 0x4B8; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_ControlPointToRadialScreenSpace {
                constexpr std::ptrdiff_t m_nCPIn = 0x1E0; // int32
                constexpr std::ptrdiff_t m_vecCP1Pos = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_nCPOut = 0x1F0; // int32
                constexpr std::ptrdiff_t m_nCPOutField = 0x1F4; // int32
                constexpr std::ptrdiff_t m_nCPSSPosOut = 0x1F8; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_NormalOffset {
                constexpr std::ptrdiff_t m_OffsetMin = 0x1E0; // Vector
                constexpr std::ptrdiff_t m_OffsetMax = 0x1EC; // Vector
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1F8; // int32
                constexpr std::ptrdiff_t m_bLocalCoords = 0x1FC; // bool
                constexpr std::ptrdiff_t m_bNormalize = 0x1FD; // bool
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            namespace C_INIT_CreationNoise {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bAbsVal = 0x1E4; // bool
                constexpr std::ptrdiff_t m_bAbsValInv = 0x1E5; // bool
                constexpr std::ptrdiff_t m_flOffset = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flNoiseScale = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flNoiseScaleLoc = 0x1F8; // float32
                constexpr std::ptrdiff_t m_vecOffsetLoc = 0x1FC; // Vector
                constexpr std::ptrdiff_t m_flWorldTimeScale = 0x208; // float32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_Spin {
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_GameLiquidSpill {
                constexpr std::ptrdiff_t m_flLiquidContentsField = 0x228; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flExpirationTime = 0x398; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flRadius = 0x508; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bCheckExposedToSky = 0x678; // bool
                constexpr std::ptrdiff_t m_nAmountAttribute = 0x67C; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_InstantaneousEmitter {
                constexpr std::ptrdiff_t m_nParticlesToEmit = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flStartTime = 0x350; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInitFromKilledParentParticles = 0x4C0; // float32
                constexpr std::ptrdiff_t m_nEventType = 0x4C4; // EventTypeSelection_t
                constexpr std::ptrdiff_t m_flParentParticleScale = 0x4C8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nMaxEmittedPerFrame = 0x638; // int32
                constexpr std::ptrdiff_t m_nSnapshotControlPoint = 0x63C; // int32
                constexpr std::ptrdiff_t m_strSnapshotSubset = 0x640; // CUtlString
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_INIT_LifespanFromVelocity {
                constexpr std::ptrdiff_t m_vecComponentScale = 0x1E0; // Vector
                constexpr std::ptrdiff_t m_flTraceOffset = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flMaxTraceLength = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flTraceTolerance = 0x1F4; // float32
                constexpr std::ptrdiff_t m_nMaxPlanes = 0x1F8; // int32
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x200; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x280; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_bIncludeWater = 0x290; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_VelocityFromCP {
                constexpr std::ptrdiff_t m_velocityInput = 0x1E0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_transformInput = 0x898; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flVelocityScale = 0x900; // float32
                constexpr std::ptrdiff_t m_bDirectionOnly = 0x904; // bool
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            namespace C_OP_MovementSkinnedPositionFromCPSnapshot {
                constexpr std::ptrdiff_t m_nSnapshotControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1DC; // int32
                constexpr std::ptrdiff_t m_bRandom = 0x1E0; // bool
                constexpr std::ptrdiff_t m_nRandomSeed = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bSetNormal = 0x1E8; // bool
                constexpr std::ptrdiff_t m_bSetRadius = 0x1E9; // bool
                constexpr std::ptrdiff_t m_nIndexType = 0x1EC; // SnapshotIndexType_t
                constexpr std::ptrdiff_t m_flReadIndex = 0x1F0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flIncrement = 0x360; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nFullLoopIncrement = 0x4D0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nSnapShotStartPoint = 0x640; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInterpolation = 0x7B0; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_OscillateVector {
                constexpr std::ptrdiff_t m_RateMin = 0x1D8; // Vector
                constexpr std::ptrdiff_t m_RateMax = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_FrequencyMin = 0x1F0; // Vector
                constexpr std::ptrdiff_t m_FrequencyMax = 0x1FC; // Vector
                constexpr std::ptrdiff_t m_nField = 0x208; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bProportional = 0x20C; // bool
                constexpr std::ptrdiff_t m_bProportionalOp = 0x20D; // bool
                constexpr std::ptrdiff_t m_bOffset = 0x20E; // bool
                constexpr std::ptrdiff_t m_flStartTime_min = 0x210; // float32
                constexpr std::ptrdiff_t m_flStartTime_max = 0x214; // float32
                constexpr std::ptrdiff_t m_flEndTime_min = 0x218; // float32
                constexpr std::ptrdiff_t m_flEndTime_max = 0x21C; // float32
                constexpr std::ptrdiff_t m_flOscMult = 0x220; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOscAdd = 0x390; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flRateScale = 0x500; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_PositionLock {
                constexpr std::ptrdiff_t m_TransformInput = 0x1D8; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flStartTime_min = 0x240; // float32
                constexpr std::ptrdiff_t m_flStartTime_max = 0x244; // float32
                constexpr std::ptrdiff_t m_flStartTime_exp = 0x248; // float32
                constexpr std::ptrdiff_t m_flEndTime_min = 0x24C; // float32
                constexpr std::ptrdiff_t m_flEndTime_max = 0x250; // float32
                constexpr std::ptrdiff_t m_flEndTime_exp = 0x254; // float32
                constexpr std::ptrdiff_t m_flRange = 0x258; // float32
                constexpr std::ptrdiff_t m_flRangeBias = 0x260; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flJumpThreshold = 0x3D0; // float32
                constexpr std::ptrdiff_t m_flPrevPosScale = 0x3D4; // float32
                constexpr std::ptrdiff_t m_bLockRot = 0x3D8; // bool
                constexpr std::ptrdiff_t m_vecScale = 0x3E0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0xA98; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutputPrev = 0xA9C; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            namespace C_OP_RenderVRHapticEvent {
                constexpr std::ptrdiff_t m_nHand = 0x228; // ParticleVRHandChoiceList_t
                constexpr std::ptrdiff_t m_nOutputHandCP = 0x22C; // int32
                constexpr std::ptrdiff_t m_nOutputField = 0x230; // int32
                constexpr std::ptrdiff_t m_flAmplitude = 0x238; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointToImpactPoint {
                constexpr std::ptrdiff_t m_nCPOut = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nCPIn = 0x1E4; // int32
                constexpr std::ptrdiff_t m_flUpdateRate = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flTraceLength = 0x1F0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flStartOffset = 0x360; // float32
                constexpr std::ptrdiff_t m_flOffset = 0x364; // float32
                constexpr std::ptrdiff_t m_vecTraceDir = 0x368; // Vector
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x374; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x3F4; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_bSetToEndpoint = 0x3F8; // bool
                constexpr std::ptrdiff_t m_bTraceToClosestSurface = 0x3F9; // bool
                constexpr std::ptrdiff_t m_bIncludeWater = 0x3FA; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ReinitializeScalarEndCap {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOutputMin = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1E0; // float32
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_TurbulenceForce {
                constexpr std::ptrdiff_t m_flNoiseCoordScale0 = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flNoiseCoordScale1 = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flNoiseCoordScale2 = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flNoiseCoordScale3 = 0x1F4; // float32
                constexpr std::ptrdiff_t m_vecNoiseAmount0 = 0x1F8; // Vector
                constexpr std::ptrdiff_t m_vecNoiseAmount1 = 0x204; // Vector
                constexpr std::ptrdiff_t m_vecNoiseAmount2 = 0x210; // Vector
                constexpr std::ptrdiff_t m_vecNoiseAmount3 = 0x21C; // Vector
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_RemapNamedModelElementOnceTimed {
                constexpr std::ptrdiff_t m_hModel = 0x1D8; // CStrongHandle<InfoForResourceTypeCModel>
                constexpr std::ptrdiff_t m_inNames = 0x1E0; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_outNames = 0x1F8; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_fallbackNames = 0x210; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_bModelFromRenderer = 0x228; // bool
                constexpr std::ptrdiff_t m_bProportional = 0x229; // bool
                constexpr std::ptrdiff_t m_nFieldInput = 0x22C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x230; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flRemapTime = 0x234; // float32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_SetControlPointToPlayer {
                constexpr std::ptrdiff_t m_nCP1 = 0x1E0; // int32
                constexpr std::ptrdiff_t m_vecCP1Pos = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_bOrientToEyes = 0x1F0; // bool
                constexpr std::ptrdiff_t m_nPosition = 0x1F4; // ParticleEntityPos_t
                constexpr std::ptrdiff_t m_nRadiusCP = 0x1F8; // int32
                constexpr std::ptrdiff_t m_nRadiusCPField = 0x1FC; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_OP_EndCapTimedFreeze {
                constexpr std::ptrdiff_t m_flFreezeTime = 0x1D8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_SetRandomControlPointPosition {
                constexpr std::ptrdiff_t m_bUseWorldLocation = 0x1E0; // bool
                constexpr std::ptrdiff_t m_bOrient = 0x1E1; // bool
                constexpr std::ptrdiff_t m_nCP1 = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nHeadLocation = 0x1E8; // int32
                constexpr std::ptrdiff_t m_flReRandomRate = 0x1F0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_vecCPMinPos = 0x360; // Vector
                constexpr std::ptrdiff_t m_vecCPMaxPos = 0x36C; // Vector
                constexpr std::ptrdiff_t m_flInterpolation = 0x378; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyDescription
            // MPropertyAttributeEditor
            // MPropertyEditContextOverrideKey
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_RenderVolumetricEmitter {
                constexpr std::ptrdiff_t m_strChannelType = 0x228; // CUtlString
                constexpr std::ptrdiff_t m_nType = 0x230; // ParticleVolumetricSmokeType_t
                constexpr std::ptrdiff_t m_nCreationType = 0x234; // ParticleVolumetricSmokeCreationType_t
                constexpr std::ptrdiff_t m_nEventType = 0x238; // EventTypeSelection_t
                constexpr std::ptrdiff_t m_vecPos = 0x240; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecVelocity = 0x8F8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vPrevPosition = 0xFB0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flSpeed = 0x1668; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flRadius = 0x17D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flDensity = 0x1948; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flTemperature = 0x1AB8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flMagnitude = 0x1C28; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flKillRadius = 0x1D98; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flKillDensityScale = 0x1F08; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flFalloff = 0x2078; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_RemapTransformVisibilityToScalar {
                constexpr std::ptrdiff_t m_nSetMethod = 0x1D8; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x248; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x24C; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x250; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x254; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x258; // float32
                constexpr std::ptrdiff_t m_flRadius = 0x25C; // float32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_RemapControlPointDirectionToVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0x1DC; // float32
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_OP_ScreenSpacePositionOfTarget {
                constexpr std::ptrdiff_t m_vecTargetPosition = 0x1D8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_bOututBehindness = 0x890; // bool
                constexpr std::ptrdiff_t m_nBehindFieldOutput = 0x894; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flBehindOutputRemap = 0x898; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nBehindSetMethod = 0xA08; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_DragRelativeToPlane {
                constexpr std::ptrdiff_t m_flDragAtPlane = 0x1D8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flFalloff = 0x348; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bDirectional = 0x4B8; // bool
                constexpr std::ptrdiff_t m_vecPlaneNormal = 0x4C0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nControlPointNumber = 0xB78; // int32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // PARTICLE_MASSMODE_RADIUS_SQUARED
            // MPropertyFriendlyName
            namespace C_OP_SetCPtoVector {
                constexpr std::ptrdiff_t m_nCPInput = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomYaw {
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SnapshotRigidSkinToBones {
                constexpr std::ptrdiff_t m_bTransformNormals = 0x1D8; // bool
                constexpr std::ptrdiff_t m_bTransformRadii = 0x1D9; // bool
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1DC; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_SetSingleControlPointPosition {
                constexpr std::ptrdiff_t m_bSetOnce = 0x1E0; // bool
                constexpr std::ptrdiff_t m_nCP1 = 0x1E4; // int32
                constexpr std::ptrdiff_t m_vecCP1Pos = 0x1E8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_transformInput = 0x8A0; // CParticleTransformInput
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_DistanceToNeighborCull {
                constexpr std::ptrdiff_t m_flDistance = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bIncludeRadii = 0x350; // bool
                constexpr std::ptrdiff_t m_flLifespanOverlap = 0x358; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nFieldModify = 0x4C8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flModify = 0x4D0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x640; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bUseNeighbor = 0x644; // bool
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_RemapCPtoScalar {
                constexpr std::ptrdiff_t m_nCPInput = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nField = 0x1E0; // int32
                constexpr std::ptrdiff_t m_flInputMin = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flStartTime = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x1F8; // float32
                constexpr std::ptrdiff_t m_flInterpRate = 0x1FC; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x200; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 66
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace CParticleSystemDefinition {
                constexpr std::ptrdiff_t m_nBehaviorVersion = 0x8; // int32
                constexpr std::ptrdiff_t m_PreEmissionOperators = 0x10; // CUtlVector<CParticleFunctionPreEmission*>
                constexpr std::ptrdiff_t m_Emitters = 0x28; // CUtlVector<CParticleFunctionEmitter*>
                constexpr std::ptrdiff_t m_Initializers = 0x40; // CUtlVector<CParticleFunctionInitializer*>
                constexpr std::ptrdiff_t m_Operators = 0x58; // CUtlVector<CParticleFunctionOperator*>
                constexpr std::ptrdiff_t m_ForceGenerators = 0x70; // CUtlVector<CParticleFunctionForce*>
                constexpr std::ptrdiff_t m_Constraints = 0x88; // CUtlVector<CParticleFunctionConstraint*>
                constexpr std::ptrdiff_t m_Renderers = 0xA0; // CUtlVector<CParticleFunctionRenderer*>
                constexpr std::ptrdiff_t m_Children = 0xB8; // CUtlVector<ParticleChildrenInfo_t>
                constexpr std::ptrdiff_t m_nFirstMultipleOverride_BackwardCompat = 0x178; // int32
                constexpr std::ptrdiff_t m_nInitialParticles = 0x258; // int32
                constexpr std::ptrdiff_t m_nMaxParticles = 0x25C; // int32
                constexpr std::ptrdiff_t m_nGroupID = 0x260; // int32
                constexpr std::ptrdiff_t m_BoundingBoxMin = 0x264; // Vector
                constexpr std::ptrdiff_t m_BoundingBoxMax = 0x270; // Vector
                constexpr std::ptrdiff_t m_flDepthSortBias = 0x27C; // float32
                constexpr std::ptrdiff_t m_nSortOverridePositionCP = 0x280; // int32
                constexpr std::ptrdiff_t m_bInfiniteBounds = 0x284; // bool
                constexpr std::ptrdiff_t m_bEnableNamedValues = 0x285; // bool
                constexpr std::ptrdiff_t m_NamedValueDomain = 0x288; // CUtlString
                constexpr std::ptrdiff_t m_NamedValueLocals = 0x290; // CUtlVector<ParticleNamedValueSource_t*>
                constexpr std::ptrdiff_t m_ConstantColor = 0x2A8; // Color
                constexpr std::ptrdiff_t m_ConstantNormal = 0x2AC; // Vector
                constexpr std::ptrdiff_t m_flConstantRadius = 0x2B8; // float32
                constexpr std::ptrdiff_t m_flConstantRotation = 0x2BC; // float32
                constexpr std::ptrdiff_t m_flConstantRotationSpeed = 0x2C0; // float32
                constexpr std::ptrdiff_t m_flConstantLifespan = 0x2C4; // float32
                constexpr std::ptrdiff_t m_nConstantSequenceNumber = 0x2C8; // int32
                constexpr std::ptrdiff_t m_nConstantSequenceNumber1 = 0x2CC; // int32
                constexpr std::ptrdiff_t m_nSnapshotControlPoint = 0x2D0; // int32
                constexpr std::ptrdiff_t m_hSnapshot = 0x2D8; // CStrongHandle<InfoForResourceTypeIParticleSnapshot>
                constexpr std::ptrdiff_t m_pszCullReplacementName = 0x2E0; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                constexpr std::ptrdiff_t m_flCullRadius = 0x2E8; // float32
                constexpr std::ptrdiff_t m_flCullFillCost = 0x2EC; // float32
                constexpr std::ptrdiff_t m_nCullControlPoint = 0x2F0; // int32
                constexpr std::ptrdiff_t m_hFallback = 0x2F8; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                constexpr std::ptrdiff_t m_nFallbackMaxCount = 0x300; // int32
                constexpr std::ptrdiff_t m_hLowViolenceDef = 0x308; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                constexpr std::ptrdiff_t m_hReferenceReplacement = 0x310; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                constexpr std::ptrdiff_t m_flPreSimulationTime = 0x318; // float32
                constexpr std::ptrdiff_t m_flStopSimulationAfterTime = 0x31C; // float32
                constexpr std::ptrdiff_t m_flMaximumTimeStep = 0x320; // float32
                constexpr std::ptrdiff_t m_flMaximumSimTime = 0x324; // float32
                constexpr std::ptrdiff_t m_flMinimumSimTime = 0x328; // float32
                constexpr std::ptrdiff_t m_flMinimumTimeStep = 0x32C; // float32
                constexpr std::ptrdiff_t m_nMinimumFrames = 0x330; // int32
                constexpr std::ptrdiff_t m_bIsGPUParticleSystem = 0x334; // bool
                constexpr std::ptrdiff_t m_nMinCPULevel = 0x338; // int32
                constexpr std::ptrdiff_t m_nMinGPULevel = 0x33C; // int32
                constexpr std::ptrdiff_t m_flNoDrawTimeToGoToSleep = 0x340; // float32
                constexpr std::ptrdiff_t m_flMaxDrawDistance = 0x344; // float32
                constexpr std::ptrdiff_t m_flStartFadeDistance = 0x348; // float32
                constexpr std::ptrdiff_t m_flMaxCreationDistance = 0x34C; // float32
                constexpr std::ptrdiff_t m_nAggregationMinAvailableParticles = 0x350; // int32
                constexpr std::ptrdiff_t m_flAggregateRadius = 0x354; // float32
                constexpr std::ptrdiff_t m_bShouldBatch = 0x358; // bool
                constexpr std::ptrdiff_t m_bShouldHitboxesFallbackToRenderBounds = 0x359; // bool
                constexpr std::ptrdiff_t m_bShouldHitboxesFallbackToSnapshot = 0x35A; // bool
                constexpr std::ptrdiff_t m_bShouldHitboxesFallbackToCollisionHulls = 0x35B; // bool
                constexpr std::ptrdiff_t m_nViewModelEffect = 0x35C; // InheritableBoolType_t
                constexpr std::ptrdiff_t m_bScreenSpaceEffect = 0x360; // bool
                constexpr std::ptrdiff_t m_pszTargetLayerID = 0x368; // CUtlSymbolLarge
                constexpr std::ptrdiff_t m_nSkipRenderControlPoint = 0x370; // int32
                constexpr std::ptrdiff_t m_nAllowRenderControlPoint = 0x374; // int32
                constexpr std::ptrdiff_t m_bShouldSort = 0x378; // bool
                constexpr std::ptrdiff_t m_controlPointConfigurations = 0x3C0; // CUtlVector<ParticleControlPointConfiguration_t>
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_RemapNamedModelMeshGroupEndCap {
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_PercentageBetweenTransformsVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E0; // float32
                constexpr std::ptrdiff_t m_vecOutputMin = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_vecOutputMax = 0x1F0; // Vector
                constexpr std::ptrdiff_t m_TransformStart = 0x200; // CParticleTransformInput
                constexpr std::ptrdiff_t m_TransformEnd = 0x268; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x2D0; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x2D4; // bool
                constexpr std::ptrdiff_t m_bRadialCheck = 0x2D5; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_CreateWithinBox {
                constexpr std::ptrdiff_t m_vecMin = 0x1E0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecMax = 0x898; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_nControlPointNumber = 0xF50; // int32
                constexpr std::ptrdiff_t m_bLocalSpace = 0xF54; // bool
                constexpr std::ptrdiff_t m_randomnessParameters = 0xF58; // CRandomNumberGeneratorParameters
                constexpr std::ptrdiff_t m_bUseNewCode = 0xF60; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ChooseRandomChildrenInGroup {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1E0; // int32
                constexpr std::ptrdiff_t m_flNumberOfChildren = 0x1E8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 33
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // PARTICLE_MASSMODE_RADIUS_SQUARED
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_OP_ControlpointLight {
                constexpr std::ptrdiff_t m_flScale = 0x1D8; // float32
                constexpr std::ptrdiff_t m_nControlPoint1 = 0x660; // int32
                constexpr std::ptrdiff_t m_nControlPoint2 = 0x664; // int32
                constexpr std::ptrdiff_t m_nControlPoint3 = 0x668; // int32
                constexpr std::ptrdiff_t m_nControlPoint4 = 0x66C; // int32
                constexpr std::ptrdiff_t m_vecCPOffset1 = 0x670; // Vector
                constexpr std::ptrdiff_t m_vecCPOffset2 = 0x67C; // Vector
                constexpr std::ptrdiff_t m_vecCPOffset3 = 0x688; // Vector
                constexpr std::ptrdiff_t m_vecCPOffset4 = 0x694; // Vector
                constexpr std::ptrdiff_t m_LightFiftyDist1 = 0x6A0; // float32
                constexpr std::ptrdiff_t m_LightZeroDist1 = 0x6A4; // float32
                constexpr std::ptrdiff_t m_LightFiftyDist2 = 0x6A8; // float32
                constexpr std::ptrdiff_t m_LightZeroDist2 = 0x6AC; // float32
                constexpr std::ptrdiff_t m_LightFiftyDist3 = 0x6B0; // float32
                constexpr std::ptrdiff_t m_LightZeroDist3 = 0x6B4; // float32
                constexpr std::ptrdiff_t m_LightFiftyDist4 = 0x6B8; // float32
                constexpr std::ptrdiff_t m_LightZeroDist4 = 0x6BC; // float32
                constexpr std::ptrdiff_t m_LightColor1 = 0x6C0; // Color
                constexpr std::ptrdiff_t m_LightColor2 = 0x6C4; // Color
                constexpr std::ptrdiff_t m_LightColor3 = 0x6C8; // Color
                constexpr std::ptrdiff_t m_LightColor4 = 0x6CC; // Color
                constexpr std::ptrdiff_t m_bLightType1 = 0x6D0; // bool
                constexpr std::ptrdiff_t m_bLightType2 = 0x6D1; // bool
                constexpr std::ptrdiff_t m_bLightType3 = 0x6D2; // bool
                constexpr std::ptrdiff_t m_bLightType4 = 0x6D3; // bool
                constexpr std::ptrdiff_t m_bLightDynamic1 = 0x6D4; // bool
                constexpr std::ptrdiff_t m_bLightDynamic2 = 0x6D5; // bool
                constexpr std::ptrdiff_t m_bLightDynamic3 = 0x6D6; // bool
                constexpr std::ptrdiff_t m_bLightDynamic4 = 0x6D7; // bool
                constexpr std::ptrdiff_t m_bUseNormal = 0x6D8; // bool
                constexpr std::ptrdiff_t m_bUseHLambert = 0x6D9; // bool
                constexpr std::ptrdiff_t m_bClampLowerRange = 0x6DE; // bool
                constexpr std::ptrdiff_t m_bClampUpperRange = 0x6DF; // bool
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_VectorFieldSnapshot {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nAttributeToWrite = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nLocalSpaceCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_flInterpolation = 0x1E8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_vecScale = 0x358; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flBoundaryDampening = 0xA10; // float32
                constexpr std::ptrdiff_t m_bSetVelocity = 0xA14; // bool
                constexpr std::ptrdiff_t m_bLockToSurface = 0xA15; // bool
                constexpr std::ptrdiff_t m_flGridSpacing = 0xA18; // float32
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_CylindricalDistanceToTransform {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x4C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x630; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_TransformStart = 0x7A0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_TransformEnd = 0x808; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x870; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x874; // bool
                constexpr std::ptrdiff_t m_bAdditive = 0x875; // bool
                constexpr std::ptrdiff_t m_bCapsule = 0x876; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomScalar {
                constexpr std::ptrdiff_t m_flMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flExponent = 0x1E8; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertySortPriority
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_RenderPostProcessing {
                constexpr std::ptrdiff_t m_flPostProcessStrength = 0x228; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_hPostTexture = 0x398; // CStrongHandle<InfoForResourceTypeCPostProcessingResource>
                constexpr std::ptrdiff_t m_nPriority = 0x3A0; // ParticlePostProcessPriorityGroup_t
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_OscillateScalar {
                constexpr std::ptrdiff_t m_RateMin = 0x1D8; // float32
                constexpr std::ptrdiff_t m_RateMax = 0x1DC; // float32
                constexpr std::ptrdiff_t m_FrequencyMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_FrequencyMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_nField = 0x1E8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bProportional = 0x1EC; // bool
                constexpr std::ptrdiff_t m_bProportionalOp = 0x1ED; // bool
                constexpr std::ptrdiff_t m_flStartTime_min = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flStartTime_max = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flEndTime_min = 0x1F8; // float32
                constexpr std::ptrdiff_t m_flEndTime_max = 0x1FC; // float32
                constexpr std::ptrdiff_t m_flOscMult = 0x200; // float32
                constexpr std::ptrdiff_t m_flOscAdd = 0x204; // float32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_FadeOut {
                constexpr std::ptrdiff_t m_flFadeOutTimeMin = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flFadeOutTimeMax = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flFadeOutTimeExp = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flFadeBias = 0x1E4; // float32
                constexpr std::ptrdiff_t m_bProportional = 0x220; // bool
                constexpr std::ptrdiff_t m_bEaseInAndOut = 0x221; // bool
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_WaterImpulseRenderer {
                constexpr std::ptrdiff_t m_vecPos = 0x228; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flRadius = 0x8E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flMagnitude = 0xA50; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flShape = 0xBC0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flWindSpeed = 0xD30; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flWobble = 0xEA0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bIsRadialWind = 0x1010; // bool
                constexpr std::ptrdiff_t m_nEventType = 0x1014; // EventTypeSelection_t
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomSequence {
                constexpr std::ptrdiff_t m_nSequenceMin = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nSequenceMax = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bShuffle = 0x1E8; // bool
                constexpr std::ptrdiff_t m_bLinear = 0x1E9; // bool
                constexpr std::ptrdiff_t m_WeightedList = 0x1F0; // CUtlVector<SequenceWeightedList_t>
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RampScalarSplineSimple {
                constexpr std::ptrdiff_t m_Rate = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flStartTime = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x1E0; // float32
                constexpr std::ptrdiff_t m_nField = 0x210; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bEaseOut = 0x214; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            namespace C_INIT_DistanceCull {
                constexpr std::ptrdiff_t m_nControlPoint = 0x1E0; // int32
                constexpr std::ptrdiff_t m_flDistance = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bCullInside = 0x358; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_INIT_InitFromVectorFieldSnapshot {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nLocalSpaceCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nWeightUpdateCP = 0x1E8; // int32
                constexpr std::ptrdiff_t m_bUseVerticalVelocity = 0x1EC; // bool
                constexpr std::ptrdiff_t m_vecScale = 0x1F0; // CPerParticleVecInput
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_OP_SetVectorAttributeToVectorExpression {
                constexpr std::ptrdiff_t m_nExpression = 0x1D8; // VectorExpressionType_t
                constexpr std::ptrdiff_t m_vInput1 = 0x1E0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vInput2 = 0x898; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flLerp = 0xF50; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x10C0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x10C4; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bNormalizedOutput = 0x10C8; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // PARTICLE_CP_AXIS_Y
            // PARTICLE_CP_AXIS_Z
            // PARTICLE_CP_AXIS_NEGATIVE_X
            // PARTICLE_CP_AXIS_NEGATIVE_Y
            // PARTICLE_CP_AXIS_NEGATIVE_Z
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_AddVectorToVector {
                constexpr std::ptrdiff_t m_vecScale = 0x1E0; // Vector
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldInput = 0x1F0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vOffsetMin = 0x1F4; // Vector
                constexpr std::ptrdiff_t m_vOffsetMax = 0x200; // Vector
                constexpr std::ptrdiff_t m_randomnessParameters = 0x20C; // CRandomNumberGeneratorParameters
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_RemapInitialVisibilityScalar {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F4; // float32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            namespace C_OP_RemapTransformOrientationToYaw {
                constexpr std::ptrdiff_t m_TransformInput = 0x1D8; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x240; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flRotOffset = 0x244; // float32
                constexpr std::ptrdiff_t m_flSpinStrength = 0x248; // float32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RenderStatusEffect {
                constexpr std::ptrdiff_t m_pTextureColorWarp = 0x228; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_pTextureDetail2 = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_pTextureDiffuseWarp = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_pTextureFresnelColorWarp = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_pTextureFresnelWarp = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_pTextureSpecularWarp = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_pTextureEnvMap = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RandomForce {
                constexpr std::ptrdiff_t m_MinForce = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_MaxForce = 0x1F4; // Vector
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapParticleCountOnScalarEndCap {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nInputMin = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nInputMax = 0x1E0; // int32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1E8; // float32
                constexpr std::ptrdiff_t m_bBackwards = 0x1EC; // bool
                constexpr std::ptrdiff_t m_nSetMethod = 0x1F0; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 18
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // INHERITABLE_BOOL_FALSE
            // INHERITABLE_BOOL_TRUE
            namespace ParticlePreviewState_t {
                constexpr std::ptrdiff_t m_previewModel = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_nModSpecificData = 0x8; // uint32
                constexpr std::ptrdiff_t m_groundType = 0xC; // PetGroundType_t
                constexpr std::ptrdiff_t m_sequenceName = 0x10; // CUtlString
                constexpr std::ptrdiff_t m_nFireParticleOnSequenceFrame = 0x18; // int32
                constexpr std::ptrdiff_t m_hitboxSetName = 0x20; // CUtlString
                constexpr std::ptrdiff_t m_materialGroupName = 0x28; // CUtlString
                constexpr std::ptrdiff_t m_vecBodyGroups = 0x30; // CUtlVector<ParticlePreviewBodyGroup_t>
                constexpr std::ptrdiff_t m_flPlaybackSpeed = 0x48; // float32
                constexpr std::ptrdiff_t m_flParticleSimulationRate = 0x4C; // float32
                constexpr std::ptrdiff_t m_bShouldDrawHitboxes = 0x50; // bool
                constexpr std::ptrdiff_t m_bShouldDrawAttachments = 0x51; // bool
                constexpr std::ptrdiff_t m_bShouldDrawAttachmentNames = 0x52; // bool
                constexpr std::ptrdiff_t m_bShouldDrawControlPointAxes = 0x53; // bool
                constexpr std::ptrdiff_t m_bAnimationNonLooping = 0x54; // bool
                constexpr std::ptrdiff_t m_bSequenceNameIsAnimClipPath = 0x55; // bool
                constexpr std::ptrdiff_t m_vecPreviewGravity = 0x58; // Vector
                constexpr std::ptrdiff_t m_vecPreviewWind = 0x64; // Vector
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_ModelCull {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_bBoundBox = 0x1DC; // bool
                constexpr std::ptrdiff_t m_bCullOutside = 0x1DD; // bool
                constexpr std::ptrdiff_t m_bUseBones = 0x1DE; // bool
                constexpr std::ptrdiff_t m_HitboxSetName = 0x1DF; // char[128]
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetFloat {
                constexpr std::ptrdiff_t m_InputValue = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x348; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x34C; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_Lerp = 0x350; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapTransformToVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vInputMin = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_vInputMax = 0x1F0; // Vector
                constexpr std::ptrdiff_t m_vOutputMin = 0x1FC; // Vector
                constexpr std::ptrdiff_t m_vOutputMax = 0x208; // Vector
                constexpr std::ptrdiff_t m_TransformInput = 0x218; // CParticleTransformInput
                constexpr std::ptrdiff_t m_LocalSpaceTransform = 0x280; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flStartTime = 0x2E8; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x2EC; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x2F0; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bOffset = 0x2F4; // bool
                constexpr std::ptrdiff_t m_bAccelerate = 0x2F5; // bool
                constexpr std::ptrdiff_t m_flRemapBias = 0x2F8; // float32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_ScreenSpaceDistanceToEdge {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flMaxDistFromEdge = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputRemap = 0x350; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x4C0; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapVectortoCP {
                constexpr std::ptrdiff_t m_nOutControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldInput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nParticleNumber = 0x1E0; // int32
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetFromCPSnapshot {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_strSnapshotSubset = 0x1E0; // CUtlString
                constexpr std::ptrdiff_t m_nAttributeToRead = 0x1E8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nAttributeToWrite = 0x1EC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nLocalSpaceCP = 0x1F0; // int32
                constexpr std::ptrdiff_t m_bRandom = 0x1F4; // bool
                constexpr std::ptrdiff_t m_bReverse = 0x1F5; // bool
                constexpr std::ptrdiff_t m_nRandomSeed = 0x1F8; // int32
                constexpr std::ptrdiff_t m_nSnapShotStartPoint = 0x200; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nSnapShotIncrement = 0x370; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInterpolation = 0x4E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bSubSample = 0x650; // bool
                constexpr std::ptrdiff_t m_bPrev = 0x651; // bool
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_DistanceBetweenCPsToCP {
                constexpr std::ptrdiff_t m_nStartCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nEndCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nOutputCP = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nOutputCPField = 0x1EC; // int32
                constexpr std::ptrdiff_t m_bSetOnce = 0x1F0; // bool
                constexpr std::ptrdiff_t m_flInputMin = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1F8; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1FC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x200; // float32
                constexpr std::ptrdiff_t m_flMaxTraceLength = 0x204; // float32
                constexpr std::ptrdiff_t m_flLOSScale = 0x208; // float32
                constexpr std::ptrdiff_t m_bLOS = 0x20C; // bool
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x20D; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x290; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_nSetParent = 0x294; // ParticleParentSetMode_t
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointToHand {
                constexpr std::ptrdiff_t m_nCP1 = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nHand = 0x1E4; // int32
                constexpr std::ptrdiff_t m_vecCP1Pos = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_bOrientToHand = 0x1F4; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_DistanceCull {
                constexpr std::ptrdiff_t m_nControlPoint = 0x1D8; // int32
                constexpr std::ptrdiff_t m_vecPointOffset = 0x1DC; // Vector
                constexpr std::ptrdiff_t m_flDistance = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bCullInside = 0x358; // bool
                constexpr std::ptrdiff_t m_nAttribute = 0x35C; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_CreateAlongPath {
                constexpr std::ptrdiff_t m_fMaxDistance = 0x1E0; // float32
                constexpr std::ptrdiff_t m_PathParams = 0x1F0; // CPathParameters
                constexpr std::ptrdiff_t m_bUseRandomCPs = 0x230; // bool
                constexpr std::ptrdiff_t m_vEndOffset = 0x234; // Vector
                constexpr std::ptrdiff_t m_bSaveOffset = 0x240; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_SetControlPointsToModelParticles {
                constexpr std::ptrdiff_t m_HitboxSetName = 0x1D8; // char[128]
                constexpr std::ptrdiff_t m_AttachmentName = 0x258; // char[128]
                constexpr std::ptrdiff_t m_nFirstControlPoint = 0x2D8; // int32
                constexpr std::ptrdiff_t m_nNumControlPoints = 0x2DC; // int32
                constexpr std::ptrdiff_t m_nFirstSourcePoint = 0x2E0; // int32
                constexpr std::ptrdiff_t m_bSkin = 0x2E4; // bool
                constexpr std::ptrdiff_t m_bAttachment = 0x2E5; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_ColorInterpolateRandom {
                constexpr std::ptrdiff_t m_ColorFadeMin = 0x1D8; // Color
                constexpr std::ptrdiff_t m_ColorFadeMax = 0x1F4; // Color
                constexpr std::ptrdiff_t m_flFadeStartTime = 0x204; // float32
                constexpr std::ptrdiff_t m_flFadeEndTime = 0x208; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x20C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bEaseInOut = 0x210; // bool
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapNamedModelSequenceToScalar {
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertySortPriority
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_RenderLights {
                constexpr std::ptrdiff_t m_flAnimationRate = 0x230; // float32
                constexpr std::ptrdiff_t m_nAnimationType = 0x234; // AnimationType_t
                constexpr std::ptrdiff_t m_bAnimateInFPS = 0x238; // bool
                constexpr std::ptrdiff_t m_flMinSize = 0x23C; // float32
                constexpr std::ptrdiff_t m_flMaxSize = 0x240; // float32
                constexpr std::ptrdiff_t m_flStartFadeSize = 0x244; // float32
                constexpr std::ptrdiff_t m_flEndFadeSize = 0x248; // float32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_DecayClampCount {
                constexpr std::ptrdiff_t m_nCount = 0x1D8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace CRandomNumberGeneratorParameters {
                constexpr std::ptrdiff_t m_bDistributeEvenly = 0x0; // bool
                constexpr std::ptrdiff_t m_nSeed = 0x4; // int32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_ColorLitPerParticle {
                constexpr std::ptrdiff_t m_ColorMin = 0x1F8; // Color
                constexpr std::ptrdiff_t m_ColorMax = 0x1FC; // Color
                constexpr std::ptrdiff_t m_TintMin = 0x200; // Color
                constexpr std::ptrdiff_t m_TintMax = 0x204; // Color
                constexpr std::ptrdiff_t m_flTintPerc = 0x208; // float32
                constexpr std::ptrdiff_t m_nTintBlendMode = 0x20C; // ParticleColorBlendMode_t
                constexpr std::ptrdiff_t m_flLightAmplification = 0x210; // float32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_SetAttributeToScalarExpression {
                constexpr std::ptrdiff_t m_nExpression = 0x1E0; // ScalarExpressionType_t
                constexpr std::ptrdiff_t m_flInput1 = 0x1E8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInput2 = 0x358; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputRemap = 0x4C8; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x638; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x63C; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_CreateOnGrid {
                constexpr std::ptrdiff_t m_nXCount = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nYCount = 0x350; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nZCount = 0x4C0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nXSpacing = 0x630; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nYSpacing = 0x7A0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nZSpacing = 0x910; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nControlPointNumber = 0xA80; // int32
                constexpr std::ptrdiff_t m_bLocalSpace = 0xA84; // bool
                constexpr std::ptrdiff_t m_bCenter = 0xA85; // bool
                constexpr std::ptrdiff_t m_bHollow = 0xA86; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_RampCPLinearRandom {
                constexpr std::ptrdiff_t m_nOutControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_vecRateMin = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_vecRateMax = 0x1F0; // Vector
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_RandomAlphaWindowThreshold {
                constexpr std::ptrdiff_t m_flMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flExponent = 0x1E8; // float32
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_INIT_CreateOnModelAtHeight {
                constexpr std::ptrdiff_t m_bUseBones = 0x1E0; // bool
                constexpr std::ptrdiff_t m_bForceZ = 0x1E1; // bool
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nHeightCP = 0x1E8; // int32
                constexpr std::ptrdiff_t m_bUseWaterHeight = 0x1EC; // bool
                constexpr std::ptrdiff_t m_flDesiredHeight = 0x1F0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_vecHitBoxScale = 0x360; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_vecDirectionBias = 0xA18; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nBiasType = 0x10D0; // ParticleHitboxBiasType_t
                constexpr std::ptrdiff_t m_bLocalCoords = 0x10D4; // bool
                constexpr std::ptrdiff_t m_bPreferMovingBoxes = 0x10D5; // bool
                constexpr std::ptrdiff_t m_HitboxSetName = 0x10D6; // char[128]
                constexpr std::ptrdiff_t m_flHitboxVelocityScale = 0x1158; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flMaxBoneVelocity = 0x12C8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_ModelSurfaceSnapshotGenerator {
                constexpr std::ptrdiff_t m_nCPSnapshot = 0x1E0; // int32
                constexpr std::ptrdiff_t m_modelInput = 0x1E8; // CParticleModelInput
                constexpr std::ptrdiff_t m_flRecalcRate = 0x248; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flUSpacing = 0x3B8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flVSpacing = 0x528; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flSurfaceOffset = 0x698; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bSetNormal = 0x808; // bool
                constexpr std::ptrdiff_t m_bSetUp = 0x809; // bool
                constexpr std::ptrdiff_t m_bSetGravity = 0x80A; // bool
                constexpr std::ptrdiff_t m_bSetUV = 0x80B; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_RestartAfterDuration {
                constexpr std::ptrdiff_t m_flDurationMin = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flDurationMax = 0x1DC; // float32
                constexpr std::ptrdiff_t m_nCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nCPField = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1E8; // int32
                constexpr std::ptrdiff_t m_bOnlyChildren = 0x1EC; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_RemapVisibilityScalar {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flRadiusScale = 0x1F0; // float32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_CreateSequentialPathV2 {
                constexpr std::ptrdiff_t m_fMaxDistance = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flNumToAssign = 0x350; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bLoop = 0x4C0; // bool
                constexpr std::ptrdiff_t m_bCPPairs = 0x4C1; // bool
                constexpr std::ptrdiff_t m_bSaveOffset = 0x4C2; // bool
                constexpr std::ptrdiff_t m_PathParams = 0x4D0; // CPathParameters
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            namespace VecInputMaterialVariable_t {
                constexpr std::ptrdiff_t m_strVariable = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_vecInput = 0x8; // CParticleCollectionVecInput
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_INIT_RemapInitialDirectionToTransformToVector {
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x248; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0x24C; // float32
                constexpr std::ptrdiff_t m_flOffsetRot = 0x250; // float32
                constexpr std::ptrdiff_t m_vecOffsetAxis = 0x254; // Vector
                constexpr std::ptrdiff_t m_bNormalize = 0x260; // bool
                constexpr std::ptrdiff_t m_vecTargetPosition = 0x1E0; // CPerParticleVecInput
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_LockToSavedSequentialPathV2 {
                constexpr std::ptrdiff_t m_flFadeStart = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flFadeEnd = 0x1DC; // float32
                constexpr std::ptrdiff_t m_bCPPairs = 0x1E0; // bool
                constexpr std::ptrdiff_t m_PathParams = 0x1F0; // CPathParameters
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_NormalLock {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapTransformOrientationToRotations {
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_vecRotation = 0x248; // Vector
                constexpr std::ptrdiff_t m_bUseQuat = 0x254; // bool
                constexpr std::ptrdiff_t m_bWriteNormal = 0x255; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_Cull {
                constexpr std::ptrdiff_t m_flCullPerc = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flCullStart = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flCullEnd = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flCullExp = 0x1E4; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomYawFlip {
                constexpr std::ptrdiff_t m_flPercent = 0x1E0; // float32
                constexpr std::ptrdiff_t m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace SequenceWeightedList_t {
                constexpr std::ptrdiff_t m_nSequence = 0x0; // int32
                constexpr std::ptrdiff_t m_flRelativeWeight = 0x4; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_ReadFromNeighboringParticle {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nIncrement = 0x1E0; // int32
                constexpr std::ptrdiff_t m_DistanceCheck = 0x1E8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInterpolation = 0x358; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RenderText {
                constexpr std::ptrdiff_t m_OutlineColor = 0x228; // Color
                constexpr std::ptrdiff_t m_DefaultText = 0x230; // CUtlString
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LerpToInitialPosition {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_flInterpolation = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nCacheField = 0x350; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0x358; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_vecScale = 0x4C8; // CParticleCollectionVecInput
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // PARTICLE_CP_AXIS_Y
            // PARTICLE_CP_AXIS_Z
            // PARTICLE_CP_AXIS_NEGATIVE_X
            // PARTICLE_CP_AXIS_NEGATIVE_Y
            // PARTICLE_CP_AXIS_NEGATIVE_Z
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomRotation {
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_LerpEndCapVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecOutput = 0x1DC; // Vector
                constexpr std::ptrdiff_t m_flLerpTime = 0x1E8; // float32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_VelocityDecay {
                constexpr std::ptrdiff_t m_flMinVelocity = 0x1D8; // float32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetCPOrientationToPointAtCP {
                constexpr std::ptrdiff_t m_nInputCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nOutputCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_flInterpolation = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_b2DOrientation = 0x358; // bool
                constexpr std::ptrdiff_t m_bAvoidSingularity = 0x359; // bool
                constexpr std::ptrdiff_t m_bPointAway = 0x35A; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_LockToPointList {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_pointList = 0x1E0; // CUtlVector<PointDefinition_t>
                constexpr std::ptrdiff_t m_bPlaceAlongPath = 0x1F8; // bool
                constexpr std::ptrdiff_t m_bClosedLoop = 0x1F9; // bool
                constexpr std::ptrdiff_t m_nNumPointsAlongPath = 0x1FC; // int32
            }
            // Parent: None
            // Field count: 18
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_MovementPlaceOnGround {
                constexpr std::ptrdiff_t m_flOffset = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flMaxTraceLength = 0x348; // float32
                constexpr std::ptrdiff_t m_flTolerance = 0x34C; // float32
                constexpr std::ptrdiff_t m_vecTraceDir = 0x350; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flTraceOffset = 0xA08; // float32
                constexpr std::ptrdiff_t m_flLerpRate = 0xA0C; // float32
                constexpr std::ptrdiff_t m_CollisionGroupName = 0xA10; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0xA90; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_nRefCP1 = 0xA94; // int32
                constexpr std::ptrdiff_t m_nRefCP2 = 0xA98; // int32
                constexpr std::ptrdiff_t m_nLerpCP = 0xA9C; // int32
                constexpr std::ptrdiff_t m_nTraceMissBehavior = 0xAA8; // ParticleTraceMissBehavior_t
                constexpr std::ptrdiff_t m_bIncludeShotHull = 0xAAC; // bool
                constexpr std::ptrdiff_t m_bIncludeWater = 0xAAD; // bool
                constexpr std::ptrdiff_t m_bSetNormal = 0xAB0; // bool
                constexpr std::ptrdiff_t m_bScaleOffset = 0xAB1; // bool
                constexpr std::ptrdiff_t m_nPreserveOffsetCP = 0xAB4; // int32
                constexpr std::ptrdiff_t m_nIgnoreCP = 0xAB8; // int32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetCPOrientationToDirection {
                constexpr std::ptrdiff_t m_nInputControlPoint = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nOutputControlPoint = 0x1DC; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_OP_RemapCrossProductOfTwoVectorsToVector {
                constexpr std::ptrdiff_t m_InputVec1 = 0x1D8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_InputVec2 = 0x890; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0xF48; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bNormalize = 0xF4C; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapTransformOrientationToRotations {
                constexpr std::ptrdiff_t m_TransformInput = 0x1D8; // CParticleTransformInput
                constexpr std::ptrdiff_t m_vecRotation = 0x240; // Vector
                constexpr std::ptrdiff_t m_bUseQuat = 0x24C; // bool
                constexpr std::ptrdiff_t m_bWriteNormal = 0x24D; // bool
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomRotationSpeed {
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_InheritFromParentParticlesV2 {
                constexpr std::ptrdiff_t m_flScale = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x348; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nIncrement = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bSubSample = 0x4C0; // bool
                constexpr std::ptrdiff_t m_bRandomDistribution = 0x4C1; // bool
                constexpr std::ptrdiff_t m_bReverse = 0x4C2; // bool
                constexpr std::ptrdiff_t m_nMissingParentBehavior = 0x4C4; // MissingParentInheritBehavior_t
                constexpr std::ptrdiff_t m_flInterpolation = 0x4C8; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RandomSecondSequence {
                constexpr std::ptrdiff_t m_nSequenceMin = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nSequenceMax = 0x1E4; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_OP_SetFloatCollection {
                constexpr std::ptrdiff_t m_InputValue = 0x1D8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x348; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x34C; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_Lerp = 0x350; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace PointDefinition_t {
                constexpr std::ptrdiff_t m_nControlPoint = 0x0; // int32
                constexpr std::ptrdiff_t m_bLocalCoords = 0x4; // bool
                constexpr std::ptrdiff_t m_vOffset = 0x8; // Vector
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_Diffusion {
                constexpr std::ptrdiff_t m_flRadiusScale = 0x1D8; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nVoxelGridResolution = 0x1E0; // int32
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_AgeNoise {
                constexpr std::ptrdiff_t m_bAbsVal = 0x1E0; // bool
                constexpr std::ptrdiff_t m_bAbsValInv = 0x1E1; // bool
                constexpr std::ptrdiff_t m_flOffset = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flAgeMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flAgeMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flNoiseScale = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flNoiseScaleLoc = 0x1F4; // float32
                constexpr std::ptrdiff_t m_vecOffsetLoc = 0x1F8; // Vector
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapVectorComponentToScalar {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nComponent = 0x1E0; // int32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace CGeneralRandomRotation {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flDegrees = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flDegreesMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flDegreesMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flRotationRandExponent = 0x1F0; // float32
                constexpr std::ptrdiff_t m_bRandomlyFlipDirection = 0x1F4; // bool
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            namespace C_OP_DistanceBetweenVecs {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecPoint1 = 0x1E0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecPoint2 = 0x898; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flInputMin = 0xF50; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x10C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x1230; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x13A0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x1510; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bDeltaTime = 0x1514; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_DampenToCP {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_flRange = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flScale = 0x1E0; // float32
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_CalculateVectorAttribute {
                constexpr std::ptrdiff_t m_vStartValue = 0x1D8; // Vector
                constexpr std::ptrdiff_t m_nFieldInput1 = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputScale1 = 0x1E8; // float32
                constexpr std::ptrdiff_t m_nFieldInput2 = 0x1EC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputScale2 = 0x1F0; // float32
                constexpr std::ptrdiff_t m_nControlPointInput1 = 0x1F4; // ControlPointReference_t
                constexpr std::ptrdiff_t m_flControlPointScale1 = 0x208; // float32
                constexpr std::ptrdiff_t m_nControlPointInput2 = 0x20C; // ControlPointReference_t
                constexpr std::ptrdiff_t m_flControlPointScale2 = 0x220; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x224; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vFinalOutputScale = 0x228; // Vector
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LockToBone {
                constexpr std::ptrdiff_t m_modelInput = 0x1D8; // CParticleModelInput
                constexpr std::ptrdiff_t m_transformInput = 0x238; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flLifeTimeFadeStart = 0x2A0; // float32
                constexpr std::ptrdiff_t m_flLifeTimeFadeEnd = 0x2A4; // float32
                constexpr std::ptrdiff_t m_flJumpThreshold = 0x2A8; // float32
                constexpr std::ptrdiff_t m_flPrevPosScale = 0x2AC; // float32
                constexpr std::ptrdiff_t m_HitboxSetName = 0x2B0; // char[128]
                constexpr std::ptrdiff_t m_bRigid = 0x330; // bool
                constexpr std::ptrdiff_t m_bUseBones = 0x331; // bool
                constexpr std::ptrdiff_t m_nFieldOutput = 0x334; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutputPrev = 0x338; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nRotationSetType = 0x33C; // ParticleRotationLockType_t
                constexpr std::ptrdiff_t m_bRigidRotationLock = 0x340; // bool
                constexpr std::ptrdiff_t m_vecRotation = 0x348; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flRotLerp = 0xA00; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapNamedModelBodyPartOnceTimed {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // p
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ScreenSpaceRotateTowardTarget {
                constexpr std::ptrdiff_t m_vecTargetPosition = 0x1D8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flOutputRemap = 0x890; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nSetMethod = 0xA00; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_flScreenEdgeAlignmentDistance = 0xA08; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_MovementMaintainOffset {
                constexpr std::ptrdiff_t m_vecOffset = 0x1D8; // Vector
                constexpr std::ptrdiff_t m_nCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bRadiusScale = 0x1E8; // bool
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_CreateWithinCapsuleTransform {
                constexpr std::ptrdiff_t m_fRadiusMin = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fRadiusMax = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fHeight = 0x4C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_TransformInput = 0x630; // CParticleTransformInput
                constexpr std::ptrdiff_t m_fSpeedMin = 0x698; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fSpeedMax = 0x808; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fSpeedRandExp = 0x978; // float32
                constexpr std::ptrdiff_t m_LocalCoordinateSystemSpeedMin = 0x980; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_LocalCoordinateSystemSpeedMax = 0x1038; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x16F0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldVelocity = 0x16F4; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_CreateFromParentParticles {
                constexpr std::ptrdiff_t m_flVelocityScale = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flIncrement = 0x1E4; // float32
                constexpr std::ptrdiff_t m_bRandomDistribution = 0x1E8; // bool
                constexpr std::ptrdiff_t m_nRandomSeed = 0x1EC; // int32
                constexpr std::ptrdiff_t m_bSubFrame = 0x1F0; // bool
                constexpr std::ptrdiff_t m_bSetRopeSegmentID = 0x1F1; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_CheckParticleForWater {
                constexpr std::ptrdiff_t m_flRadius = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x350; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOutputRemap = 0x358; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x4C8; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_RandomNamedModelBodyPart {
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace CPAssignment_t {
                constexpr std::ptrdiff_t m_nCPNumber = 0x0; // int32
                constexpr std::ptrdiff_t m_Pos = 0x8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_nOrientationMode = 0x6C0; // ParticleOrientationSetMode_t
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapParticleCountToNamedModelBodyPartScalar {
            }
            // Parent: None
            // Field count: 19
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_InitSkinnedPositionFromCPSnapshot {
                constexpr std::ptrdiff_t m_nSnapshotControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bRandom = 0x1E8; // bool
                constexpr std::ptrdiff_t m_nRandomSeed = 0x1EC; // int32
                constexpr std::ptrdiff_t m_bRigid = 0x1F0; // bool
                constexpr std::ptrdiff_t m_bSetNormal = 0x1F1; // bool
                constexpr std::ptrdiff_t m_bIgnoreDt = 0x1F2; // bool
                constexpr std::ptrdiff_t m_flMinNormalVelocity = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flMaxNormalVelocity = 0x1F8; // float32
                constexpr std::ptrdiff_t m_nIndexType = 0x1FC; // SnapshotIndexType_t
                constexpr std::ptrdiff_t m_flReadIndex = 0x200; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flIncrement = 0x370; // float32
                constexpr std::ptrdiff_t m_nFullLoopIncrement = 0x374; // int32
                constexpr std::ptrdiff_t m_nSnapShotStartPoint = 0x378; // int32
                constexpr std::ptrdiff_t m_flBoneVelocity = 0x37C; // float32
                constexpr std::ptrdiff_t m_flBoneVelocityMax = 0x380; // float32
                constexpr std::ptrdiff_t m_bCopyColor = 0x384; // bool
                constexpr std::ptrdiff_t m_bCopyAlpha = 0x385; // bool
                constexpr std::ptrdiff_t m_bSetRadius = 0x386; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LagCompensation {
                constexpr std::ptrdiff_t m_nDesiredVelocityCP = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nLatencyCP = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nLatencyCPField = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nDesiredVelocityCPField = 0x1E4; // int32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_Noise {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOutputMin = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1E0; // float32
                constexpr std::ptrdiff_t m_fl4NoiseScale = 0x1E4; // float32
                constexpr std::ptrdiff_t m_bAdditive = 0x1E8; // bool
                constexpr std::ptrdiff_t m_flNoiseAnimationTimeScale = 0x1EC; // float32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_FadeAndKillForTracers {
                constexpr std::ptrdiff_t m_flStartFadeInTime = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flEndFadeInTime = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flStartFadeOutTime = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flEndFadeOutTime = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flStartAlpha = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flEndAlpha = 0x1EC; // float32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace CParticleMassCalculationParameters {
                constexpr std::ptrdiff_t m_nMassMode = 0x0; // ParticleMassMode_t
                constexpr std::ptrdiff_t m_flRadius = 0x8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flNominalRadius = 0x178; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flScale = 0x2E8; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SequenceFromModel {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutputAnim = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F0; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x1F4; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_AlphaDecay {
                constexpr std::ptrdiff_t m_flMinAlpha = 0x1D8; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_InitVec {
                constexpr std::ptrdiff_t m_InputValue = 0x1E0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_nOutputField = 0x898; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x89C; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bNormalizedOutput = 0x8A0; // bool
                constexpr std::ptrdiff_t m_bWritePreviousPosition = 0x8A1; // bool
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_SetHitboxToModel {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nForceInModel = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bEvenDistribution = 0x1E8; // bool
                constexpr std::ptrdiff_t m_nDesiredHitbox = 0x1EC; // int32
                constexpr std::ptrdiff_t m_vecHitBoxScale = 0x1F0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_vecDirectionBias = 0x8A8; // Vector
                constexpr std::ptrdiff_t m_bMaintainHitbox = 0x8B4; // bool
                constexpr std::ptrdiff_t m_bUseBones = 0x8B5; // bool
                constexpr std::ptrdiff_t m_HitboxSetName = 0x8B6; // char[128]
                constexpr std::ptrdiff_t m_flShellSize = 0x938; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_MovementMoveAlongSkinnedCPSnapshot {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nSnapshotControlPointNumber = 0x1DC; // int32
                constexpr std::ptrdiff_t m_bSetNormal = 0x1E0; // bool
                constexpr std::ptrdiff_t m_bSetRadius = 0x1E1; // bool
                constexpr std::ptrdiff_t m_flInterpolation = 0x1E8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flTValue = 0x358; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LerpScalar {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOutput = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flStartTime = 0x350; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x354; // float32
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_InitialRepulsionVelocity {
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x1E0; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x260; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_vecOutputMin = 0x264; // Vector
                constexpr std::ptrdiff_t m_vecOutputMax = 0x270; // Vector
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x27C; // int32
                constexpr std::ptrdiff_t m_bPerParticle = 0x280; // bool
                constexpr std::ptrdiff_t m_bTranslate = 0x281; // bool
                constexpr std::ptrdiff_t m_bProportional = 0x282; // bool
                constexpr std::ptrdiff_t m_flTraceLength = 0x284; // float32
                constexpr std::ptrdiff_t m_bPerParticleTR = 0x288; // bool
                constexpr std::ptrdiff_t m_bInherit = 0x289; // bool
                constexpr std::ptrdiff_t m_nChildCP = 0x28C; // int32
                constexpr std::ptrdiff_t m_nChildGroupID = 0x290; // int32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ClampScalar {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x350; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointToHMD {
                constexpr std::ptrdiff_t m_nCP1 = 0x1E0; // int32
                constexpr std::ptrdiff_t m_vecCP1Pos = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_bOrientToHMD = 0x1F0; // bool
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace C_OP_DifferencePreviousParticle {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x1F0; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x1F4; // bool
                constexpr std::ptrdiff_t m_bSetPreviousParticle = 0x1F5; // bool
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_PercentageBetweenTransforms {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1E8; // float32
                constexpr std::ptrdiff_t m_TransformStart = 0x1F0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_TransformEnd = 0x258; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x2C0; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x2C4; // bool
                constexpr std::ptrdiff_t m_bRadialCheck = 0x2C5; // bool
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // PARTICLE_MASSMODE_RADIUS_SQUARED
            // MPropertyFriendlyName
            namespace C_OP_RemapNamedModelSequenceEndCap {
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_INIT_InitFromCPSnapshot {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_strSnapshotSubset = 0x1E8; // CUtlString
                constexpr std::ptrdiff_t m_nAttributeToRead = 0x1F0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nAttributeToWrite = 0x1F4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nLocalSpaceCP = 0x1F8; // int32
                constexpr std::ptrdiff_t m_bRandom = 0x1FC; // bool
                constexpr std::ptrdiff_t m_bReverse = 0x1FD; // bool
                constexpr std::ptrdiff_t m_nSnapShotIncrement = 0x200; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nManualSnapshotIndex = 0x370; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nRandomSeed = 0x4E0; // int32
                constexpr std::ptrdiff_t m_bLocalSpaceAngles = 0x4E4; // bool
            }
            // Parent: None
            // Field count: 24
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            namespace C_OP_RenderCables {
                constexpr std::ptrdiff_t m_flRadiusScale = 0x228; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flAlphaScale = 0x398; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_vecColorScale = 0x508; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nColorBlendType = 0xBC0; // ParticleColorBlendType_t
                constexpr std::ptrdiff_t m_hMaterial = 0xBC8; // CStrongHandle<InfoForResourceTypeIMaterial2>
                constexpr std::ptrdiff_t m_nTextureRepetitionMode = 0xBD0; // TextureRepetitionMode_t
                constexpr std::ptrdiff_t m_flTextureRepeatsPerSegment = 0xBD8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flTextureRepeatsCircumference = 0xD48; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flColorMapOffsetV = 0xEB8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flColorMapOffsetU = 0x1028; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flNormalMapOffsetV = 0x1198; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flNormalMapOffsetU = 0x1308; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bDrawCableCaps = 0x1478; // bool
                constexpr std::ptrdiff_t m_flCapRoundness = 0x147C; // float32
                constexpr std::ptrdiff_t m_flCapOffsetAmount = 0x1480; // float32
                constexpr std::ptrdiff_t m_flTessScale = 0x1484; // float32
                constexpr std::ptrdiff_t m_nMinTesselation = 0x1488; // int32
                constexpr std::ptrdiff_t m_nMaxTesselation = 0x148C; // int32
                constexpr std::ptrdiff_t m_nRoundness = 0x1490; // int32
                constexpr std::ptrdiff_t m_nForceRoundnessFixed = 0x1494; // bool
                constexpr std::ptrdiff_t m_bOnlyRenderInEffectsBloomPass = 0x1495; // bool
                constexpr std::ptrdiff_t m_LightingTransform = 0x1498; // CParticleTransformInput
                constexpr std::ptrdiff_t m_MaterialFloatVars = 0x1500; // CUtlLeanVector<FloatInputMaterialVariable_t>
                constexpr std::ptrdiff_t m_MaterialVecVars = 0x1520; // CUtlLeanVector<VecInputMaterialVariable_t>
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_InheritVelocity {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_flVelocityScale = 0x1E4; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_NormalAlignToCP {
                constexpr std::ptrdiff_t m_transformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nControlPointAxis = 0x248; // ParticleControlPointAxis_t
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            namespace C_OP_SetChildControlPoints {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFirstControlPoint = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nNumControlPoints = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nFirstSourcePoint = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bReverse = 0x358; // bool
                constexpr std::ptrdiff_t m_bSetOrientation = 0x359; // bool
                constexpr std::ptrdiff_t m_nOrientation = 0x35C; // ParticleOrientationType_t
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_OP_ChladniWave {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x4C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x630; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_vecWaveLength = 0x7A0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecHarmonics = 0xE58; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_nSetMethod = 0x1510; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_nLocalSpaceControlPoint = 0x1514; // int32
                constexpr std::ptrdiff_t m_b3D = 0x1518; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_RemapDirectionToCPToVector {
                constexpr std::ptrdiff_t m_nCP = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flOffsetRot = 0x1E4; // float32
                constexpr std::ptrdiff_t m_vecOffsetAxis = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_bNormalize = 0x1F4; // bool
                constexpr std::ptrdiff_t m_nFieldStrength = 0x1F8; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_OP_DriveCPFromGlobalSoundFloat {
                constexpr std::ptrdiff_t m_nOutputControlPoint = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nOutputField = 0x1E4; // int32
                constexpr std::ptrdiff_t m_flInputMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F4; // float32
                constexpr std::ptrdiff_t m_StackName = 0x1F8; // CUtlString
                constexpr std::ptrdiff_t m_OperatorName = 0x200; // CUtlString
                constexpr std::ptrdiff_t m_FieldName = 0x208; // CUtlString
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_ScreenSpacePositionOfTarget {
                constexpr std::ptrdiff_t m_vecTargetPosition = 0x1E0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_bOututBehindness = 0x898; // bool
                constexpr std::ptrdiff_t m_nBehindFieldOutput = 0x89C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flBehindOutputRemap = 0x8A0; // CParticleRemapFloatInput
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RtEnvCull {
                constexpr std::ptrdiff_t m_vecTestDir = 0x1D8; // Vector
                constexpr std::ptrdiff_t m_vecTestNormal = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_bCullOnMiss = 0x1F0; // bool
                constexpr std::ptrdiff_t m_bStickInsteadOfCull = 0x1F1; // bool
                constexpr std::ptrdiff_t m_RtEnvName = 0x1F2; // char[128]
                constexpr std::ptrdiff_t m_nRTEnvCP = 0x274; // int32
                constexpr std::ptrdiff_t m_nComponent = 0x278; // int32
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_PinParticleToCP {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_vecOffset = 0x1E0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_bOffsetLocal = 0x898; // bool
                constexpr std::ptrdiff_t m_nParticleSelection = 0x89C; // ParticleSelection_t
                constexpr std::ptrdiff_t m_nParticleNumber = 0x8A0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nPinBreakType = 0xA10; // ParticlePinDistance_t
                constexpr std::ptrdiff_t m_flBreakDistance = 0xA18; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flBreakSpeed = 0xB88; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flAge = 0xCF8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nBreakControlPointNumber = 0xE68; // int32
                constexpr std::ptrdiff_t m_nBreakControlPointNumber2 = 0xE6C; // int32
                constexpr std::ptrdiff_t m_flBreakValue = 0xE70; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInterpolation = 0xFE0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bRetainInitialVelocity = 0x1150; // bool
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // p
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapCPtoVector {
                constexpr std::ptrdiff_t m_nCPInput = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nLocalSpaceCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_vInputMin = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_vInputMax = 0x1F0; // Vector
                constexpr std::ptrdiff_t m_vOutputMin = 0x1FC; // Vector
                constexpr std::ptrdiff_t m_vOutputMax = 0x208; // Vector
                constexpr std::ptrdiff_t m_flStartTime = 0x214; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x218; // float32
                constexpr std::ptrdiff_t m_flInterpRate = 0x21C; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x220; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bOffset = 0x224; // bool
                constexpr std::ptrdiff_t m_bAccelerate = 0x225; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_CreateParticleImpulse {
                constexpr std::ptrdiff_t m_InputRadius = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_InputMagnitude = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nFalloffFunction = 0x4C0; // ParticleFalloffFunction_t
                constexpr std::ptrdiff_t m_InputFalloffExp = 0x4C8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nImpulseType = 0x638; // ParticleImpulseType_t
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_CreateInEpitrochoid {
                constexpr std::ptrdiff_t m_nComponent1 = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nComponent2 = 0x1E4; // int32
                constexpr std::ptrdiff_t m_TransformInput = 0x1E8; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flParticleDensity = 0x250; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOffset = 0x3C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flRadius1 = 0x530; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flRadius2 = 0x6A0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bUseCount = 0x810; // bool
                constexpr std::ptrdiff_t m_bUseLocalCoords = 0x811; // bool
                constexpr std::ptrdiff_t m_bOffsetExistingPos = 0x812; // bool
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointPositions {
                constexpr std::ptrdiff_t m_bUseWorldLocation = 0x1E0; // bool
                constexpr std::ptrdiff_t m_bOrient = 0x1E1; // bool
                constexpr std::ptrdiff_t m_bSetOnce = 0x1E2; // bool
                constexpr std::ptrdiff_t m_nCP1 = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nCP2 = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nCP3 = 0x1EC; // int32
                constexpr std::ptrdiff_t m_nCP4 = 0x1F0; // int32
                constexpr std::ptrdiff_t m_vecCP1Pos = 0x1F4; // Vector
                constexpr std::ptrdiff_t m_vecCP2Pos = 0x200; // Vector
                constexpr std::ptrdiff_t m_vecCP3Pos = 0x20C; // Vector
                constexpr std::ptrdiff_t m_vecCP4Pos = 0x218; // Vector
                constexpr std::ptrdiff_t m_nHeadLocation = 0x224; // int32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_OP_SetFloatAttributeToVectorExpression {
                constexpr std::ptrdiff_t m_nExpression = 0x1D8; // VectorFloatExpressionType_t
                constexpr std::ptrdiff_t m_vInput1 = 0x1E0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vInput2 = 0x898; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flOutputRemap = 0xF50; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x10C0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x10C4; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_MovementRotateParticleAroundAxis {
                constexpr std::ptrdiff_t m_vecRotAxis = 0x1D8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_flRotRate = 0x890; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_TransformInput = 0xA00; // CParticleTransformInput
                constexpr std::ptrdiff_t m_bLocalSpace = 0xA68; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_InitFloat {
                constexpr std::ptrdiff_t m_InputValue = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x350; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x354; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_InputStrength = 0x358; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_INIT_CreateOnModel {
                constexpr std::ptrdiff_t m_modelInput = 0x1E0; // CParticleModelInput
                constexpr std::ptrdiff_t m_transformInput = 0x240; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nForceInModel = 0x2A8; // int32
                constexpr std::ptrdiff_t m_bScaleToVolume = 0x2AC; // bool
                constexpr std::ptrdiff_t m_bEvenDistribution = 0x2AD; // bool
                constexpr std::ptrdiff_t m_nDesiredHitbox = 0x2B0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nHitboxValueFromControlPointIndex = 0x420; // int32
                constexpr std::ptrdiff_t m_vecHitBoxScale = 0x428; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_flBoneVelocity = 0xAE0; // float32
                constexpr std::ptrdiff_t m_flMaxBoneVelocity = 0xAE4; // float32
                constexpr std::ptrdiff_t m_vecDirectionBias = 0xAE8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_HitboxSetName = 0x11A0; // char[128]
                constexpr std::ptrdiff_t m_bLocalCoords = 0x1220; // bool
                constexpr std::ptrdiff_t m_bUseBones = 0x1221; // bool
                constexpr std::ptrdiff_t m_bUseMesh = 0x1222; // bool
                constexpr std::ptrdiff_t m_flShellSize = 0x1228; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // p
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_InheritFromPeerSystem {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldInput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nIncrement = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nGroupID = 0x1E4; // int32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_RandomNamedModelMeshGroup {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_MaxVelocity {
                constexpr std::ptrdiff_t m_flMaxVelocity = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flMinVelocity = 0x348; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_MaintainEmitter {
                constexpr std::ptrdiff_t m_nParticlesToMaintain = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flStartTime = 0x350; // float32
                constexpr std::ptrdiff_t m_flEmissionDuration = 0x358; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flEmissionRate = 0x4C8; // float32
                constexpr std::ptrdiff_t m_nSnapshotControlPoint = 0x4CC; // int32
                constexpr std::ptrdiff_t m_strSnapshotSubset = 0x4D0; // CUtlString
                constexpr std::ptrdiff_t m_bEmitInstantaneously = 0x4D8; // bool
                constexpr std::ptrdiff_t m_bFinalEmitOnStop = 0x4D9; // bool
                constexpr std::ptrdiff_t m_flScale = 0x4E0; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_PositionOffsetToCP {
                constexpr std::ptrdiff_t m_nControlPointNumberStart = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nControlPointNumberEnd = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bLocalCoords = 0x1E8; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_RemapInitialTransformDirectionToRotation {
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x248; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOffsetRot = 0x24C; // float32
                constexpr std::ptrdiff_t m_nComponent = 0x250; // int32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_FadeAndKill {
                constexpr std::ptrdiff_t m_flStartFadeInTime = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flEndFadeInTime = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flStartFadeOutTime = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flEndFadeOutTime = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flStartAlpha = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flEndAlpha = 0x1EC; // float32
                constexpr std::ptrdiff_t m_bForcePreserveParticleOrder = 0x1F0; // bool
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_RampScalarSpline {
                constexpr std::ptrdiff_t m_RateMin = 0x1D8; // float32
                constexpr std::ptrdiff_t m_RateMax = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flStartTime_min = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flStartTime_max = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flEndTime_min = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flEndTime_max = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flBias = 0x1F0; // float32
                constexpr std::ptrdiff_t m_nField = 0x220; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bProportionalOp = 0x224; // bool
                constexpr std::ptrdiff_t m_bEaseOut = 0x225; // bool
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapNamedModelSequenceOnceTimed {
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_MaintainSequentialPath {
                constexpr std::ptrdiff_t m_fMaxDistance = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flNumToAssign = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flCohesionStrength = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flTolerance = 0x1E4; // float32
                constexpr std::ptrdiff_t m_bLoop = 0x1E8; // bool
                constexpr std::ptrdiff_t m_bUseParticleCount = 0x1E9; // bool
                constexpr std::ptrdiff_t m_PathParams = 0x1F0; // CPathParameters
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapNamedModelBodyPartEndCap {
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_StopAfterCPDuration {
                constexpr std::ptrdiff_t m_flDuration = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bDestroyImmediately = 0x350; // bool
                constexpr std::ptrdiff_t m_bPlayEndCap = 0x351; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace CGeneralSpin {
                constexpr std::ptrdiff_t m_nSpinRateDegrees = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nSpinRateMinDegrees = 0x1DC; // int32
                constexpr std::ptrdiff_t m_fSpinRateStopTime = 0x1E4; // float32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_RemapNamedModelElementToScalar {
                constexpr std::ptrdiff_t m_hModel = 0x1E0; // CStrongHandle<InfoForResourceTypeCModel>
                constexpr std::ptrdiff_t m_names = 0x1E8; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_values = 0x200; // CUtlVector<float32>
                constexpr std::ptrdiff_t m_nFieldInput = 0x218; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x21C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x220; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bModelFromRenderer = 0x224; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ClampVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecOutputMin = 0x1E0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecOutputMax = 0x898; // CPerParticleVecInput
            }
            // Parent: None
            // Field count: 34
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RenderStandardLight {
                constexpr std::ptrdiff_t m_nLightType = 0x228; // ParticleLightTypeChoiceList_t
                constexpr std::ptrdiff_t m_nMaxAllowed = 0x22C; // uint16
                constexpr std::ptrdiff_t m_vecColorScale = 0x230; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nColorBlendType = 0x8E8; // ParticleColorBlendType_t
                constexpr std::ptrdiff_t m_strLightStyle = 0x8F0; // CUtlString
                constexpr std::ptrdiff_t m_flLightStyleTime = 0x8F8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flIntensity = 0xA68; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bCastShadows = 0xBD8; // bool
                constexpr std::ptrdiff_t m_bDynamicBounce = 0xBD9; // bool
                constexpr std::ptrdiff_t m_flBounceScale = 0xBE0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flTheta = 0xD50; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flPhi = 0xEC0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flRadiusMultiplier = 0x1030; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nAttenuationStyle = 0x11A0; // StandardLightingAttenuationStyle_t
                constexpr std::ptrdiff_t m_flFalloffLinearity = 0x11A8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flFiftyPercentFalloff = 0x1318; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flZeroPercentFalloff = 0x1488; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bRenderDiffuse = 0x15F8; // bool
                constexpr std::ptrdiff_t m_bRenderSpecular = 0x15F9; // bool
                constexpr std::ptrdiff_t m_lightCookie = 0x1600; // CUtlString
                constexpr std::ptrdiff_t m_nPriority = 0x1608; // int32
                constexpr std::ptrdiff_t m_nFogLightingMode = 0x160C; // ParticleLightFogLightingMode_t
                constexpr std::ptrdiff_t m_flFogContribution = 0x1610; // CParticleCollectionRendererFloatInput
                constexpr std::ptrdiff_t m_nCapsuleLightBehavior = 0x1780; // ParticleLightBehaviorChoiceList_t
                constexpr std::ptrdiff_t m_flCapsuleLength = 0x1784; // float32
                constexpr std::ptrdiff_t m_bReverseOrder = 0x1788; // bool
                constexpr std::ptrdiff_t m_bClosedLoop = 0x1789; // bool
                constexpr std::ptrdiff_t m_nPrevPntSource = 0x178C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flMaxLength = 0x1790; // float32
                constexpr std::ptrdiff_t m_flMinLength = 0x1794; // float32
                constexpr std::ptrdiff_t m_bIgnoreDT = 0x1798; // bool
                constexpr std::ptrdiff_t m_flConstrainRadiusToLengthRatio = 0x179C; // float32
                constexpr std::ptrdiff_t m_flLengthScale = 0x17A0; // float32
                constexpr std::ptrdiff_t m_flLengthFadeInTime = 0x17A4; // float32
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_DistanceToTransform {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x4C0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x630; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_TransformStart = 0x7A0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_bLOS = 0x808; // bool
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x809; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x88C; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_flMaxTraceLength = 0x890; // float32
                constexpr std::ptrdiff_t m_flLOSScale = 0x894; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x898; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x89C; // bool
                constexpr std::ptrdiff_t m_bAdditive = 0x89D; // bool
                constexpr std::ptrdiff_t m_vecComponentScale = 0x8A0; // CPerParticleVecInput
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_RemapControlPointOrientationToRotation {
                constexpr std::ptrdiff_t m_nCP = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOffsetRot = 0x1E0; // float32
                constexpr std::ptrdiff_t m_nComponent = 0x1E4; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointToCenter {
                constexpr std::ptrdiff_t m_nCP1 = 0x1E0; // int32
                constexpr std::ptrdiff_t m_vecCP1Pos = 0x1E4; // Vector
                constexpr std::ptrdiff_t m_bUseAvgParticlePos = 0x1F0; // bool
                constexpr std::ptrdiff_t m_nSetParent = 0x1F4; // ParticleParentSetMode_t
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapAverageScalarValuetoCP {
                constexpr std::ptrdiff_t m_nExpression = 0x1E0; // SetStatisticExpressionType_t
                constexpr std::ptrdiff_t m_flDecimalPlaces = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nOutControlPointNumber = 0x358; // int32
                constexpr std::ptrdiff_t m_nOutVectorField = 0x35C; // int32
                constexpr std::ptrdiff_t m_nField = 0x360; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOutputRemap = 0x368; // CParticleRemapFloatInput
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapDotProductToScalar {
                constexpr std::ptrdiff_t m_nInputCP1 = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nInputCP2 = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F0; // float32
                constexpr std::ptrdiff_t m_bUseParticleVelocity = 0x1F4; // bool
                constexpr std::ptrdiff_t m_nSetMethod = 0x1F8; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x1FC; // bool
                constexpr std::ptrdiff_t m_bUseParticleNormal = 0x1FD; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_Orient2DRelToCP {
                constexpr std::ptrdiff_t m_nCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flRotOffset = 0x1E8; // float32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_FadeIn {
                constexpr std::ptrdiff_t m_flFadeInTimeMin = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flFadeInTimeMax = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flFadeInTimeExp = 0x1E0; // float32
                constexpr std::ptrdiff_t m_bProportional = 0x1E4; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapVectorToRotations {
                constexpr std::ptrdiff_t m_vecInput = 0x1D8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecRotation = 0x890; // CPerParticleVecInput
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_INIT_GlobalScale {
                constexpr std::ptrdiff_t m_flScale = 0x1E0; // float32
                constexpr std::ptrdiff_t m_nScaleControlPointNumber = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E8; // int32
                constexpr std::ptrdiff_t m_bScaleRadius = 0x1EC; // bool
                constexpr std::ptrdiff_t m_bScalePosition = 0x1ED; // bool
                constexpr std::ptrdiff_t m_bScaleVelocity = 0x1EE; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RadiusFromCPObject {
                constexpr std::ptrdiff_t m_nControlPoint = 0x1E0; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            namespace C_INIT_InitialVelocityFromHitbox {
                constexpr std::ptrdiff_t m_flVelocityMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flVelocityMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E8; // int32
                constexpr std::ptrdiff_t m_HitboxSetName = 0x1EC; // char[128]
                constexpr std::ptrdiff_t m_bUseBones = 0x26C; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LerpVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecOutput = 0x1DC; // Vector
                constexpr std::ptrdiff_t m_flStartTime = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x1EC; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x1F0; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointFieldToWater {
                constexpr std::ptrdiff_t m_nSourceCP = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nDestCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nCPField = 0x1E8; // int32
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // PARTICLE_VRHAND_RIGHT
            // PARTICLE_VRHAND_CP
            namespace TextureGroup_t {
                constexpr std::ptrdiff_t m_bEnabled = 0x0; // bool
                constexpr std::ptrdiff_t m_bReplaceTextureWithGradient = 0x1; // bool
                constexpr std::ptrdiff_t m_hTexture = 0x8; // CStrongHandle<InfoForResourceTypeCTextureBase>
                constexpr std::ptrdiff_t m_Gradient = 0x10; // CColorGradient
                constexpr std::ptrdiff_t m_nTextureType = 0x28; // SpriteCardTextureType_t
                constexpr std::ptrdiff_t m_nTextureChannels = 0x2C; // SpriteCardTextureChannel_t
                constexpr std::ptrdiff_t m_nTextureBlendMode = 0x30; // ParticleTextureLayerBlendType_t
                constexpr std::ptrdiff_t m_flTextureBlend = 0x38; // CParticleCollectionRendererFloatInput
                constexpr std::ptrdiff_t m_TextureControls = 0x1A8; // TextureControls_t
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetCPOrientationToGroundNormal {
                constexpr std::ptrdiff_t m_flInterpRate = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flMaxTraceLength = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flTolerance = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flTraceOffset = 0x1E4; // float32
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x1E8; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x268; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_nInputCP = 0x26C; // int32
                constexpr std::ptrdiff_t m_nOutputCP = 0x270; // int32
                constexpr std::ptrdiff_t m_bIncludeWater = 0x280; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SnapshotSkinToBones {
                constexpr std::ptrdiff_t m_bTransformNormals = 0x1D8; // bool
                constexpr std::ptrdiff_t m_bTransformRadii = 0x1D9; // bool
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1DC; // int32
                constexpr std::ptrdiff_t m_flLifeTimeFadeStart = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flLifeTimeFadeEnd = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flJumpThreshold = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flPrevPosScale = 0x1EC; // float32
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_CreateWithinSphereTransform {
                constexpr std::ptrdiff_t m_fRadiusMin = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fRadiusMax = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_vecDistanceBias = 0x4C0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecDistanceBiasAbs = 0xB78; // Vector
                constexpr std::ptrdiff_t m_TransformInput = 0xB88; // CParticleTransformInput
                constexpr std::ptrdiff_t m_fSpeedMin = 0xBF0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fSpeedMax = 0xD60; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_fSpeedRandExp = 0xED0; // float32
                constexpr std::ptrdiff_t m_bLocalCoords = 0xED4; // bool
                constexpr std::ptrdiff_t m_LocalCoordinateSystemSpeedMin = 0xED8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_LocalCoordinateSystemSpeedMax = 0x1590; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1C48; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldVelocity = 0x1C4C; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RadiusDecay {
                constexpr std::ptrdiff_t m_flMinRadius = 0x1D8; // float32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapNamedModelBodyPartToScalar {
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_INIT_RemapScalarToVector {
                constexpr std::ptrdiff_t m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1EC; // float32
                constexpr std::ptrdiff_t m_vecOutputMin = 0x1F0; // Vector
                constexpr std::ptrdiff_t m_vecOutputMax = 0x1FC; // Vector
                constexpr std::ptrdiff_t m_flStartTime = 0x208; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x20C; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x210; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x214; // int32
                constexpr std::ptrdiff_t m_bLocalCoords = 0x218; // bool
                constexpr std::ptrdiff_t m_flRemapBias = 0x21C; // float32
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_InitialSequenceFromModel {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutputAnim = 0x1E8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F8; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x1FC; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_NoiseEmitter {
                constexpr std::ptrdiff_t m_flEmissionDuration = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flStartTime = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flEmissionScale = 0x1E8; // float32
                constexpr std::ptrdiff_t m_nScaleControlPoint = 0x1EC; // int32
                constexpr std::ptrdiff_t m_nScaleControlPointField = 0x1F0; // int32
                constexpr std::ptrdiff_t m_nWorldNoisePoint = 0x1F4; // int32
                constexpr std::ptrdiff_t m_bAbsVal = 0x1F8; // bool
                constexpr std::ptrdiff_t m_bAbsValInv = 0x1F9; // bool
                constexpr std::ptrdiff_t m_flOffset = 0x1FC; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x200; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x204; // float32
                constexpr std::ptrdiff_t m_flNoiseScale = 0x208; // float32
                constexpr std::ptrdiff_t m_flWorldNoiseScale = 0x20C; // float32
                constexpr std::ptrdiff_t m_vecOffsetLoc = 0x210; // Vector
                constexpr std::ptrdiff_t m_flWorldTimeScale = 0x21C; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SelectivelyEnableChildren {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nFirstChild = 0x350; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nNumChildrenToEnable = 0x4C0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bPlayEndcapOnStop = 0x630; // bool
                constexpr std::ptrdiff_t m_bDestroyImmediately = 0x631; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            namespace ModelReference_t {
                constexpr std::ptrdiff_t m_model = 0x0; // CStrongHandle<InfoForResourceTypeCModel>
                constexpr std::ptrdiff_t m_flRelativeProbabilityOfSpawn = 0x8; // float32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_CreateFromCPs {
                constexpr std::ptrdiff_t m_nIncrement = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nMinCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nMaxCP = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nDynamicCPCount = 0x1F0; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_INIT_CreateSpiralSphere {
                constexpr std::ptrdiff_t m_TransformInput = 0x1E0; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flDensity = 0x248; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInitialRadius = 0x3B8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInitialSpeedMin = 0x528; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInitialSpeedMax = 0x698; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bUseParticleCount = 0x808; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_CPVelocityForce {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E8; // int32
                constexpr std::ptrdiff_t m_flScale = 0x1F0; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_OP_RemapNamedModelElementEndCap {
                constexpr std::ptrdiff_t m_hModel = 0x1D8; // CStrongHandle<InfoForResourceTypeCModel>
                constexpr std::ptrdiff_t m_inNames = 0x1E0; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_outNames = 0x1F8; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_fallbackNames = 0x210; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_bModelFromRenderer = 0x228; // bool
                constexpr std::ptrdiff_t m_nFieldInput = 0x22C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x230; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_ScaleVelocity {
                constexpr std::ptrdiff_t m_vecScale = 0x1E0; // CParticleCollectionVecInput
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_MoveToHitbox {
                constexpr std::ptrdiff_t m_modelInput = 0x1D8; // CParticleModelInput
                constexpr std::ptrdiff_t m_transformInput = 0x238; // CParticleTransformInput
                constexpr std::ptrdiff_t m_flLifeTimeLerpStart = 0x2A4; // float32
                constexpr std::ptrdiff_t m_flLifeTimeLerpEnd = 0x2A8; // float32
                constexpr std::ptrdiff_t m_flPrevPosScale = 0x2AC; // float32
                constexpr std::ptrdiff_t m_HitboxSetName = 0x2B0; // char[128]
                constexpr std::ptrdiff_t m_bUseBones = 0x330; // bool
                constexpr std::ptrdiff_t m_nLerpType = 0x334; // HitboxLerpType_t
                constexpr std::ptrdiff_t m_flInterpolation = 0x338; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_PinRopeSegmentParticleToParent {
                constexpr std::ptrdiff_t m_nParticleSelection = 0x1D8; // ParticleSelection_t
                constexpr std::ptrdiff_t m_nParticleNumber = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flInterpolation = 0x350; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_PointList {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_pointList = 0x1E8; // CUtlVector<PointDefinition_t>
                constexpr std::ptrdiff_t m_bPlaceAlongPath = 0x200; // bool
                constexpr std::ptrdiff_t m_bClosedLoop = 0x201; // bool
                constexpr std::ptrdiff_t m_nNumPointsAlongPath = 0x204; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LerpToOtherAttribute {
                constexpr std::ptrdiff_t m_flInterpolation = 0x1D8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nFieldInputFrom = 0x348; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldInput = 0x34C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x350; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_RemapParticleCountToScalar {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nInputMin = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nInputMax = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nScaleControlPoint = 0x1EC; // int32
                constexpr std::ptrdiff_t m_nScaleControlPointField = 0x1F0; // int32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1F8; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x1FC; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x200; // bool
                constexpr std::ptrdiff_t m_bInvert = 0x201; // bool
                constexpr std::ptrdiff_t m_bWrap = 0x202; // bool
                constexpr std::ptrdiff_t m_flRemapBias = 0x204; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            namespace C_INIT_InheritFromParentParticles {
                constexpr std::ptrdiff_t m_flScale = 0x1E0; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nIncrement = 0x1E8; // int32
                constexpr std::ptrdiff_t m_bRandomDistribution = 0x1EC; // bool
                constexpr std::ptrdiff_t m_nRandomSeed = 0x1F0; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_OP_RampScalarLinearSimple {
                constexpr std::ptrdiff_t m_Rate = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flStartTime = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flEndTime = 0x1E0; // float32
                constexpr std::ptrdiff_t m_nField = 0x210; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_INIT_ChaoticAttractor {
                constexpr std::ptrdiff_t m_flAParm = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flBParm = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flCParm = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flDParm = 0x1EC; // float32
                constexpr std::ptrdiff_t m_flScale = 0x1F0; // float32
                constexpr std::ptrdiff_t m_flSpeedMin = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flSpeedMax = 0x1F8; // float32
                constexpr std::ptrdiff_t m_nBaseCP = 0x1FC; // int32
                constexpr std::ptrdiff_t m_bUniformSpeed = 0x200; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_MovementRigidAttachToCP {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nScaleControlPoint = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nScaleCPField = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nFieldInput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bOffsetLocal = 0x1EC; // bool
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_INIT_DistanceToCPInit {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1E8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInputMax = 0x358; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x4C8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x638; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nStartCP = 0x7A8; // int32
                constexpr std::ptrdiff_t m_bLOS = 0x7AC; // bool
                constexpr std::ptrdiff_t m_CollisionGroupName = 0x7AD; // char[128]
                constexpr std::ptrdiff_t m_nTraceSet = 0x830; // ParticleTraceSet_t
                constexpr std::ptrdiff_t m_flMaxTraceLength = 0x838; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flLOSScale = 0x9A8; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x9AC; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x9B0; // bool
                constexpr std::ptrdiff_t m_vecDistanceScale = 0x9B4; // Vector
                constexpr std::ptrdiff_t m_flRemapBias = 0x9C0; // float32
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            namespace C_OP_EndCapDecay {
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // p
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_RemapDensityToVector {
                constexpr std::ptrdiff_t m_flRadiusScale = 0x1D8; // float32
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1DC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flDensityMin = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flDensityMax = 0x1E4; // float32
                constexpr std::ptrdiff_t m_vecOutputMin = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_vecOutputMax = 0x1F4; // Vector
                constexpr std::ptrdiff_t m_bUseParentDensity = 0x200; // bool
                constexpr std::ptrdiff_t m_nVoxelGridResolution = 0x204; // int32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // PARTICLE_WORLDSPACE_CENTER
            // PARTICLE_EYES
            namespace ParticleControlPointConfiguration_t {
                constexpr std::ptrdiff_t m_name = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_drivers = 0x8; // CUtlVector<ParticleControlPointDriver_t>
                constexpr std::ptrdiff_t m_previewState = 0x20; // ParticlePreviewState_t
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_SetRigidAttachment {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nFieldInput = 0x1E4; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bLocalSpace = 0x1EC; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySortPriority
            namespace MaterialVariable_t {
                constexpr std::ptrdiff_t m_strVariable = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_nVariableField = 0x8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flScale = 0xC; // float32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_RemapSpeed {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flOutputMin = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flOutputMax = 0x1E8; // float32
                constexpr std::ptrdiff_t m_nSetMethod = 0x1EC; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bIgnoreDelta = 0x1F0; // bool
            }
            // Parent: None
            // Field count: 58
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // UNUSED
            // F_NORMAL_MAP
            // F_TEXTURE_LAYERS
            // F_REFRACT_BLUR
            // F_PARTICLE_SHADOWS
            // F_DIRECTIONAL_LIGHTING
            // F_REVERSE_DEPTH
            // F_LIGHTEN_MODE
            // F_MIXED_RENDERING
            // F_PARTICLE_ORIENTATION
            // F_SATURATE_COLOR_PRE_ALPHA_BLEND
            // F_FOG_PARTICLES
            // F_SELF_ILLUM_OVER_1
            // F_PER_PIXEL_LIGHTING
            // F_HAS_NO_NORMAL_FOR_LIGHTING
            // F_NUM_SEQUENCES_PER_PARTICLE
            // F_SELF_ILLUM_PER_PARTICLE
            // F_REFRACT_SOLID
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertySortPriority
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            namespace C_OP_RenderModels {
                constexpr std::ptrdiff_t m_bOnlyRenderInEffectsBloomPass = 0x228; // bool
                constexpr std::ptrdiff_t m_bOnlyRenderInEffectsWaterPass = 0x229; // bool
                constexpr std::ptrdiff_t m_bUseMixedResolutionRendering = 0x22A; // bool
                constexpr std::ptrdiff_t m_bOnlyRenderInEffecsGameOverlay = 0x22B; // bool
                constexpr std::ptrdiff_t m_ModelList = 0x230; // CUtlVector<ModelReference_t>
                constexpr std::ptrdiff_t m_nBodyGroupField = 0x248; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSubModelField = 0x24C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bIgnoreNormal = 0x250; // bool
                constexpr std::ptrdiff_t m_bOrientZ = 0x251; // bool
                constexpr std::ptrdiff_t m_bCenterOffset = 0x252; // bool
                constexpr std::ptrdiff_t m_vecLocalOffset = 0x258; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vecLocalRotation = 0x910; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_bIgnoreRadius = 0xFC8; // bool
                constexpr std::ptrdiff_t m_nModelScaleCP = 0xFCC; // int32
                constexpr std::ptrdiff_t m_vecComponentScale = 0xFD0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_bLocalScale = 0x1688; // bool
                constexpr std::ptrdiff_t m_nSizeCullBloat = 0x168C; // int32
                constexpr std::ptrdiff_t m_bAnimated = 0x1690; // bool
                constexpr std::ptrdiff_t m_flAnimationRate = 0x1698; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_bScaleAnimationRate = 0x1808; // bool
                constexpr std::ptrdiff_t m_bForceLoopingAnimation = 0x1809; // bool
                constexpr std::ptrdiff_t m_bResetAnimOnStop = 0x180A; // bool
                constexpr std::ptrdiff_t m_bManualAnimFrame = 0x180B; // bool
                constexpr std::ptrdiff_t m_nAnimationScaleField = 0x180C; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nAnimationField = 0x1810; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nManualFrameField = 0x1814; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_ActivityName = 0x1818; // char[256]
                constexpr std::ptrdiff_t m_SequenceName = 0x1918; // char[256]
                constexpr std::ptrdiff_t m_bEnableClothSimulation = 0x1A18; // bool
                constexpr std::ptrdiff_t m_bDisableClothGroundCollision = 0x1A19; // bool
                constexpr std::ptrdiff_t m_ClothEffectName = 0x1A1A; // char[64]
                constexpr std::ptrdiff_t m_hOverrideMaterial = 0x1A60; // CStrongHandle<InfoForResourceTypeIMaterial2>
                constexpr std::ptrdiff_t m_bOverrideTranslucentMaterials = 0x1A68; // bool
                constexpr std::ptrdiff_t m_nSkin = 0x1A70; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_MaterialVars = 0x1BE0; // CUtlVector<MaterialVariable_t>
                constexpr std::ptrdiff_t m_flRenderFilter = 0x1BF8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flManualModelSelection = 0x1D68; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_modelInput = 0x1ED8; // CParticleModelInput
                constexpr std::ptrdiff_t m_nLOD = 0x1F38; // int32
                constexpr std::ptrdiff_t m_EconSlotName = 0x1F3C; // char[256]
                constexpr std::ptrdiff_t m_bOriginalModel = 0x203C; // bool
                constexpr std::ptrdiff_t m_bSuppressTint = 0x203D; // bool
                constexpr std::ptrdiff_t m_nSubModelFieldType = 0x2040; // RenderModelSubModelFieldType_t
                constexpr std::ptrdiff_t m_bDisableShadows = 0x2044; // bool
                constexpr std::ptrdiff_t m_bDisableDepthPrepass = 0x2045; // bool
                constexpr std::ptrdiff_t m_bAcceptsDecals = 0x2046; // bool
                constexpr std::ptrdiff_t m_bForceDrawInterlevedWithSiblings = 0x2047; // bool
                constexpr std::ptrdiff_t m_bDoNotDrawInParticlePass = 0x2048; // bool
                constexpr std::ptrdiff_t m_bAllowApproximateTransforms = 0x2049; // bool
                constexpr std::ptrdiff_t m_szRenderAttribute = 0x204A; // char[260]
                constexpr std::ptrdiff_t m_flRadiusScale = 0x2150; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flAlphaScale = 0x22C0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flRollScale = 0x2430; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nAlpha2Field = 0x25A0; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecColorScale = 0x25A8; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nColorBlendType = 0x2C60; // ParticleColorBlendType_t
                constexpr std::ptrdiff_t m_strLightStyle = 0x2C68; // CUtlString
                constexpr std::ptrdiff_t m_flLightStyleTime = 0x2C70; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapNamedModelMeshGroupToScalar {
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            namespace C_INIT_PositionWarpScalar {
                constexpr std::ptrdiff_t m_vecWarpMin = 0x1E0; // Vector
                constexpr std::ptrdiff_t m_vecWarpMax = 0x1EC; // Vector
                constexpr std::ptrdiff_t m_InputValue = 0x1F8; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flPrevPosScale = 0x368; // float32
                constexpr std::ptrdiff_t m_nScaleControlPointNumber = 0x36C; // int32
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x370; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_ForceControlPointStub {
                constexpr std::ptrdiff_t m_ControlPoint = 0x1E0; // int32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_VectorNoise {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecOutputMin = 0x1DC; // Vector
                constexpr std::ptrdiff_t m_vecOutputMax = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_fl4NoiseScale = 0x1F4; // float32
                constexpr std::ptrdiff_t m_bAdditive = 0x1F8; // bool
                constexpr std::ptrdiff_t m_bOffset = 0x1F9; // bool
                constexpr std::ptrdiff_t m_flNoiseAnimationTimeScale = 0x1FC; // float32
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace C_OP_RemapParticleCountToScalar {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nInputMin = 0x1E0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nInputMax = 0x350; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOutputMin = 0x4C0; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_flOutputMax = 0x630; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bActiveRange = 0x7A0; // bool
                constexpr std::ptrdiff_t m_nSetMethod = 0x7A4; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_QuantizeFloat {
                constexpr std::ptrdiff_t m_InputValue = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x350; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetToCP {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1D8; // int32
                constexpr std::ptrdiff_t m_vecOffset = 0x1DC; // Vector
                constexpr std::ptrdiff_t m_bOffsetLocal = 0x1E8; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // PARTICLE_FAN_TYPE_ROTOR_WASH
            // PARTICLE_FAN_TYPE_RADIAL
            namespace ParticleControlPointDriver_t {
                constexpr std::ptrdiff_t m_iControlPoint = 0x0; // ParticleParamID_t
                constexpr std::ptrdiff_t m_iAttachType = 0x10; // ParticleAttachment_t
                constexpr std::ptrdiff_t m_attachmentName = 0x18; // CUtlString
                constexpr std::ptrdiff_t m_vecOffset = 0x20; // Vector
                constexpr std::ptrdiff_t m_angOffset = 0x2C; // QAngle
                constexpr std::ptrdiff_t m_entityName = 0x38; // CUtlString
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_SetControlPointToCPVelocity {
                constexpr std::ptrdiff_t m_nCPInput = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nCPOutputVel = 0x1E4; // int32
                constexpr std::ptrdiff_t m_bNormalize = 0x1E8; // bool
                constexpr std::ptrdiff_t m_nCPOutputMag = 0x1EC; // int32
                constexpr std::ptrdiff_t m_nCPField = 0x1F0; // int32
                constexpr std::ptrdiff_t m_vecComparisonVelocity = 0x1F8; // CParticleCollectionVecInput
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            namespace RenderProjectedMaterial_t {
                constexpr std::ptrdiff_t m_hMaterial = 0x0; // CStrongHandle<InfoForResourceTypeIMaterial2>
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_SetFloatAttributeToVectorExpression {
                constexpr std::ptrdiff_t m_nExpression = 0x1E0; // VectorFloatExpressionType_t
                constexpr std::ptrdiff_t m_vInput1 = 0x1E8; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_vInput2 = 0x8A0; // CPerParticleVecInput
                constexpr std::ptrdiff_t m_flOutputRemap = 0xF58; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x10C8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x10CC; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_ModelCull {
                constexpr std::ptrdiff_t m_nControlPointNumber = 0x1E0; // int32
                constexpr std::ptrdiff_t m_bBoundBox = 0x1E4; // bool
                constexpr std::ptrdiff_t m_bCullOutside = 0x1E5; // bool
                constexpr std::ptrdiff_t m_bUseBones = 0x1E6; // bool
                constexpr std::ptrdiff_t m_HitboxSetName = 0x1E7; // char[128]
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_OP_PercentageBetweenTransformLerpCPs {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flInputMin = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1E0; // float32
                constexpr std::ptrdiff_t m_TransformStart = 0x1E8; // CParticleTransformInput
                constexpr std::ptrdiff_t m_TransformEnd = 0x250; // CParticleTransformInput
                constexpr std::ptrdiff_t m_nOutputStartCP = 0x2B8; // int32
                constexpr std::ptrdiff_t m_nOutputStartField = 0x2BC; // int32
                constexpr std::ptrdiff_t m_nOutputEndCP = 0x2C0; // int32
                constexpr std::ptrdiff_t m_nOutputEndField = 0x2C4; // int32
                constexpr std::ptrdiff_t m_nSetMethod = 0x2C8; // ParticleSetMethod_t
                constexpr std::ptrdiff_t m_bActiveRange = 0x2CC; // bool
                constexpr std::ptrdiff_t m_bRadialCheck = 0x2CD; // bool
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetPerChildControlPoint {
                constexpr std::ptrdiff_t m_nChildGroupID = 0x1D8; // int32
                constexpr std::ptrdiff_t m_nFirstControlPoint = 0x1DC; // int32
                constexpr std::ptrdiff_t m_nNumControlPoints = 0x1E0; // int32
                constexpr std::ptrdiff_t m_nParticleIncrement = 0x1E8; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_nFirstSourcePoint = 0x358; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_bSetOrientation = 0x4C8; // bool
                constexpr std::ptrdiff_t m_nOrientationField = 0x4CC; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bNumBasedOnParticleCount = 0x4D0; // bool
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_SetAttributeToScalarExpression {
                constexpr std::ptrdiff_t m_nExpression = 0x1D8; // ScalarExpressionType_t
                constexpr std::ptrdiff_t m_flInput1 = 0x1E0; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flInput2 = 0x350; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_flOutputRemap = 0x4C0; // CParticleRemapFloatInput
                constexpr std::ptrdiff_t m_nOutputField = 0x630; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_nSetMethod = 0x634; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // UNUSED
            // F_NORMAL_MAP
            // F_TEXTURE_LAYERS
            // F_REFRACT_BLUR
            // F_PARTICLE_SHADOWS
            // F_DIRECTIONAL_LIGHTING
            // F_REVERSE_DEPTH
            // F_LIGHTEN_MODE
            // F_MIXED_RENDERING
            // F_PARTICLE_ORIENTATION
            // F_SATURATE_COLOR_PRE_ALPHA_BLEND
            // F_FOG_PARTICLES
            // F_SELF_ILLUM_OVER_1
            // F_PER_PIXEL_LIGHTING
            // F_HAS_NO_NORMAL_FOR_LIGHTING
            // F_NUM_SEQUENCES_PER_PARTICLE
            // F_SELF_ILLUM_PER_PARTICLE
            // F_REFRACT_SOLID
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertySuppressExpr
            // MPropertySortPriority
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyStartGroup
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySortPriority
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeEditor
            // MPropertyFriendlyName
            namespace C_OP_RenderMaterialProxy {
                constexpr std::ptrdiff_t m_nMaterialControlPoint = 0x228; // int32
                constexpr std::ptrdiff_t m_nProxyType = 0x22C; // MaterialProxyType_t
                constexpr std::ptrdiff_t m_MaterialVars = 0x230; // CUtlVector<MaterialVariable_t>
                constexpr std::ptrdiff_t m_hOverrideMaterial = 0x248; // CStrongHandle<InfoForResourceTypeIMaterial2>
                constexpr std::ptrdiff_t m_flMaterialOverrideEnabled = 0x250; // CParticleCollectionFloatInput
                constexpr std::ptrdiff_t m_vecColorScale = 0x3C0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_flAlpha = 0xA78; // CPerParticleFloatInput
                constexpr std::ptrdiff_t m_nColorBlendType = 0xBE8; // ParticleColorBlendType_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            namespace FloatInputMaterialVariable_t {
                constexpr std::ptrdiff_t m_strVariable = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_flInput = 0x8; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            namespace C_OP_RampScalarLinear {
                constexpr std::ptrdiff_t m_RateMin = 0x1D8; // float32
                constexpr std::ptrdiff_t m_RateMax = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flStartTime_min = 0x1E0; // float32
                constexpr std::ptrdiff_t m_flStartTime_max = 0x1E4; // float32
                constexpr std::ptrdiff_t m_flEndTime_min = 0x1E8; // float32
                constexpr std::ptrdiff_t m_flEndTime_max = 0x1EC; // float32
                constexpr std::ptrdiff_t m_nField = 0x210; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_bProportionalOp = 0x214; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MParticleMinVersion
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            namespace C_OP_RotateVector {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_vecRotAxisMin = 0x1DC; // Vector
                constexpr std::ptrdiff_t m_vecRotAxisMax = 0x1E8; // Vector
                constexpr std::ptrdiff_t m_flRotRateMin = 0x1F4; // float32
                constexpr std::ptrdiff_t m_flRotRateMax = 0x1F8; // float32
                constexpr std::ptrdiff_t m_bNormalize = 0x1FC; // bool
                constexpr std::ptrdiff_t m_flScale = 0x200; // CPerParticleFloatInput
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace C_INIT_InitVecCollection {
                constexpr std::ptrdiff_t m_InputValue = 0x1E0; // CParticleCollectionVecInput
                constexpr std::ptrdiff_t m_nOutputField = 0x898; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_INIT_RemapParticleCountToNamedModelMeshGroupScalar {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            namespace C_INIT_SequenceFromCP {
                constexpr std::ptrdiff_t m_bKillUnused = 0x1E0; // bool
                constexpr std::ptrdiff_t m_bRadiusScale = 0x1E1; // bool
                constexpr std::ptrdiff_t m_nCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_vecOffset = 0x1E8; // Vector
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MParticleMaxVersion
            // MParticleReplacementOp
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_CPOffsetToPercentageBetweenCPs {
                constexpr std::ptrdiff_t m_flInputMin = 0x1D8; // float32
                constexpr std::ptrdiff_t m_flInputMax = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flInputBias = 0x1E0; // float32
                constexpr std::ptrdiff_t m_nStartCP = 0x1E4; // int32
                constexpr std::ptrdiff_t m_nEndCP = 0x1E8; // int32
                constexpr std::ptrdiff_t m_nOffsetCP = 0x1EC; // int32
                constexpr std::ptrdiff_t m_nOuputCP = 0x1F0; // int32
                constexpr std::ptrdiff_t m_nInputCP = 0x1F4; // int32
                constexpr std::ptrdiff_t m_bRadialCheck = 0x1F8; // bool
                constexpr std::ptrdiff_t m_bScaleOffset = 0x1F9; // bool
                constexpr std::ptrdiff_t m_vecOffset = 0x1FC; // Vector
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MVectorIsSometimesCoordinate
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeRange
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            namespace C_OP_LerpEndCapScalar {
                constexpr std::ptrdiff_t m_nFieldOutput = 0x1D8; // ParticleAttributeIndex_t
                constexpr std::ptrdiff_t m_flOutput = 0x1DC; // float32
                constexpr std::ptrdiff_t m_flLerpTime = 0x1E0; // float32
            }
        }
    }
}
