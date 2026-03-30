// Generated using https://github.com/ikhsanprasetyo/Source2Dumped
// 2026-03-30 05:23:54.809776700 UTC

#pragma once

#include <cstddef>
#include <cstdint>

namespace cs2_dumper {
    namespace schemas {
        // Module: particles.dll
        // Class count: 496
        // Enum count: 76
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
            // Member count: 3
            enum class ParticleEntityPos_t : uint32_t {
                PARTICLE_ABS_ORIGIN = 0x0,
                PARTICLE_WORLDSPACE_CENTER = 0x1,
                PARTICLE_EYES = 0x2
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
            // Member count: 11
            enum class EventTypeSelection_t : uint32_t {
                PARTICLE_EVENT_TYPE_MASK_NONE = 0x0,
                PARTICLE_EVENT_TYPE_MASK_SPAWNED = 0x1,
                PARTICLE_EVENT_TYPE_MASK_KILLED = 0x2,
                PARTICLE_EVENT_TYPE_MASK_COLLISION = 0x4,
                PARTICLE_EVENT_TYPE_MASK_FIRST_COLLISION = 0x8,
                PARTICLE_EVENT_TYPE_MASK_COLLISION_STOPPED = 0x10,
                PARTICLE_EVENT_TYPE_MASK_KILLED_ON_COLLISION = 0x20,
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
            // Member count: 7
            enum class ParticleAttrBoxFlags_t : uint32_t {
                PARTICLE_ATTR_BOX_FLAGS_NONE = 0x0,
                PARTICLE_ATTR_BOX_FLAGS_WATER = 0x1,
                PARTICLE_ATTR_BOX_FLAGS_ON_FIRE = 0x2,
                PARTICLE_ATTR_BOX_FLAGS_ELECTRIFIED = 0x4,
                PARTICLE_ATTR_BOX_FLAGS_ASLEEP = 0x8,
                PARTICLE_ATTR_BOX_FLAGS_FROZEN = 0x10,
                PARTICLE_ATTR_BOX_FLAGS_TIMED_DECAY = 0x20
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
            // Member count: 3
            enum class ParticleVolumetricSmokeType_t : uint32_t {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_EMISSION = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_SINK = 0x1,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_REPEL = 0x2
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
            namespace CPulseCell_WaitForCursorsWithTag {
            }
            // Parent: None
            // Field count: 47
            namespace CPulseCell_Base {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 942
            namespace CPulse_ResumePoint {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_PickBestOutflowSelector {
            }
            // Parent: None
            // Field count: 0
            namespace CParticleBindingRealPulse {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_WaitForObservable {
            }
            // Parent: None
            // Field count: 45
            namespace CPulse_OutflowConnection {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 47
            namespace CPulseGraphDef {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_FireCursors {
            }
            // Parent: None
            // Field count: 47
            namespace CPulseCell_Timeline__TimelineEvent_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 49
            namespace CPulseCell_IntervalTimer__CursorState_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_BaseState {
            }
            // Parent: None
            // Field count: 47
            namespace OutflowWithRequirements_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_IsRequirementValid {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Value_Gradient {
            }
            // Parent: None
            // Field count: 45
            namespace CPulseCursorFuncs {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 47
            namespace PulseNodeDynamicOutflows_t__DynamicOutflow_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            namespace CBasePulseGraphInstance {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Inflow_GraphHook {
            }
            // Parent: None
            // Field count: 0
            namespace SignatureOutflow_Resume {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Inflow_BaseEntrypoint {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_WaitForCursorsWithTagBase {
            }
            // Parent: None
            // Field count: 47
            namespace CPulse_InvokeBinding {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_IntervalTimer {
            }
            // Parent: None
            // Field count: 45
            namespace CPulseTestScriptLib {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_BaseLerp {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Value_Curve {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Inflow_EventHandler {
            }
            // Parent: None
            // Field count: 47
            namespace CPulseCell_Outflow_CycleShuffled__InstanceState_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 49
            namespace CPulseCell_BaseLerp__CursorState_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            namespace CPulseCell_WaitForCursorsWithTagBase__CursorState_t {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            namespace CPulseArraylib {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 942
            namespace SignatureOutflow_Continue {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Timeline {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Inflow_EntOutputHandler {
            }
            // Parent: None
            // Field count: 49
            namespace CPulseCell_Outflow_CycleOrdered__InstanceState_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 942
            namespace CParticleCollectionBindingInstance {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 49
            namespace CPulseCell_LimitCount__InstanceState_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Step_DebugLog {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_BaseYieldingInflow {
            }
            // Parent: None
            // Field count: 47
            namespace PulseNodeDynamicOutflows_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            namespace CPulseCell_IsRequirementValid__Criteria_t {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Inflow_ObservableVariableListener {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Outflow_CycleOrdered {
            }
            // Parent: None
            // Field count: 47
            namespace PulseSelectorOutflowList_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Inflow_Wait {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Outflow_CycleShuffled {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Inflow_Method {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_BooleanSwitchState {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Inflow_Yield {
            }
            // Parent: None
            // Field count: 45
            namespace CPulseMathlib {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Outflow_CycleRandom {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_Step_PublicOutput {
            }
            // Parent: None
            // Field count: 47
            namespace CPulse_BlackboardReference {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Value_RandomInt {
            }
            // Parent: None
            // Field count: 47
            namespace CPulse_CallInfo {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_InlineNodeSkipSelector {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_LimitCount {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Step_CallExternalMethod {
            }
            // Parent: None
            // Field count: 47
            namespace PulseObservableBoolExpression_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            namespace CPulseCell_LimitCount__Criteria_t {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            namespace CPulseCell_CursorQueue {
            }
            // Parent: None
            // Field count: 0
            namespace CPulseCell_Value_RandomFloat {
            }
            // Parent: None
            // Field count: 45
            namespace CPulseExecCursor {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            namespace IParticleCollection {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            namespace ParticleAttributeIndex_t {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapGravityToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_Decay {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderDeferredLight {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapSpeedtoCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapTransformToVelocity {
            }
            // Parent: None
            // Field count: 49
            namespace CollisionGroupContext_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            namespace CParticleFunctionPreEmission {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_FadeOutSimple {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SpringToVectorConstraint {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderRopes {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_StatusEffectCitadel {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderSound {
            }
            // Parent: None
            // Field count: 49
            namespace CParticleVisibilityInputs {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointsToParticle {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapCPVelocityToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PointVectorAtNextParticle {
            }
            // Parent: None
            // Field count: 47
            namespace ParticlePreviewBodyGroup_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_OscillateScalarSimple {
            }
            // Parent: ______
            // Field count: 0
            namespace C_INIT_StatusEffect {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RtEnvCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ConstrainDistance {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitialVelocityNoise {
            }
            // Parent: None
            // Field count: 54
            namespace ParticleChildrenInfo_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10740; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100000; // 
                constexpr std::ptrdiff_t  = 0x88; // 
                constexpr std::ptrdiff_t � = 0x1F; // �
                constexpr std::ptrdiff_t  = 0x10018; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t P = 0x102C0; // P
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xBC2B8EA0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t P = 0x4; // P
                constexpr std::ptrdiff_t  = 0x14C3D0; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x65006C; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x750064; // 
                constexpr std::ptrdiff_t  = 0x43002E; // 
                constexpr std::ptrdiff_t  = 0x69004C; // 
                constexpr std::ptrdiff_t  = 0x5C0073; // 
                constexpr std::ptrdiff_t  = 0x650062; // 
                constexpr std::ptrdiff_t  = 0x36006E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x50; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10150; // 
                constexpr std::ptrdiff_t P = 0x310038; // P
                constexpr std::ptrdiff_t  = 0x4E002E; // 
                constexpr std::ptrdiff_t  = 0x55002D; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x10150; // 
                constexpr std::ptrdiff_t  = 0x640064; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20009; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5467122E; // 
                constexpr std::ptrdiff_t  = 0x10150; // 
                constexpr std::ptrdiff_t � = 0x10060; // �
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapScalarOnceTimed {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomNamedModelSequence {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PlaneCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_VelocityRandom {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ModelDampenMovement {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_TwistAroundAxis {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_TeleportBeam {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RemapExternalWindToCP {
            }
            // Parent: None
            // Field count: 0
            namespace CBaseRendererSource2 {
            }
            // Parent: None
            // Field count: 0
            namespace CSpinUpdateBase {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_OrientTo2dDirection {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RemapDotProductToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapParticleCountToNamedModelElementScalar {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RenderTrails {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointPositionToTimeOfDayValue {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DecayMaintainCount {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomModelSequence {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ExternalGameImpulseForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapAverageHitboxSpeedtoCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomAlpha {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_NormalizeVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_FadeInSimple {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RepeatedTriggerChildGroup {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapVelocityToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_SetHitboxToClosest {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RingWave {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomTrailLength {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DistanceBetweenTransforms {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DecayOffscreen {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateSequentialPath {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_EndCapTimedDecay {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RemapDistanceToLineSegmentBase {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ContinuousEmitter {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_OscillateVectorSimple {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_SequenceLifeTime {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_MoveBetweenPoints {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetUserEvent {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_QuantizeFloat {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_BasicMovement {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomNamedModelElement {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitFromParentKilled {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_Callback {
            }
            // Parent: None
            // Field count: 45
            namespace CParticleFunction {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_GlobalLight {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_OffsetVectorToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetPerChildControlPointFromAttribute {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetParentControlPointsToChildCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_BoxConstraint {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreatePhyllotaxis {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_AttractToControlPoint {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomLifeTime {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapParticleCountToNamedModelSequenceScalar {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_VelocityRadialRandom {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomRadius {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_Orient2DRelToCP {
            }
            // Parent: None
            // Field count: 47
            namespace TextureControls_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 49
            namespace ControlPointReference_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetControlPointToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LightningSnapshotGenerator {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RemapNamedModelMeshGroupOnceTimed {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapQAnglesToRotation {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_PositionWarp {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointFieldToScalarExpression {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_CreateParticleSystemRenderer {
            }
            // Parent: None
            // Field count: 942
            namespace CParticleFunctionForce {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomVectorComponent {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_InheritFromParentParticles {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_SetVectorAttributeToVectorExpression {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RemapTransformVisibilityToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DirectionBetweenVecsToVec {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MovementLoopInsideSphere {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderSimpleModelCollection {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_QuantizeCPComponent {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_PlayEndCapWhenFinished {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitFloatCollection {
            }
            // Parent: None
            // Field count: 54
            namespace CPathParameters {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10740; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100000; // 
                constexpr std::ptrdiff_t  = 0x88; // 
                constexpr std::ptrdiff_t � = 0x1F; // �
                constexpr std::ptrdiff_t  = 0x10018; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t P = 0x102C0; // P
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xBC2B8EA0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t P = 0x4; // P
                constexpr std::ptrdiff_t  = 0x14C3D0; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x65006C; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x750064; // 
                constexpr std::ptrdiff_t  = 0x43002E; // 
                constexpr std::ptrdiff_t  = 0x69004C; // 
                constexpr std::ptrdiff_t  = 0x5C0073; // 
                constexpr std::ptrdiff_t  = 0x650062; // 
                constexpr std::ptrdiff_t  = 0x36006E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x50; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10150; // 
                constexpr std::ptrdiff_t P = 0x310038; // P
                constexpr std::ptrdiff_t  = 0x4E002E; // 
                constexpr std::ptrdiff_t  = 0x55002D; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x10150; // 
                constexpr std::ptrdiff_t  = 0x640064; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20009; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5467122E; // 
                constexpr std::ptrdiff_t  = 0x10150; // 
                constexpr std::ptrdiff_t � = 0x10060; // �
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapScalarEndCap {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_CreateFromPlaneCache {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LazyCullCompareFloat {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ControlPointToRadialScreenSpace {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SpinUpdate {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_NormalOffset {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RemapDistanceToLineSegmentToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderAsModels {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreationNoise {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_Spin {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_GameLiquidSpill {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_InstantaneousEmitter {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ConstrainLineLength {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_LifespanFromVelocity {
            }
            // Parent: None
            // Field count: 0
            namespace CBaseTrailRenderer {
            }
            // Parent: ______
            // Field count: 0
            namespace C_INIT_VelocityFromCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointOrientation {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MovementSkinnedPositionFromCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MultiSegmentDisplaySnapshotGenerator {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_OscillateVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PositionLock {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderVRHapticEvent {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetControlPointToImpactPoint {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_InterpolateRadius {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ReinitializeScalarEndCap {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_TurbulenceForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapNamedModelElementOnceTimed {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetControlPointToPlayer {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_EndCapTimedFreeze {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RenderGpuImplicit {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetRandomControlPointPosition {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderVolumetricEmitter {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapTransformVisibilityToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapControlPointDirectionToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ScreenSpacePositionOfTarget {
            }
            // Parent: particles
            // Field count: 942
            namespace CParticleFunctionOperator {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DragRelativeToPlane {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetCPtoVector {
            }
            // Parent: ______
            // Field count: 0
            namespace C_INIT_RandomYaw {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SnapshotRigidSkinToBones {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetSingleControlPointPosition {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_DistanceToNeighborCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapCPtoScalar {
            }
            // Parent: None
            // Field count: 942
            namespace CParticleFunctionRenderer {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapNamedModelMeshGroupEndCap {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PercentageBetweenTransformsVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderScreenVelocityRotate {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_UpdateLightSource {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateWithinBox {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ChooseRandomChildrenInGroup {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ControlpointLight {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_VectorFieldSnapshot {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_CylindricalDistanceToTransform {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_PositionPlaceOnGround {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderPostProcessing {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_WorldTraceConstraint {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderBlobs {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_OscillateScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_FadeOut {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_WaterImpulseRenderer {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomSequence {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RampScalarSplineSimple {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_DistanceCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_CollideWithParentParticles {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitFromVectorFieldSnapshot {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetVectorAttributeToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_AddVectorToVector {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_RemapInitialVisibilityScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapTransformOrientationToYaw {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderStatusEffect {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RandomForce {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RemapParticleCountOnScalarEndCap {
            }
            // Parent: None
            // Field count: 47
            namespace ParticlePreviewState_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LocalAccelerationForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ModelCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetFloat {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapTransformToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ScreenSpaceDistanceToEdge {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapDistanceToLineSegmentToScalar {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RemapVectortoCP {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetFromCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DistanceBetweenCPsToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointToHand {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ConstrainDistanceToPath {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DistanceCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateAlongPath {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_GameDecalRenderer {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_SetControlPointsToModelParticles {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ColorInterpolateRandom {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapNamedModelSequenceToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderLights {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DecayClampCount {
            }
            // Parent: None
            // Field count: 49
            namespace CRandomNumberGeneratorParameters {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_ColorLitPerParticle {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderPoints {
            }
            // Parent: ______
            // Field count: 0
            namespace C_INIT_SetAttributeToScalarExpression {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateOnGrid {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RampCPLinearRandom {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_VelocityMatchingForce {
            }
            // Parent: ______
            // Field count: 0
            namespace C_INIT_RandomAlphaWindowThreshold {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateOnModelAtHeight {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_ModelSurfaceSnapshotGenerator {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RestartAfterDuration {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderClothForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapVisibilityScalar {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_CreateSequentialPathV2 {
            }
            // Parent: None
            // Field count: 47
            namespace VecInputMaterialVariable_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapInitialDirectionToTransformToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LockToSavedSequentialPathV2 {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_NormalLock {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapTransformOrientationToRotations {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_Cull {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomYawFlip {
            }
            // Parent: None
            // Field count: 49
            namespace SequenceWeightedList_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_ReadFromNeighboringParticle {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderText {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_LerpToInitialPosition {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomRotation {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LerpEndCapVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_VelocityDecay {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetCPOrientationToPointAtCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LockToPointList {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MovementPlaceOnGround {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetCPOrientationToDirection {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapCrossProductOfTwoVectorsToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapTransformOrientationToRotations {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomRotationSpeed {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_InheritFromParentParticlesV2 {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomSecondSequence {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetFloatCollection {
            }
            // Parent: None
            // Field count: 49
            namespace PointDefinition_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointPositionToRandomActiveCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_Diffusion {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_AgeNoise {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapVectorComponentToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace CGeneralRandomRotation {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DistanceBetweenVecs {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DampenToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_CalculateVectorAttribute {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LockToBone {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapNamedModelBodyPartOnceTimed {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ScreenSpaceRotateTowardTarget {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_MovementMaintainOffset {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateWithinCapsuleTransform {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetVec {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateFromParentParticles {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CheckParticleForWater {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomNamedModelBodyPart {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderOmni2Light {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ConnectParentParticleToNearest {
            }
            // Parent: None
            // Field count: 47
            namespace CPAssignment_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapParticleCountToNamedModelBodyPartScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitSkinnedPositionFromCPSnapshot {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_LagCompensation {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_CollideWithSelf {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_Noise {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_FadeAndKillForTracers {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ColorAdjustHSL {
            }
            // Parent: None
            // Field count: 47
            namespace CParticleMassCalculationParameters {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SequenceFromModel {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_AlphaDecay {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapDensityGradientToVectorAttribute {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitVec {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_SetHitboxToModel {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MovementMoveAlongSkinnedCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LerpScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitialRepulsionVelocity {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ClampScalar {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetControlPointToHMD {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DifferencePreviousParticle {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointFieldFromVectorExpression {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_PercentageBetweenTransforms {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_PlaneCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapNamedModelSequenceEndCap {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitFromCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderCables {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InheritVelocity {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointToWaterSurface {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_PositionOffset {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_NormalAlignToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ShapeMatchingConstraint {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetChildControlPoints {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ChladniWave {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapDirectionToCPToVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DriveCPFromGlobalSoundFloat {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_ScreenSpacePositionOfTarget {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RtEnvCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PinParticleToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapCPtoVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateParticleImpulse {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_DensityForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateInEpitrochoid {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ConstrainDistanceToUserSpecifiedPath {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetControlPointPositions {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetFloatAttributeToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MovementRotateParticleAroundAxis {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_IntraParticleForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitFloat {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateOnModel {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_InheritFromPeerSystem {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PerParticleForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomNamedModelMeshGroup {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderProjected {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_MaxVelocity {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_VelocityFromNormal {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MaintainEmitter {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_PositionOffsetToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapInitialTransformDirectionToRotation {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_FadeAndKill {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ColorInterpolate {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RampScalarSpline {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapNamedModelSequenceOnceTimed {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_SetControlPointFromObjectScale {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MaintainSequentialPath {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapNamedModelBodyPartEndCap {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_StopAfterCPDuration {
            }
            // Parent: None
            // Field count: 0
            namespace CGeneralSpin {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LockToSavedSequentialPath {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapNamedModelElementToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ClampVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderStatusEffectCitadel {
            }
            // Parent: None
            // Field count: 45
            namespace IParticleSystemDefinition {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x72006100; // 
                constexpr std::ptrdiff_t  = 0x6D006D00; // 
                constexpr std::ptrdiff_t  = 0x67005C00; // 
                constexpr std::ptrdiff_t  = 0x69006C00; // 
                constexpr std::ptrdiff_t  = 0x64002E00; // 
                constexpr std::ptrdiff_t  = 0x72006200; // 
                constexpr std::ptrdiff_t  = 0x6F006300; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x5C003400; // 
                constexpr std::ptrdiff_t  = 0x30002D00; // 
                constexpr std::ptrdiff_t  = 0x3A004400; // 
                constexpr std::ptrdiff_t  = 0x74007300; // 
                constexpr std::ptrdiff_t  = 0x64005C00; // 
                constexpr std::ptrdiff_t  = 0x5C006500; // 
                constexpr std::ptrdiff_t  = 0x55002D00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_WindForce {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_SetVariable {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderStandardLight {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_DistanceToTransform {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RemapControlPointOrientationToRotation {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointToCenter {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapAverageScalarValuetoCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapDotProductToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapCPtoCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointRotation {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_CurlNoiseForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_Orient2DRelToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetSimulationRate {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_FadeIn {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderScreenShake {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapBoundingVolumetoCP {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_HSVShiftToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapVectorToRotations {
            }
            // Parent: ______
            // Field count: 0
            namespace C_INIT_GlobalScale {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_RadiusFromCPObject {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InitialVelocityFromHitbox {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LerpVector {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointFieldToWater {
            }
            // Parent: None
            // Field count: 47
            namespace TextureGroup_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_TimeVaryingForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetCPOrientationToGroundNormal {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SnapshotSkinToBones {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateWithinSphereTransform {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RadiusDecay {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapNamedModelBodyPartToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapScalarToVector {
            }
            // Parent: ______
            // Field count: 0
            namespace C_INIT_InitialSequenceFromModel {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_NoiseEmitter {
            }
            // Parent: None
            // Field count: 942
            namespace CParticleFunctionInitializer {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SelectivelyEnableChildren {
            }
            // Parent: None
            // Field count: 47
            namespace ModelReference_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PlanarConstraint {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_CreateFromCPs {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LockPoints {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_CreateSpiralSphere {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_CPVelocityForce {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RemapNamedModelElementEndCap {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_ScaleVelocity {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_MoveToHitbox {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PinRopeSegmentParticleToParent {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_PointList {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LerpToOtherAttribute {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RandomColor {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetGravityToCP {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_RemapParticleCountToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_InheritFromParentParticles {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RampScalarLinearSimple {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_ChaoticAttractor {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_MovementRigidAttachToCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderFlattenGrass {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderLightBeam {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_EnableChildrenFromParentParticleCount {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_DistanceToCPInit {
            }
            // Parent: None
            // Field count: 47
            namespace CReplicationParameters {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_EndCapDecay {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ForceBasedOnDistanceToPlane {
            }
            // Parent: ______
            // Field count: 0
            namespace C_OP_RemapDensityToVector {
            }
            // Parent: None
            // Field count: 47
            namespace ParticleControlPointConfiguration_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_SetRigidAttachment {
            }
            // Parent: None
            // Field count: 47
            namespace MaterialVariable_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 942
            namespace CParticleFunctionConstraint {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapSpeed {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderModels {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderClientPhysicsImpulse {
            }
            // Parent: None
            // Field count: 942
            namespace CParticleFunctionEmitter {
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1B00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0xF00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0xCCCCCC00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80000040; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x1F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xB00; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10007208; // 
                constexpr std::ptrdiff_t  = 0x1000D20B; // 
                constexpr std::ptrdiff_t  = 0x100E0600; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x870A3D3F; // 
                constexpr std::ptrdiff_t  = 0x1000D20A; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x59999A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0xA00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x12FF; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4EB85200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0xD555553E; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10010600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001F3F; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x2001; // 
                constexpr std::ptrdiff_t  = 0x10000A03; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10002208; // 
                constexpr std::ptrdiff_t  = 0x10002209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000C20F; // 
                constexpr std::ptrdiff_t 
                 = 0x700000C0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10001A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x107B4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x2F41; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003205; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x6147AEBD; // 
                constexpr std::ptrdiff_t  = 0x3B; // 
                constexpr std::ptrdiff_t  = 0x10080600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x10001207; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x4001F00; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10001208; // 
                constexpr std::ptrdiff_t  = 0x1000120A; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x1000720C; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x23F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x32BF; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x363F; // 
                constexpr std::ptrdiff_t  = 0x100AA600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10075600; // 
                constexpr std::ptrdiff_t  = 0x10084600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x10055600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x1000F20B; // 
                constexpr std::ptrdiff_t  = 0x8000003F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x1000F20A; // 
                constexpr std::ptrdiff_t 
                 = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000F205; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x800000BF; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0xE3F; // 
                constexpr std::ptrdiff_t  = 0x1000520F; // 
                constexpr std::ptrdiff_t  = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000320C; // 
                constexpr std::ptrdiff_t  = 0x3F; // 
                constexpr std::ptrdiff_t  = 0x1800; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x107E4600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x10004208; // 
                constexpr std::ptrdiff_t  = 0x10004207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x3601; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10090600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x900; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000820A; // 
                constexpr std::ptrdiff_t  = 0x3D; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x10003A03; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x1500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0xD0955600; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x1900; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x6147AE00; // 
                constexpr std::ptrdiff_t  = 0x4D2E1C00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t 
                 = 0xBF; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x1000520A; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10008600; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x10008208; // 
                constexpr std::ptrdiff_t  = 0x10008205; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x40; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5600; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10004205; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x800; // 
                constexpr std::ptrdiff_t  = 0x4001F01; // 
                constexpr std::ptrdiff_t  = 0x4500; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10101A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1DBD; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x10004600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10600000; // 
                constexpr std::ptrdiff_t  = 0x1000F209; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x3700; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000E208; // 
                constexpr std::ptrdiff_t  = 0x10007209; // 
                constexpr std::ptrdiff_t  = 0x3600; // 
                constexpr std::ptrdiff_t  = 0x500; // 
                constexpr std::ptrdiff_t  = 0x1645A23E; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10007205; // 
                constexpr std::ptrdiff_t  = 0x2000; // 
                constexpr std::ptrdiff_t  = 0x10002207; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10040600; // 
                constexpr std::ptrdiff_t  = 0x100E8600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x1000320A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400103; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x323F; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x601; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x93A92A3F; // 
                constexpr std::ptrdiff_t  = 0x300; // 
                constexpr std::ptrdiff_t  = 0x100E4600; // 
                constexpr std::ptrdiff_t  = 0x10000600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x1000720A; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20824600; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10000A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10103A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1700; // 
                constexpr std::ptrdiff_t  = 0x20801A00; // 
                constexpr std::ptrdiff_t  = 0x10002A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x20802A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400200; // 
                constexpr std::ptrdiff_t  = 0x1000E20A; // 
                constexpr std::ptrdiff_t  = 0x1A00; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x4B00; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x10004209; // 
                constexpr std::ptrdiff_t  = 0x3200; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x200; // 
                constexpr std::ptrdiff_t  = 0x10003A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x100FF600; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x75564E10; // 
                constexpr std::ptrdiff_t  = 0x2C00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF7FF0001; // 
                constexpr std::ptrdiff_t  = 0x67FF15E0; // 
                constexpr std::ptrdiff_t  = 0x7F01201; // 
                constexpr std::ptrdiff_t  = 0x671404C0; // 
                constexpr std::ptrdiff_t  = 0x800000D8; // 
                constexpr std::ptrdiff_t  = 0x7091C49; // 
                constexpr std::ptrdiff_t  = 0xC7191F49; // 
                constexpr std::ptrdiff_t  = 0x671F1F5C; // 
                constexpr std::ptrdiff_t  = 0x6717175C; // 
                constexpr std::ptrdiff_t  = 0x671D1D5C; // 
                constexpr std::ptrdiff_t  = 0x701165C; // 
                constexpr std::ptrdiff_t  = 0xF7161E5C; // 
                constexpr std::ptrdiff_t  = 0x771E1E5C; // 
                constexpr std::ptrdiff_t  = 0x71B1B5C; // 
                constexpr std::ptrdiff_t  = 0xD717074C; // 
                constexpr std::ptrdiff_t  = 0x970D085C; // 
                constexpr std::ptrdiff_t  = 0xF7F01F5C; // 
                constexpr std::ptrdiff_t  = 0x8701085C; // 
                constexpr std::ptrdiff_t  = 0x17F0025C; // 
                constexpr std::ptrdiff_t  = 0x8705085C; // 
                constexpr std::ptrdiff_t  = 0x17F00A5C; // 
                constexpr std::ptrdiff_t  = 0x37161601; // 
                constexpr std::ptrdiff_t  = 0xF7140C5C; // 
                constexpr std::ptrdiff_t  = 0x271400C0; // 
                constexpr std::ptrdiff_t  = 0x37181D50; // 
                constexpr std::ptrdiff_t  = 0xA70E18F0; // 
                constexpr std::ptrdiff_t  = 0x71B0A49; // 
                constexpr std::ptrdiff_t  = 0xA090B39; // 
                constexpr std::ptrdiff_t  = 0x70F0A49; // 
                constexpr std::ptrdiff_t  = 0x8706185C; // 
                constexpr std::ptrdiff_t  = 0x7190732; // 
                constexpr std::ptrdiff_t  = 0x71D0732; // 
                constexpr std::ptrdiff_t  = 0xE7031E38; // 
                constexpr std::ptrdiff_t  = 0x770A0A49; // 
                constexpr std::ptrdiff_t  = 0xE7000E33; // 
                constexpr std::ptrdiff_t  = 0xE7131F32; // 
                constexpr std::ptrdiff_t  = 0xE7101032; // 
                constexpr std::ptrdiff_t  = 0xB7080732; // 
                constexpr std::ptrdiff_t  = 0x37FF1A01; // 
                constexpr std::ptrdiff_t  = 0x27170C58; // 
                constexpr std::ptrdiff_t  = 0x971B1B5C; // 
                constexpr std::ptrdiff_t  = 0xB7161C4B; // 
                constexpr std::ptrdiff_t  = 0x57FF045C; // 
                constexpr std::ptrdiff_t  = 0x70B0F5C; // 
                constexpr std::ptrdiff_t  = 0x37010F5C; // 
                constexpr std::ptrdiff_t  = 0x67090E5C; // 
                constexpr std::ptrdiff_t  = 0x4717175C; // 
                constexpr std::ptrdiff_t  = 0xEA0E02D8; // 
                constexpr std::ptrdiff_t  = 0x8000005C; // 
                constexpr std::ptrdiff_t  = 0xC7FF095C; // 
                constexpr std::ptrdiff_t  = 0x71B0F5C; // 
                constexpr std::ptrdiff_t  = 0x709095C; // 
                constexpr std::ptrdiff_t  = 0xE7000C38; // 
                constexpr std::ptrdiff_t  = 0x1713115C; // 
                constexpr std::ptrdiff_t  = 0xA011849; // 
                constexpr std::ptrdiff_t  = 0x9702FF5C; // 
                constexpr std::ptrdiff_t  = 0x97021A5B; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0xE3041E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x90319D8; // 
                constexpr std::ptrdiff_t  = 0xF71900E3; // 
                constexpr std::ptrdiff_t  = 0x97008F5C; // 
                constexpr std::ptrdiff_t  = 0x171CFF5B; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0x5B; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0xC80E0250; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0x9700875C; // 
                constexpr std::ptrdiff_t  = 0xF71C0058; // 
                constexpr std::ptrdiff_t  = 0xF71C1FE2; // 
                constexpr std::ptrdiff_t  = 0xF71C075B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D00D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E02E3; // 
                constexpr std::ptrdiff_t  = 0x9011849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80319D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0x8A000F5C; // 
                constexpr std::ptrdiff_t  = 0xC70E19F0; // 
                constexpr std::ptrdiff_t  = 0x770D18E3; // 
                constexpr std::ptrdiff_t  = 0x790D02D8; // 
                constexpr std::ptrdiff_t  = 0x7000F49; // 
                constexpr std::ptrdiff_t  = 0x9702005C; // 
                constexpr std::ptrdiff_t  = 0x3000F5C; // 
                constexpr std::ptrdiff_t  = 0x9703A7E2; // 
                constexpr std::ptrdiff_t  = 0xE3060E5B; // 
                constexpr std::ptrdiff_t  = 0x7404175C; // 
                constexpr std::ptrdiff_t  = 0xFE2; // 
                constexpr std::ptrdiff_t  = 0x1000FF0; // 
                constexpr std::ptrdiff_t  = 0xC80E00E3; // 
                constexpr std::ptrdiff_t  = 0x9031849; // 
                constexpr std::ptrdiff_t  = 0xF718035C; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0x8000005B; // 
                constexpr std::ptrdiff_t  = 0x97038F5B; // 
                constexpr std::ptrdiff_t  = 0xD4060D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x97030158; // 
                constexpr std::ptrdiff_t  = 0x97029FE2; // 
                constexpr std::ptrdiff_t  = 0x9702875B; // 
                constexpr std::ptrdiff_t  = 0xC3040C5C; // 
                constexpr std::ptrdiff_t  = 0xE7001B5C; // 
                constexpr std::ptrdiff_t  = 0x77001D5C; // 
                constexpr std::ptrdiff_t  = 0xC7000D5C; // 
                constexpr std::ptrdiff_t  = 0xE2; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x7000FD8; // 
                constexpr std::ptrdiff_t  = 0x80119D8; // 
                constexpr std::ptrdiff_t  = 0xF71902E3; // 
                constexpr std::ptrdiff_t  = 0x7F02750; // 
                constexpr std::ptrdiff_t  = 0x1700FF58; // 
                constexpr std::ptrdiff_t  = 0xA001F5B; // 
                constexpr std::ptrdiff_t  = 0xA3041A5C; // 
                constexpr std::ptrdiff_t  = 0xBA000E5C; // 
                constexpr std::ptrdiff_t  = 0xDA00175C; // 
                constexpr std::ptrdiff_t  = 0xE7140E5B; // 
                constexpr std::ptrdiff_t  = 0x70150D5C; // 
                constexpr std::ptrdiff_t  = 0x670F165C; // 
                constexpr std::ptrdiff_t  = 0x870F185C; // 
                constexpr std::ptrdiff_t  = 0x670D0D5B; // 
                constexpr std::ptrdiff_t  = 0xF70B015C; // 
                constexpr std::ptrdiff_t  = 0xA7150A5C; // 
                constexpr std::ptrdiff_t  = 0x77110449; // 
                constexpr std::ptrdiff_t  = 0x7130049; // 
                constexpr std::ptrdiff_t  = 0x37130359; // 
                constexpr std::ptrdiff_t  = 0x2704045C; // 
                constexpr std::ptrdiff_t  = 0x703875C; // 
                constexpr std::ptrdiff_t  = 0x87000FE3; // 
                constexpr std::ptrdiff_t  = 0x70F0050; // 
                constexpr std::ptrdiff_t  = 0xC122BB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8F800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xB5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCB000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x127000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x14A800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x160800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x189000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10001A00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x3800; // 
                constexpr std::ptrdiff_t  = 0x10008209; // 
                constexpr std::ptrdiff_t  = 0x203600; // 
                constexpr std::ptrdiff_t  = 0x20803A00; // 
                constexpr std::ptrdiff_t  = 0x4180; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0x10001205; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x10024600; // 
                constexpr std::ptrdiff_t  = 0xA1C91000; // 
                constexpr std::ptrdiff_t  = 0x8F16DC83; // 
                constexpr std::ptrdiff_t  = 0x1A5A16DD; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF14B0010; // 
                constexpr std::ptrdiff_t  = 0x8D00; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0xB0C62000; // 
                constexpr std::ptrdiff_t 8 = 0x26AE800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5E; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310F1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BC000; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x48000; // 
                constexpr std::ptrdiff_t  = 0x7094F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B4800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x5047B000; // 
                constexpr std::ptrdiff_t  = 0x2798800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3104F000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1AEA00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x838000; // 
                constexpr std::ptrdiff_t  = 0x6FA5F000; // 
                constexpr std::ptrdiff_t  = 0x440100; // 
                constexpr std::ptrdiff_t  = 0x20000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x383D600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7B00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3000; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2882800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDA; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x31115000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x78; // 
                constexpr std::ptrdiff_t  = 0x1B0400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x71033000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x12FC00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x4F8EB000; // 
                constexpr std::ptrdiff_t r = 0x296C800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x30B69000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BE300; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x480000; // 
                constexpr std::ptrdiff_t  = 0x6F98F000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x46D400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x783780BF; // 
                constexpr std::ptrdiff_t  = 0x40260000; // 
                constexpr std::ptrdiff_t  = 0x2A56800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFC; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x310B5000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF; // 
                constexpr std::ptrdiff_t  = 0x1BBE00; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x258000; // 
                constexpr std::ptrdiff_t  = 0x70F0B000; // 
                constexpr std::ptrdiff_t  = 0x2040100; // 
                constexpr std::ptrdiff_t  = 0x7B000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4200; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7834D094; // 
                constexpr std::ptrdiff_t �� = 0x40DC8000; // 
                constexpr std::ptrdiff_t  = 0x2B40800; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapNamedModelMeshGroupToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointOrientationToCPVelocity {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_RopeSpringConstraint {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_PositionWarpScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ForceControlPointStub {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_VectorNoise {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapParticleCountToScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_QuantizeFloat {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RemapModelVolumetoCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetToCP {
            }
            // Parent: None
            // Field count: 47
            namespace ParticleControlPointDriver_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ParentVortices {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetControlPointToCPVelocity {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ClientPhysics {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SpinYaw {
            }
            // Parent: None
            // Field count: 949
            namespace PointDefinitionWithTimeValues_t {
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x208E46; // 
                constexpr std::ptrdiff_t  = 0x208E46; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x5555; // 
                constexpr std::ptrdiff_t  = 0x3001062; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x3000065; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x10001A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x7; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x8000000; // 
                constexpr std::ptrdiff_t  = 0x9000032; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7000001; // 
                constexpr std::ptrdiff_t  = 0x100042; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x107E46; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x100AA6; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100006; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x107E46; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x100556; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x7000055; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x9000032; // 
                constexpr std::ptrdiff_t  = 0xBF000000; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x10001A; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106000; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x107E46; // 
                constexpr std::ptrdiff_t  = 0x5000036; // 
                constexpr std::ptrdiff_t  = 0x100046; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x1000015; // 
                constexpr std::ptrdiff_t  = 0x8000000; // 
                constexpr std::ptrdiff_t  = 0x9000032; // 
                constexpr std::ptrdiff_t  = 0xB; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8000000; // 
                constexpr std::ptrdiff_t  = 0x9000032; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x700004F; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x414EB852; // 
                constexpr std::ptrdiff_t  = 0xA000038; // 
                constexpr std::ptrdiff_t  = 0x3ED55555; // 
                constexpr std::ptrdiff_t  = 0x100072; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x9000037; // 
                constexpr std::ptrdiff_t  = 0xA; // 
                constexpr std::ptrdiff_t  = 0x3F372474; // 
                constexpr std::ptrdiff_t  = 0x10000A; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x7000018; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x1000015; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x106000; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x100E46; // 
                constexpr std::ptrdiff_t  = 0x7000020; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xA000038; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100032; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x9000037; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100206; // 
                constexpr std::ptrdiff_t  = 0x3E000000; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x7000018; // 
                constexpr std::ptrdiff_t  = 0x10000A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x1000015; // 
                constexpr std::ptrdiff_t  = 0x100012; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x100012; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x10100A; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0xA; // 
                constexpr std::ptrdiff_t  = 0x300001F; // 
                constexpr std::ptrdiff_t  = 0x9000045; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10101A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100906; // 
                constexpr std::ptrdiff_t  = 0x100006; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x8010003A; // 
                constexpr std::ptrdiff_t  = 0x10000A; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100806; // 
                constexpr std::ptrdiff_t  = 0x100032; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3B4D2E1C; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x100EA6; // 
                constexpr std::ptrdiff_t  = 0x1000C2; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x80100006; // 
                constexpr std::ptrdiff_t  = 0x10001A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x1000015; // 
                constexpr std::ptrdiff_t  = 0x100006; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x300004C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100AA6; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x3000006; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x100906; // 
                constexpr std::ptrdiff_t  = 0x100072; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xA000010; // 
                constexpr std::ptrdiff_t  = 0x3DE978D5; // 
                constexpr std::ptrdiff_t  = 0x3000006; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x7000001; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x100042; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xA000032; // 
                constexpr std::ptrdiff_t  = 0x100AE6; // 
                constexpr std::ptrdiff_t  = 0x3000006; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x80100AA6; // 
                constexpr std::ptrdiff_t  = 0x100AA6; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xA000000; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x100E46; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0xBF800000; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x300001F; // 
                constexpr std::ptrdiff_t  = 0x1000012; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100042; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x10100A; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x300001F; // 
                constexpr std::ptrdiff_t  = 0x9000045; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10101A; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100E46; // 
                constexpr std::ptrdiff_t  = 0x100AA6; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8010003A; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x100042; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x1000D2; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0xB000032; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x80100386; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x500002F; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x100072; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0xBD6147AE; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100E06; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x9000032; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5000056; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0xA000010; // 
                constexpr std::ptrdiff_t  = 0x3DE978D5; // 
                constexpr std::ptrdiff_t  = 0x100072; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100042; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x500002F; // 
                constexpr std::ptrdiff_t  = 0x100046; // 
                constexpr std::ptrdiff_t  = 0x100032; // 
                constexpr std::ptrdiff_t  = 0x6; // 
                constexpr std::ptrdiff_t  = 0xBD6147AE; // 
                constexpr std::ptrdiff_t  = 0x3B4D2E1C; // 
                constexpr std::ptrdiff_t  = 0x100106; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x7000000; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x5000056; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x1000015; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100A26; // 
                constexpr std::ptrdiff_t  = 0x100012; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x107B46; // 
                constexpr std::ptrdiff_t  = 0x5000036; // 
                constexpr std::ptrdiff_t  = 0x1005D6; // 
                constexpr std::ptrdiff_t  = 0x10000A; // 
                constexpr std::ptrdiff_t  = 0x1000F2; // 
                constexpr std::ptrdiff_t  = 0x1000015; // 
                constexpr std::ptrdiff_t  = 0x5; // 
                constexpr std::ptrdiff_t  = 0x106000; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x90901A; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20803A; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x414EB852; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xF000032; // 
                constexpr std::ptrdiff_t  = 0x100032; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100AE6; // 
                constexpr std::ptrdiff_t  = 0xBF000000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x100406; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x304001F; // 
                constexpr std::ptrdiff_t  = 0x107E46; // 
                constexpr std::ptrdiff_t  = 0x9000045; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100012; // 
                constexpr std::ptrdiff_t  = 0x100012; // 
                constexpr std::ptrdiff_t  = 0x9; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x10001A; // 
                constexpr std::ptrdiff_t  = 0x3000006; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100556; // 
                constexpr std::ptrdiff_t  = 0x3000006; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0xA000010; // 
                constexpr std::ptrdiff_t  = 0x3DE978D5; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0xA000032; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x3E991687; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0xA000032; // 
                constexpr std::ptrdiff_t  = 0x100756; // 
                constexpr std::ptrdiff_t  = 0x3000006; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5000036; // 
                constexpr std::ptrdiff_t  = 0xC; // 
                constexpr std::ptrdiff_t  = 0x7000001; // 
                constexpr std::ptrdiff_t  = 0x100062; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0xA000032; // 
                constexpr std::ptrdiff_t  = 0x100556; // 
                constexpr std::ptrdiff_t  = 0x3000006; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x100E46; // 
                constexpr std::ptrdiff_t  = 0x8000000; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x100E46; // 
                constexpr std::ptrdiff_t  = 0x4001; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x3F000000; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1000002; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xA000032; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x10103A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x8020803A; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x8010002A; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x10103A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x8000036; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0xA000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x208246; // 
                constexpr std::ptrdiff_t  = 0x10002A; // 
                constexpr std::ptrdiff_t  = 0x3F800000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x8000000; // 
                constexpr std::ptrdiff_t  = 0x7000010; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10003A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8020803A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0xA000032; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x8; // 
                constexpr std::ptrdiff_t  = 0x1B; // 
                constexpr std::ptrdiff_t  = 0x50; // 
                constexpr std::ptrdiff_t  = 0x1B80; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x20; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x618; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE003FF87; // 
                constexpr std::ptrdiff_t  = 0xE043FF88; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0xC0F85007; // 
                constexpr std::ptrdiff_t  = 0xE2900001; // 
                constexpr std::ptrdiff_t  = 0x49A00440; // 
                constexpr std::ptrdiff_t  = 0x5C601380; // 
                constexpr std::ptrdiff_t  = 0x5C601380; // 
                constexpr std::ptrdiff_t  = 0x5C601780; // 
                constexpr std::ptrdiff_t  = 0x5C601380; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x5C601380; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x3868103E; // 
                constexpr std::ptrdiff_t  = 0x5BB68380; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x5CA00500; // 
                constexpr std::ptrdiff_t  = 0xC0F85007; // 
                constexpr std::ptrdiff_t  = 0xC0F85007; // 
                constexpr std::ptrdiff_t  = 0x5CA00500; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x3968103F; // 
                constexpr std::ptrdiff_t  = 0x49A00440; // 
                constexpr std::ptrdiff_t  = 0x32A0053E; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x32A20B3F; // 
                constexpr std::ptrdiff_t  = 0x32A003BF; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x5C5A5000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x5C5A1000; // 
                constexpr std::ptrdiff_t  = 0x5C5B1000; // 
                constexpr std::ptrdiff_t  = 0x51A00D40; // 
                constexpr std::ptrdiff_t  = 0x5B450500; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x4B4B0540; // 
                constexpr std::ptrdiff_t  = 0x3868103F; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0xE2900001; // 
                constexpr std::ptrdiff_t  = 0x5C5B1000; // 
                constexpr std::ptrdiff_t  = 0x3868103F; // 
                constexpr std::ptrdiff_t  = 0x3868103E; // 
                constexpr std::ptrdiff_t  = 0x5C980780; // 
                constexpr std::ptrdiff_t  = 0x5C681000; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x58C68380; // 
                constexpr std::ptrdiff_t  = 0x58C60380; // 
                constexpr std::ptrdiff_t  = 0x5BB98380; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xE2900000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x49A00140; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x5BB68380; // 
                constexpr std::ptrdiff_t  = 0x5C433000; // 
                constexpr std::ptrdiff_t  = 0xE2A00000; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xE2900000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x5BB68380; // 
                constexpr std::ptrdiff_t  = 0x5B5A2000; // 
                constexpr std::ptrdiff_t  = 0x5B640400; // 
                constexpr std::ptrdiff_t  = 0x5B6A2000; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x58C62000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0x5BB98480; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x5C433000; // 
                constexpr std::ptrdiff_t  = 0xE2A00000; // 
                constexpr std::ptrdiff_t  = 0x5BB6A080; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xE2900000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x49A00140; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x58C62080; // 
                constexpr std::ptrdiff_t  = 0x5BB98400; // 
                constexpr std::ptrdiff_t  = 0x5BB6A000; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x58C62000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0x5BB98480; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x5C433000; // 
                constexpr std::ptrdiff_t  = 0xE2A00000; // 
                constexpr std::ptrdiff_t  = 0x5BB6A080; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xE2900000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x49A00140; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x58C62080; // 
                constexpr std::ptrdiff_t  = 0x5BB98400; // 
                constexpr std::ptrdiff_t  = 0x5BB6A000; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x58C62000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0x5BB98480; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x5C433000; // 
                constexpr std::ptrdiff_t  = 0xE2A00000; // 
                constexpr std::ptrdiff_t  = 0x5BB6A080; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xE2900000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x49A00140; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x58C62080; // 
                constexpr std::ptrdiff_t  = 0x5BB98400; // 
                constexpr std::ptrdiff_t  = 0x5BB6A000; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD84C500F; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x58C62000; // 
                constexpr std::ptrdiff_t  = 0xE2400000; // 
                constexpr std::ptrdiff_t  = 0x5BB98480; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0xD850500F; // 
                constexpr std::ptrdiff_t  = 0x49A00140; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x5C433000; // 
                constexpr std::ptrdiff_t  = 0xE2A00000; // 
                constexpr std::ptrdiff_t  = 0x5BB6A080; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0xE2900000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x58C62080; // 
                constexpr std::ptrdiff_t  = 0x5BB98400; // 
                constexpr std::ptrdiff_t  = 0x5BB6A000; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C980780; // 
                constexpr std::ptrdiff_t  = 0x5C980780; // 
                constexpr std::ptrdiff_t  = 0x5C980780; // 
                constexpr std::ptrdiff_t  = 0xE2900000; // 
                constexpr std::ptrdiff_t  = 0xF0F80000; // 
                constexpr std::ptrdiff_t  = 0xE3400000; // 
                constexpr std::ptrdiff_t  = 0x49A00040; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x50900380; // 
                constexpr std::ptrdiff_t  = 0x5C423000; // 
                constexpr std::ptrdiff_t  = 0x50900380; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x5C980780; // 
                constexpr std::ptrdiff_t  = 0x5C980780; // 
                constexpr std::ptrdiff_t  = 0x5C583000; // 
                constexpr std::ptrdiff_t  = 0x5C591000; // 
                constexpr std::ptrdiff_t  = 0x58A10B80; // 
                constexpr std::ptrdiff_t  = 0x58A10B80; // 
                constexpr std::ptrdiff_t  = 0x51A10040; // 
                constexpr std::ptrdiff_t  = 0x5B450500; // 
                constexpr std::ptrdiff_t  = 0x5C581000; // 
                constexpr std::ptrdiff_t  = 0x49A000C0; // 
                constexpr std::ptrdiff_t  = 0x59A10900; // 
                constexpr std::ptrdiff_t  = 0x59A10280; // 
                constexpr std::ptrdiff_t  = 0x5C601780; // 
                constexpr std::ptrdiff_t  = 0x36B383B3; // 
                constexpr std::ptrdiff_t  = 0xE2400FFF; // 
                constexpr std::ptrdiff_t  = 0x50B00000; // 
                constexpr std::ptrdiff_t  = 0x3B23D70A; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x808; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xA48; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xC78; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xDD8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1010; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1190; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1398; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15D0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1730; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1AF8; // 
                constexpr std::ptrdiff_t  = 0x100022; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x8000034; // 
                constexpr std::ptrdiff_t  = 0x7000038; // 
                constexpr std::ptrdiff_t  = 0x100082; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100072; // 
                constexpr std::ptrdiff_t  = 0x41; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x8020803A; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x4002; // 
                constexpr std::ptrdiff_t  = 0xA000032; // 
                constexpr std::ptrdiff_t  = 0x100246; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xCAB1508F; // 
                constexpr std::ptrdiff_t  = 0xDD7F5579; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2FFFF03; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t   = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2FFFF03; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t   = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x20; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2FFFF03; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2FFFF03; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t   = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2FFFF03; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFFFFFFFF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10001; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t   = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2FFFF03; // 
            }
            // Parent: None
            // Field count: 47
            namespace RenderProjectedMaterial_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_SetFloatAttributeToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_ExternalWindForce {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_ModelCull {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderSprites {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_PercentageBetweenTransformLerpCPs {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetPerChildControlPoint {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderTreeShake {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_WorldCollideConstraint {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_SetAttributeToScalarExpression {
            }
            // Parent: particles
            // Field count: 0
            namespace C_OP_CycleScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RenderMaterialProxy {
            }
            // Parent: None
            // Field count: 47
            namespace FloatInputMaterialVariable_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RampScalarLinear {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_RotateVector {
            }
            // Parent: particles
            // Field count: 0
            namespace C_INIT_InitVecCollection {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_RemapParticleCountToNamedModelMeshGroupScalar {
            }
            // Parent: None
            // Field count: 0
            namespace C_INIT_SequenceFromCP {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_CPOffsetToPercentageBetweenCPs {
            }
            // Parent: None
            // Field count: 0
            namespace C_OP_LerpEndCapScalar {
            }
        }
    }
}
