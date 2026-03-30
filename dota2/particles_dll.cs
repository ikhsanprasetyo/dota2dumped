// Generated using https://github.com/ikhsanprasetyo/Source2Dumped
// 2026-03-30 05:23:54.809776700 UTC

namespace CS2Dumper.Schemas {
    // Module: particles.dll
    // Class count: 496
    // Enum count: 76
    public static class ParticlesDll {
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
        // Member count: 7
        public enum Detail2Combo_t : uint {
            DETAIL_2_COMBO_UNINITIALIZED = unchecked((uint)-1),
            DETAIL_2_COMBO_OFF = 0x0,
            DETAIL_2_COMBO_ADD = 0x1,
            DETAIL_2_COMBO_ADD_SELF_ILLUM = 0x2,
            DETAIL_2_COMBO_MOD2X = 0x3,
            DETAIL_2_COMBO_MUL = 0x4,
            DETAIL_2_COMBO_CROSSFADE = 0x5
        }
        // Alignment: 4
        // Member count: 4
        public enum MissingParentInheritBehavior_t : uint {
            MISSING_PARENT_DO_NOTHING = unchecked((uint)-1),
            MISSING_PARENT_KILL = 0x0,
            MISSING_PARENT_FIND_NEW = 0x1,
            MISSING_PARENT_SAME_INDEX = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleTraceMissBehavior_t : uint {
            PARTICLE_TRACE_MISS_BEHAVIOR_NONE = 0x0,
            PARTICLE_TRACE_MISS_BEHAVIOR_KILL = 0x1,
            PARTICLE_TRACE_MISS_BEHAVIOR_TRACE_END = 0x2
        }
        // Alignment: 4
        // Member count: 7
        public enum PFuncVisualizationType_t : uint {
            PFUNC_VISUALIZATION_SPHERE_WIREFRAME = 0x0,
            PFUNC_VISUALIZATION_SPHERE_SOLID = 0x1,
            PFUNC_VISUALIZATION_BOX = 0x2,
            PFUNC_VISUALIZATION_RING = 0x3,
            PFUNC_VISUALIZATION_PLANE = 0x4,
            PFUNC_VISUALIZATION_LINE = 0x5,
            PFUNC_VISUALIZATION_CYLINDER = 0x6
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleVRHandChoiceList_t : uint {
            PARTICLE_VRHAND_LEFT = 0x0,
            PARTICLE_VRHAND_RIGHT = 0x1,
            PARTICLE_VRHAND_CP = 0x2,
            PARTICLE_VRHAND_CP_OBJECT = 0x3
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleReplicationMode_t : uint {
            PARTICLE_REPLICATIONMODE_NONE = 0x0,
            PARTICLE_REPLICATIONMODE_REPLICATE_FOR_EACH_PARENT_PARTICLE = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleEntityPos_t : uint {
            PARTICLE_ABS_ORIGIN = 0x0,
            PARTICLE_WORLDSPACE_CENTER = 0x1,
            PARTICLE_EYES = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleFanType_t : uint {
            PARTICLE_FAN_TYPE_FAN = 0x0,
            PARTICLE_FAN_TYPE_ROTOR_WASH = 0x1,
            PARTICLE_FAN_TYPE_RADIAL = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum PetGroundType_t : uint {
            PET_GROUND_NONE = 0x0,
            PET_GROUND_GRID = 0x1,
            PET_GROUND_PLANE = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum InheritableBoolType_t : uint {
            INHERITABLE_BOOL_INHERIT = 0x0,
            INHERITABLE_BOOL_FALSE = 0x1,
            INHERITABLE_BOOL_TRUE = 0x2
        }
        // Alignment: 4
        // Member count: 6
        public enum ParticlePostProcessPriorityGroup_t : uint {
            PARTICLE_POST_PROCESS_PRIORITY_LEVEL_VOLUME = 0x0,
            PARTICLE_POST_PROCESS_PRIORITY_LEVEL_OVERRIDE = 0x1,
            PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_EFFECT = 0x2,
            PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_LOW = 0x3,
            PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_HIGH = 0x4,
            PARTICLE_POST_PROCESS_PRIORITY_GLOBAL_UI = 0x5
        }
        // Alignment: 4
        // Member count: 7
        public enum ParticleCollisionGroup_t : uint {
            PARTICLE_COLLISION_GROUP_DEFAULT = 0x4,
            PARTICLE_COLLISION_GROUP_DEBRIS = 0x5,
            PARTICLE_COLLISION_GROUP_INTERACTIVE = 0x7,
            PARTICLE_COLLISION_GROUP_PLAYER = 0x8,
            PARTICLE_COLLISION_GROUP_VEHICLE = 0xA,
            PARTICLE_COLLISION_GROUP_NPC = 0xC,
            PARTICLE_COLLISION_GROUP_PROPS = 0x18
        }
        // Alignment: 4
        // Member count: 4
        public enum DetailCombo_t : uint {
            DETAIL_COMBO_OFF = 0x0,
            DETAIL_COMBO_ADD = 0x1,
            DETAIL_COMBO_ADD_SELF_ILLUM = 0x2,
            DETAIL_COMBO_MOD2X = 0x3
        }
        // Alignment: 4
        // Member count: 12
        public enum ScalarExpressionType_t : uint {
            SCALAR_EXPRESSION_UNINITIALIZED = unchecked((uint)-1),
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
        }
        // Alignment: 4
        // Member count: 14
        public enum SpriteCardPerParticleScale_t : uint {
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
        }
        // Alignment: 4
        // Member count: 2
        public enum BlurFilterType_t : uint {
            BLURFILTER_GAUSSIAN = 0x0,
            BLURFILTER_BOX = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum StandardLightingAttenuationStyle_t : uint {
            LIGHT_STYLE_OLD = 0x0,
            LIGHT_STYLE_NEW = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleParentSetMode_t : uint {
            PARTICLE_SET_PARENT_NO = 0x0,
            PARTICLE_SET_PARENT_IMMEDIATE = 0x1,
            PARTICLE_SET_PARENT_ROOT = 0x2
        }
        // Alignment: 4
        // Member count: 6
        public enum ParticleLightingQuality_t : uint {
            PARTICLE_LIGHTING_PER_PARTICLE = 0x0,
            PARTICLE_LIGHTING_PER_VERTEX = 0x1,
            PARTICLE_LIGHTING_PER_PIXEL = unchecked((uint)-1),
            PARTICLE_LIGHTING_OVERRIDE_POSITION = 0x2,
            PARTICLE_LIGHTING_OVERRIDE_COLOR = 0x3,
            PARTICLE_LIGHTING_ADD_EXTRA_LIGHT = 0x4
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleVolumetricSmokeCreationType_t : uint {
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_CONTINUOUS = 0x0,
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_IMPULSE = 0x1
        }
        // Alignment: 4
        // Member count: 8
        public enum SetStatisticExpressionType_t : uint {
            SET_EXPRESSION_UNINITIALIZED = unchecked((uint)-1),
            SET_EXPRESSION_SUM = 0x0,
            SET_EXPRESSION_MEAN = 0x1,
            SET_EXPRESSION_MEDIAN = 0x2,
            SET_EXPRESSION_MODE = 0x3,
            SET_EXPRESSION_STANDARD_DEVIATION = 0x4,
            SET_EXPRESSION_MIN = 0x5,
            SET_EXPRESSION_MAX = 0x6
        }
        // Alignment: 4
        // Member count: 11
        public enum EventTypeSelection_t : uint {
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
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleMassMode_t : uint {
            PARTICLE_MASSMODE_RADIUS_CUBED = 0x0,
            PARTICLE_MASSMODE_RADIUS_SQUARED = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleHitboxBiasType_t : uint {
            PARTICLE_HITBOX_BIAS_ENTITY = 0x0,
            PARTICLE_HITBOX_BIAS_HITBOX = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum ParticleControlPointAxis_t : uint {
            PARTICLE_CP_AXIS_X = 0x0,
            PARTICLE_CP_AXIS_Y = 0x1,
            PARTICLE_CP_AXIS_Z = 0x2,
            PARTICLE_CP_AXIS_NEGATIVE_X = 0x3,
            PARTICLE_CP_AXIS_NEGATIVE_Y = 0x4,
            PARTICLE_CP_AXIS_NEGATIVE_Z = 0x5
        }
        // Alignment: 4
        // Member count: 12
        public enum ParticlePinDistance_t : uint {
            PARTICLE_PIN_DISTANCE_NONE = unchecked((uint)-1),
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
        }
        // Alignment: 4
        // Member count: 7
        public enum VectorFloatExpressionType_t : uint {
            VECTOR_FLOAT_EXPRESSION_UNINITIALIZED = unchecked((uint)-1),
            VECTOR_FLOAT_EXPRESSION_DOTPRODUCT = 0x0,
            VECTOR_FLOAT_EXPRESSION_DISTANCE = 0x1,
            VECTOR_FLOAT_EXPRESSION_DISTANCESQR = 0x2,
            VECTOR_FLOAT_EXPRESSION_INPUT1_LENGTH = 0x3,
            VECTOR_FLOAT_EXPRESSION_INPUT1_LENGTHSQR = 0x4,
            VECTOR_FLOAT_EXPRESSION_INPUT1_NOISE = 0x5
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleFogType_t : uint {
            PARTICLE_FOG_GAME_DEFAULT = 0x0,
            PARTICLE_FOG_ENABLED = 0x1,
            PARTICLE_FOG_DISABLED = 0x2
        }
        // Alignment: 4
        // Member count: 10
        public enum VectorExpressionType_t : uint {
            VECTOR_EXPRESSION_UNINITIALIZED = unchecked((uint)-1),
            VECTOR_EXPRESSION_ADD = 0x0,
            VECTOR_EXPRESSION_SUBTRACT = 0x1,
            VECTOR_EXPRESSION_MUL = 0x2,
            VECTOR_EXPRESSION_DIVIDE = 0x3,
            VECTOR_EXPRESSION_INPUT_1 = 0x4,
            VECTOR_EXPRESSION_MIN = 0x5,
            VECTOR_EXPRESSION_MAX = 0x6,
            VECTOR_EXPRESSION_CROSSPRODUCT = 0x7,
            VECTOR_EXPRESSION_LERP = 0x8
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleMultiSegmentInputSelection_t : uint {
            PARTICLE_MULTISEGMENT_SELECTION_FLOAT = 0x0,
            PARTICLE_MULTISEGMENT_SELECTION_STRING = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleRotationLockType_t : uint {
            PARTICLE_ROTATION_LOCK_NONE = 0x0,
            PARTICLE_ROTATION_LOCK_ROTATIONS = 0x1,
            PARTICLE_ROTATION_LOCK_NORMAL = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum HitboxLerpType_t : uint {
            HITBOX_LERP_LIFETIME = 0x0,
            HITBOX_LERP_CONSTANT = 0x1
        }
        // Alignment: 4
        // Member count: 7
        public enum ParticleAttrBoxFlags_t : uint {
            PARTICLE_ATTR_BOX_FLAGS_NONE = 0x0,
            PARTICLE_ATTR_BOX_FLAGS_WATER = 0x1,
            PARTICLE_ATTR_BOX_FLAGS_ON_FIRE = 0x2,
            PARTICLE_ATTR_BOX_FLAGS_ELECTRIFIED = 0x4,
            PARTICLE_ATTR_BOX_FLAGS_ASLEEP = 0x8,
            PARTICLE_ATTR_BOX_FLAGS_FROZEN = 0x10,
            PARTICLE_ATTR_BOX_FLAGS_TIMED_DECAY = 0x20
        }
        // Alignment: 4
        // Member count: 5
        public enum ParticleTopology_t : uint {
            PARTICLE_TOPOLOGY_POINTS = 0x0,
            PARTICLE_TOPOLOGY_LINES = 0x1,
            PARTICLE_TOPOLOGY_TRIS = 0x2,
            PARTICLE_TOPOLOGY_QUADS = 0x3,
            PARTICLE_TOPOLOGY_CUBES = 0x4
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleLightBehaviorChoiceList_t : uint {
            PARTICLE_LIGHT_BEHAVIOR_FOLLOW_DIRECTION = 0x0,
            PARTICLE_LIGHT_BEHAVIOR_ROPE = 0x1,
            PARTICLE_LIGHT_BEHAVIOR_TRAILS = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum ModelHitboxType_t : uint {
            MODEL_HITBOX_TYPE_STANDARD = 0x0,
            MODEL_HITBOX_TYPE_RAW_BONES = 0x1,
            MODEL_HITBOX_TYPE_RENDERBOUNDS = 0x2,
            MODEL_HITBOX_TYPE_SNAPSHOT = 0x3
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleMultiSegmentCountSelection_t : uint {
            PARTICLE_MULTISEGMENT_SEG_COUNT_7 = 0x7,
            PARTICLE_MULTISEGMENT_SEG_COUNT_14 = 0xE,
            PARTICLE_MULTISEGMENT_SEG_COUNT_16 = 0x10
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleOrientationType_t : uint {
            PARTICLE_ORIENTATION_NONE = 0x0,
            PARTICLE_ORIENTATION_VELOCITY = 0x1,
            PARTICLE_ORIENTATION_NORMAL = 0x2,
            PARTICLE_ORIENTATION_ROTATION = 0x4
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleTraceSet_t : uint {
            PARTICLE_TRACE_SET_ALL = 0x0,
            PARTICLE_TRACE_SET_STATIC = 0x1,
            PARTICLE_TRACE_SET_STATIC_AND_KEYFRAMED = 0x2,
            PARTICLE_TRACE_SET_DYNAMIC = 0x3
        }
        // Alignment: 4
        // Member count: 7
        public enum ParticleTextureLayerBlendType_t : uint {
            SPRITECARD_TEXTURE_BLEND_MULTIPLY = 0x0,
            SPRITECARD_TEXTURE_BLEND_MOD2X = 0x1,
            SPRITECARD_TEXTURE_BLEND_REPLACE = 0x2,
            SPRITECARD_TEXTURE_BLEND_ADD = 0x3,
            SPRITECARD_TEXTURE_BLEND_SUBTRACT = 0x4,
            SPRITECARD_TEXTURE_BLEND_AVERAGE = 0x5,
            SPRITECARD_TEXTURE_BLEND_LUMINANCE = 0x6
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleSelection_t : uint {
            PARTICLE_SELECTION_FIRST = 0x0,
            PARTICLE_SELECTION_LAST = 0x1,
            PARTICLE_SELECTION_NUMBER = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleToolsState_t : uint {
            PARTICLE_TOOLS_STATE_ALWAYS_ON = unchecked((uint)-1),
            PARTICLE_TOOLS_STATE_TOOLS_ONLY = 0x0,
            PARTICLE_TOOLS_STATE_GAME_ONLY = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum SnapshotIndexType_t : uint {
            SNAPSHOT_INDEX_INCREMENT = 0x0,
            SNAPSHOT_INDEX_DIRECT = 0x1
        }
        // Alignment: 4
        // Member count: 7
        public enum ParticleOutputBlendMode_t : uint {
            PARTICLE_OUTPUT_BLEND_MODE_ALPHA = 0x0,
            PARTICLE_OUTPUT_BLEND_MODE_ADD = 0x1,
            PARTICLE_OUTPUT_BLEND_MODE_BLEND_ADD = 0x2,
            PARTICLE_OUTPUT_BLEND_MODE_HALF_BLEND_ADD = 0x3,
            PARTICLE_OUTPUT_BLEND_MODE_NEG_HALF_BLEND_ADD = 0x4,
            PARTICLE_OUTPUT_BLEND_MODE_MOD2X = 0x5,
            PARTICLE_OUTPUT_BLEND_MODE_LIGHTEN = 0x6
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleLightnintBranchBehavior_t : uint {
            PARTICLE_LIGHTNING_BRANCH_CURRENT_DIR = 0x0,
            PARTICLE_LIGHTNING_BRANCH_ENDPOINT_DIR = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum MaterialProxyType_t : uint {
            MATERIAL_PROXY_STATUS_EFFECT = 0x0,
            MATERIAL_PROXY_TINT = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleDepthFeatheringMode_t : uint {
            PARTICLE_DEPTH_FEATHERING_OFF = 0x0,
            PARTICLE_DEPTH_FEATHERING_ON_OPTIONAL = 0x1,
            PARTICLE_DEPTH_FEATHERING_ON_REQUIRED = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleLightUnitChoiceList_t : uint {
            PARTICLE_LIGHT_UNIT_CANDELAS = 0x0,
            PARTICLE_LIGHT_UNIT_LUMENS = 0x1
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleMultiSegmentSpecialCharacter_t : uint {
            PARTICLE_MULTISEGMENT_SPECIAL_NONE = unchecked((uint)-1),
            PARTICLE_MULTISEGMENT_SPECIAL_DECIMAL = 0x0,
            PARTICLE_MULTISEGMENT_SPECIAL_COLON = 0x1,
            PARTICLE_MULTISEGMENT_SPECIAL_DEGREES = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleFalloffFunction_t : uint {
            PARTICLE_FALLOFF_CONSTANT = 0x0,
            PARTICLE_FALLOFF_LINEAR = 0x1,
            PARTICLE_FALLOFF_EXPONENTIAL = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleSequenceCropOverride_t : uint {
            PARTICLE_SEQUENCE_CROP_OVERRIDE_DEFAULT = unchecked((uint)-1),
            PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_OFF = 0x0,
            PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_ON = 0x1
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleDetailLevel_t : uint {
            PARTICLEDETAIL_LOW = 0x0,
            PARTICLEDETAIL_MEDIUM = 0x1,
            PARTICLEDETAIL_HIGH = 0x2,
            PARTICLEDETAIL_ULTRA = 0x3
        }
        // Alignment: 4
        // Member count: 4
        public enum BBoxVolumeType_t : uint {
            BBOX_VOLUME = 0x0,
            BBOX_DIMENSIONS = 0x1,
            BBOX_MINS_MAXS = 0x2,
            BBOX_RADIUS = 0x3
        }
        // Alignment: 4
        // Member count: 12
        public enum SpriteCardTextureType_t : uint {
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
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleAlphaReferenceType_t : uint {
            PARTICLE_ALPHA_REFERENCE_ALPHA_ALPHA = 0x0,
            PARTICLE_ALPHA_REFERENCE_OPAQUE_ALPHA = 0x1,
            PARTICLE_ALPHA_REFERENCE_ALPHA_OPAQUE = 0x2,
            PARTICLE_ALPHA_REFERENCE_OPAQUE_OPAQUE = 0x3
        }
        // Alignment: 4
        // Member count: 15
        public enum SpriteCardTextureChannel_t : uint {
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
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleVolumetricSmokeType_t : uint {
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_EMISSION = 0x0,
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_SINK = 0x1,
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_REPEL = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum RenderModelSubModelFieldType_t : uint {
            SUBMODEL_AS_BODYGROUP_SUBMODEL = 0x0,
            SUBMODEL_AS_MESHGROUP_INDEX = 0x1,
            SUBMODEL_AS_MESHGROUP_MASK = 0x2,
            SUBMODEL_IGNORED_USE_MODEL_DEFAULT_MESHGROUP_MASK = 0x3
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleHitboxDataSelection_t : uint {
            PARTICLE_HITBOX_AVERAGE_SPEED = 0x0,
            PARTICLE_HITBOX_COUNT = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum ParticleOrientationChoiceList_t : uint {
            PARTICLE_ORIENTATION_SCREEN_ALIGNED = 0x0,
            PARTICLE_ORIENTATION_SCREEN_Z_ALIGNED = 0x1,
            PARTICLE_ORIENTATION_WORLD_Z_ALIGNED = 0x2,
            PARTICLE_ORIENTATION_ALIGN_TO_PARTICLE_NORMAL = 0x3,
            PARTICLE_ORIENTATION_SCREENALIGN_TO_PARTICLE_NORMAL = 0x4,
            PARTICLE_ORIENTATION_FULL_3AXIS_ROTATION = 0x5
        }
        // Alignment: 4
        // Member count: 5
        public enum ParticleCollisionMode_t : uint {
            COLLISION_MODE_PER_PARTICLE_TRACE = 0x3,
            COLLISION_MODE_USE_NEAREST_TRACE = 0x2,
            COLLISION_MODE_PER_FRAME_PLANESET = 0x1,
            COLLISION_MODE_INITIAL_TRACE_DOWN = 0x0,
            COLLISION_MODE_DISABLED = unchecked((uint)-1)
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleSortingChoiceList_t : uint {
            PARTICLE_SORTING_NEAREST = 0x0,
            PARTICLE_SORTING_CREATION_TIME = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleEndcapMode_t : uint {
            PARTICLE_ENDCAP_ALWAYS_ON = unchecked((uint)-1),
            PARTICLE_ENDCAP_ENDCAP_OFF = 0x0,
            PARTICLE_ENDCAP_ENDCAP_ON = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum ClosestPointTestType_t : uint {
            PARTICLE_CLOSEST_TYPE_BOX = 0x0,
            PARTICLE_CLOSEST_TYPE_CAPSULE = 0x1,
            PARTICLE_CLOSEST_TYPE_HYBRID = 0x2
        }
        // Alignment: 4
        // Member count: 6
        public enum ParticleImpulseType_t : uint {
            IMPULSE_TYPE_NONE = 0x0,
            IMPULSE_TYPE_GENERIC = 0x1,
            IMPULSE_TYPE_ROPE = 0x2,
            IMPULSE_TYPE_EXPLOSION = 0x4,
            IMPULSE_TYPE_EXPLOSION_UNDERWATER = 0x8,
            IMPULSE_TYPE_PARTICLE_SYSTEM = 0x10
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleLiquidContents_t : uint {
            PARTICLE_LIQUID_NONE = 0x0,
            PARTICLE_LIQUID_OIL = 0x1,
            PARTICLE_LIQUID_WATER = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum SpriteCardShaderType_t : uint {
            SPRITECARD_SHADER_BASE = 0x0,
            SPRITECARD_SHADER_CUSTOM = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum ParticleOmni2LightTypeChoiceList_t : uint {
            PARTICLE_OMNI2_LIGHT_TYPE_POINT = 0x0,
            PARTICLE_OMNI2_LIGHT_TYPE_SPHERE = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleLightFogLightingMode_t : uint {
            PARTICLE_LIGHT_FOG_LIGHTING_MODE_NONE = 0x0,
            PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC = 0x2,
            PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC_NOSHADOWS = 0x4
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleLightTypeChoiceList_t : uint {
            PARTICLE_LIGHT_TYPE_POINT = 0x0,
            PARTICLE_LIGHT_TYPE_SPOT = 0x1,
            PARTICLE_LIGHT_TYPE_FX = 0x2,
            PARTICLE_LIGHT_TYPE_CAPSULE = 0x3
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleOrientationSetMode_t : uint {
            PARTICLE_ORIENTATION_SET_NONE = unchecked((uint)-1),
            PARTICLE_ORIENTATION_SET_FROM_VELOCITY = 0x0,
            PARTICLE_ORIENTATION_SET_FROM_NORMAL = 0x1,
            PARTICLE_ORIENTATION_SET_FROM_ROTATIONS = 0x2
        }
        // Alignment: 8
        // Member count: 10
        public enum ParticleCollisionMask_t : ulong {
            PARTICLE_MASK_ALL = unchecked((ulong)-1),
            PARTICLE_MASK_SOLID = 0xC3001,
            PARTICLE_MASK_WATER = 0x18000,
            PARTICLE_MASK_SOLID_WATER = 0xDB001,
            PARTICLE_MASK_SHOT = 0x1C1003,
            PARTICLE_MASK_SHOT_BRUSHONLY = 0x101001,
            PARTICLE_MASK_SHOT_HULL = 0x1C3001,
            PARTICLE_MASK_OPAQUE = 0x80,
            PARTICLE_MASK_DEFAULTPLAYERSOLID = 0xC3011,
            PARTICLE_MASK_NPCSOLID = 0xC3021
        }
        // Alignment: 4
        // Member count: 2
        public enum TextureRepetitionMode_t : uint {
            TEXTURE_REPETITION_PARTICLE = 0x0,
            TEXTURE_REPETITION_PATH = 0x1
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_WaitForCursorsWithTag {
        }
        // Parent: None
        // Field count: 47
        public static class CPulseCell_Base {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 942
        public static class CPulse_ResumePoint {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_PickBestOutflowSelector {
        }
        // Parent: None
        // Field count: 0
        public static class CParticleBindingRealPulse {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_WaitForObservable {
        }
        // Parent: None
        // Field count: 45
        public static class CPulse_OutflowConnection {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 47
        public static class CPulseGraphDef {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_FireCursors {
        }
        // Parent: None
        // Field count: 47
        public static class CPulseCell_Timeline__TimelineEvent_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 49
        public static class CPulseCell_IntervalTimer__CursorState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_BaseState {
        }
        // Parent: None
        // Field count: 47
        public static class OutflowWithRequirements_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_IsRequirementValid {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_Gradient {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCursorFuncs {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 47
        public static class PulseNodeDynamicOutflows_t__DynamicOutflow_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class CBasePulseGraphInstance {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_GraphHook {
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Resume {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_BaseEntrypoint {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_WaitForCursorsWithTagBase {
        }
        // Parent: None
        // Field count: 47
        public static class CPulse_InvokeBinding {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_IntervalTimer {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseTestScriptLib {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_BaseLerp {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_Curve {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_EventHandler {
        }
        // Parent: None
        // Field count: 47
        public static class CPulseCell_Outflow_CycleShuffled__InstanceState_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 49
        public static class CPulseCell_BaseLerp__CursorState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCell_WaitForCursorsWithTagBase__CursorState_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class CPulseArraylib {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 942
        public static class SignatureOutflow_Continue {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Timeline {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_EntOutputHandler {
        }
        // Parent: None
        // Field count: 49
        public static class CPulseCell_Outflow_CycleOrdered__InstanceState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 942
        public static class CParticleCollectionBindingInstance {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 49
        public static class CPulseCell_LimitCount__InstanceState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Step_DebugLog {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_BaseYieldingInflow {
        }
        // Parent: None
        // Field count: 47
        public static class PulseNodeDynamicOutflows_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCell_IsRequirementValid__Criteria_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_ObservableVariableListener {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Outflow_CycleOrdered {
        }
        // Parent: None
        // Field count: 47
        public static class PulseSelectorOutflowList_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_Wait {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Outflow_CycleShuffled {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_Method {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_BooleanSwitchState {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_Yield {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseMathlib {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Outflow_CycleRandom {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Step_PublicOutput {
        }
        // Parent: None
        // Field count: 47
        public static class CPulse_BlackboardReference {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_RandomInt {
        }
        // Parent: None
        // Field count: 47
        public static class CPulse_CallInfo {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_InlineNodeSkipSelector {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_LimitCount {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Step_CallExternalMethod {
        }
        // Parent: None
        // Field count: 47
        public static class PulseObservableBoolExpression_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCell_LimitCount__Criteria_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_CursorQueue {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_RandomFloat {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseExecCursor {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class IParticleCollection {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class ParticleAttributeIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapGravityToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_Decay {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderDeferredLight {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapSpeedtoCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapTransformToVelocity {
        }
        // Parent: None
        // Field count: 49
        public static class CollisionGroupContext_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: particles
        // Field count: 0
        public static class CParticleFunctionPreEmission {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_FadeOutSimple {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SpringToVectorConstraint {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderRopes {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_StatusEffectCitadel {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderSound {
        }
        // Parent: None
        // Field count: 49
        public static class CParticleVisibilityInputs {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointsToParticle {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapCPVelocityToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PointVectorAtNextParticle {
        }
        // Parent: None
        // Field count: 47
        public static class ParticlePreviewBodyGroup_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_OscillateScalarSimple {
        }
        // Parent: ______
        // Field count: 0
        public static class C_INIT_StatusEffect {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RtEnvCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ConstrainDistance {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitialVelocityNoise {
        }
        // Parent: None
        // Field count: 54
        public static class ParticleChildrenInfo_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10740; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100000; // 
            public const nint  = 0x88; // 
            public const nint � = 0x1F; // �
            public const nint  = 0x10018; // 
            public const nint  = 0x0; // 
            public const nint P = 0x102C0; // P
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0xBC2B8EA0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint P = 0x4; // P
            public const nint  = 0x14C3D0; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x65006C; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x750064; // 
            public const nint  = 0x43002E; // 
            public const nint  = 0x69004C; // 
            public const nint  = 0x5C0073; // 
            public const nint  = 0x650062; // 
            public const nint  = 0x36006E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x50; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10150; // 
            public const nint P = 0x310038; // P
            public const nint  = 0x4E002E; // 
            public const nint  = 0x55002D; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x10150; // 
            public const nint  = 0x640064; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20009; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5467122E; // 
            public const nint  = 0x10150; // 
            public const nint � = 0x10060; // �
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapScalarOnceTimed {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomNamedModelSequence {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PlaneCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_VelocityRandom {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ModelDampenMovement {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_TwistAroundAxis {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_TeleportBeam {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RemapExternalWindToCP {
        }
        // Parent: None
        // Field count: 0
        public static class CBaseRendererSource2 {
        }
        // Parent: None
        // Field count: 0
        public static class CSpinUpdateBase {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_OrientTo2dDirection {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RemapDotProductToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapParticleCountToNamedModelElementScalar {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RenderTrails {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointPositionToTimeOfDayValue {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DecayMaintainCount {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomModelSequence {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ExternalGameImpulseForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapAverageHitboxSpeedtoCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomAlpha {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_NormalizeVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_FadeInSimple {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RepeatedTriggerChildGroup {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapVelocityToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_SetHitboxToClosest {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RingWave {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomTrailLength {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DistanceBetweenTransforms {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DecayOffscreen {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateSequentialPath {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_EndCapTimedDecay {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RemapDistanceToLineSegmentBase {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ContinuousEmitter {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_OscillateVectorSimple {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_SequenceLifeTime {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_MoveBetweenPoints {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetUserEvent {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_QuantizeFloat {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_BasicMovement {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomNamedModelElement {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitFromParentKilled {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_Callback {
        }
        // Parent: None
        // Field count: 45
        public static class CParticleFunction {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_GlobalLight {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_OffsetVectorToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetPerChildControlPointFromAttribute {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetParentControlPointsToChildCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_BoxConstraint {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreatePhyllotaxis {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_AttractToControlPoint {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomLifeTime {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapParticleCountToNamedModelSequenceScalar {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_VelocityRadialRandom {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomRadius {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_Orient2DRelToCP {
        }
        // Parent: None
        // Field count: 47
        public static class TextureControls_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 49
        public static class ControlPointReference_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetControlPointToVectorExpression {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LightningSnapshotGenerator {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RemapNamedModelMeshGroupOnceTimed {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapQAnglesToRotation {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_PositionWarp {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointFieldToScalarExpression {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_CreateParticleSystemRenderer {
        }
        // Parent: None
        // Field count: 942
        public static class CParticleFunctionForce {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomVectorComponent {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_InheritFromParentParticles {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_SetVectorAttributeToVectorExpression {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RemapTransformVisibilityToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DirectionBetweenVecsToVec {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MovementLoopInsideSphere {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderSimpleModelCollection {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_QuantizeCPComponent {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_PlayEndCapWhenFinished {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitFloatCollection {
        }
        // Parent: None
        // Field count: 54
        public static class CPathParameters {
            public const nint  = 0x10120; // 
            public const nint  = 0x10740; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100000; // 
            public const nint  = 0x88; // 
            public const nint � = 0x1F; // �
            public const nint  = 0x10018; // 
            public const nint  = 0x0; // 
            public const nint P = 0x102C0; // P
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0xBC2B8EA0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint P = 0x4; // P
            public const nint  = 0x14C3D0; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x65006C; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x750064; // 
            public const nint  = 0x43002E; // 
            public const nint  = 0x69004C; // 
            public const nint  = 0x5C0073; // 
            public const nint  = 0x650062; // 
            public const nint  = 0x36006E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x50; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10150; // 
            public const nint P = 0x310038; // P
            public const nint  = 0x4E002E; // 
            public const nint  = 0x55002D; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x10150; // 
            public const nint  = 0x640064; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20009; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5467122E; // 
            public const nint  = 0x10150; // 
            public const nint � = 0x10060; // �
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapScalarEndCap {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_CreateFromPlaneCache {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LazyCullCompareFloat {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ControlPointToRadialScreenSpace {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SpinUpdate {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_NormalOffset {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RemapDistanceToLineSegmentToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderAsModels {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreationNoise {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_Spin {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_GameLiquidSpill {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_InstantaneousEmitter {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ConstrainLineLength {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_LifespanFromVelocity {
        }
        // Parent: None
        // Field count: 0
        public static class CBaseTrailRenderer {
        }
        // Parent: ______
        // Field count: 0
        public static class C_INIT_VelocityFromCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointOrientation {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MovementSkinnedPositionFromCPSnapshot {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MultiSegmentDisplaySnapshotGenerator {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_OscillateVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PositionLock {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderVRHapticEvent {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetControlPointToImpactPoint {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_InterpolateRadius {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ReinitializeScalarEndCap {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_TurbulenceForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapNamedModelElementOnceTimed {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetControlPointToPlayer {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_EndCapTimedFreeze {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RenderGpuImplicit {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetRandomControlPointPosition {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderVolumetricEmitter {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapTransformVisibilityToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapControlPointDirectionToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ScreenSpacePositionOfTarget {
        }
        // Parent: particles
        // Field count: 942
        public static class CParticleFunctionOperator {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DragRelativeToPlane {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetCPtoVector {
        }
        // Parent: ______
        // Field count: 0
        public static class C_INIT_RandomYaw {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SnapshotRigidSkinToBones {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetSingleControlPointPosition {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_DistanceToNeighborCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapCPtoScalar {
        }
        // Parent: None
        // Field count: 942
        public static class CParticleFunctionRenderer {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapNamedModelMeshGroupEndCap {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PercentageBetweenTransformsVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderScreenVelocityRotate {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_UpdateLightSource {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateWithinBox {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ChooseRandomChildrenInGroup {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ControlpointLight {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_VectorFieldSnapshot {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_CylindricalDistanceToTransform {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_PositionPlaceOnGround {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderPostProcessing {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_WorldTraceConstraint {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderBlobs {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_OscillateScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_FadeOut {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_WaterImpulseRenderer {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomSequence {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RampScalarSplineSimple {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_DistanceCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_CollideWithParentParticles {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitFromVectorFieldSnapshot {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetVectorAttributeToVectorExpression {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_AddVectorToVector {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_RemapInitialVisibilityScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapTransformOrientationToYaw {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderStatusEffect {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RandomForce {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RemapParticleCountOnScalarEndCap {
        }
        // Parent: None
        // Field count: 47
        public static class ParticlePreviewState_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LocalAccelerationForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ModelCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetFloat {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapTransformToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ScreenSpaceDistanceToEdge {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapDistanceToLineSegmentToScalar {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RemapVectortoCP {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetFromCPSnapshot {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DistanceBetweenCPsToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointToHand {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ConstrainDistanceToPath {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DistanceCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateAlongPath {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_GameDecalRenderer {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_SetControlPointsToModelParticles {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ColorInterpolateRandom {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapNamedModelSequenceToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderLights {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DecayClampCount {
        }
        // Parent: None
        // Field count: 49
        public static class CRandomNumberGeneratorParameters {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_ColorLitPerParticle {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderPoints {
        }
        // Parent: ______
        // Field count: 0
        public static class C_INIT_SetAttributeToScalarExpression {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateOnGrid {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RampCPLinearRandom {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_VelocityMatchingForce {
        }
        // Parent: ______
        // Field count: 0
        public static class C_INIT_RandomAlphaWindowThreshold {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateOnModelAtHeight {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_ModelSurfaceSnapshotGenerator {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RestartAfterDuration {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderClothForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapVisibilityScalar {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_CreateSequentialPathV2 {
        }
        // Parent: None
        // Field count: 47
        public static class VecInputMaterialVariable_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapInitialDirectionToTransformToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LockToSavedSequentialPathV2 {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_NormalLock {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapTransformOrientationToRotations {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_Cull {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomYawFlip {
        }
        // Parent: None
        // Field count: 49
        public static class SequenceWeightedList_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_ReadFromNeighboringParticle {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderText {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_LerpToInitialPosition {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomRotation {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LerpEndCapVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_VelocityDecay {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetCPOrientationToPointAtCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LockToPointList {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MovementPlaceOnGround {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetCPOrientationToDirection {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapCrossProductOfTwoVectorsToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapTransformOrientationToRotations {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomRotationSpeed {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_InheritFromParentParticlesV2 {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomSecondSequence {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetFloatCollection {
        }
        // Parent: None
        // Field count: 49
        public static class PointDefinition_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x720061; // 
            public const nint  = 0x6D006D; // 
            public const nint  = 0x67005C; // 
            public const nint  = 0x69006C; // 
            public const nint  = 0x64002E; // 
            public const nint  = 0x53005C; // 
            public const nint  = 0x610065; // 
            public const nint  = 0x74006F; // 
            public const nint  = 0x690062; // 
            public const nint  = 0x53; // 
            public const nint l = 0xBC16BD86; // 
            public const nint  = 0x0; // ����3�H��$�
            public const nint  = 0x10000A0; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointPositionToRandomActiveCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_Diffusion {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_AgeNoise {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapVectorComponentToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class CGeneralRandomRotation {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DistanceBetweenVecs {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DampenToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_CalculateVectorAttribute {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LockToBone {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapNamedModelBodyPartOnceTimed {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ScreenSpaceRotateTowardTarget {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_MovementMaintainOffset {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateWithinCapsuleTransform {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetVec {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateFromParentParticles {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CheckParticleForWater {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomNamedModelBodyPart {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderOmni2Light {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ConnectParentParticleToNearest {
        }
        // Parent: None
        // Field count: 47
        public static class CPAssignment_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapParticleCountToNamedModelBodyPartScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitSkinnedPositionFromCPSnapshot {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_LagCompensation {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_CollideWithSelf {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_Noise {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_FadeAndKillForTracers {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ColorAdjustHSL {
        }
        // Parent: None
        // Field count: 47
        public static class CParticleMassCalculationParameters {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SequenceFromModel {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_AlphaDecay {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapDensityGradientToVectorAttribute {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitVec {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_SetHitboxToModel {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MovementMoveAlongSkinnedCPSnapshot {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LerpScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitialRepulsionVelocity {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ClampScalar {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetControlPointToHMD {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DifferencePreviousParticle {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointFieldFromVectorExpression {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_PercentageBetweenTransforms {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_PlaneCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapNamedModelSequenceEndCap {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitFromCPSnapshot {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderCables {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InheritVelocity {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointToWaterSurface {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_PositionOffset {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_NormalAlignToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ShapeMatchingConstraint {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetChildControlPoints {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ChladniWave {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapDirectionToCPToVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DriveCPFromGlobalSoundFloat {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_ScreenSpacePositionOfTarget {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RtEnvCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PinParticleToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapCPtoVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateParticleImpulse {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_DensityForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateInEpitrochoid {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ConstrainDistanceToUserSpecifiedPath {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetControlPointPositions {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetFloatAttributeToVectorExpression {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MovementRotateParticleAroundAxis {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_IntraParticleForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitFloat {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateOnModel {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_InheritFromPeerSystem {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PerParticleForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomNamedModelMeshGroup {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderProjected {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_MaxVelocity {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_VelocityFromNormal {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MaintainEmitter {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_PositionOffsetToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapInitialTransformDirectionToRotation {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_FadeAndKill {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ColorInterpolate {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RampScalarSpline {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapNamedModelSequenceOnceTimed {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_SetControlPointFromObjectScale {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MaintainSequentialPath {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapNamedModelBodyPartEndCap {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_StopAfterCPDuration {
        }
        // Parent: None
        // Field count: 0
        public static class CGeneralSpin {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LockToSavedSequentialPath {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapNamedModelElementToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ClampVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderStatusEffectCitadel {
        }
        // Parent: None
        // Field count: 45
        public static class IParticleSystemDefinition {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x72006100; // 
            public const nint  = 0x6D006D00; // 
            public const nint  = 0x67005C00; // 
            public const nint  = 0x69006C00; // 
            public const nint  = 0x64002E00; // 
            public const nint  = 0x72006200; // 
            public const nint  = 0x6F006300; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x5C003400; // 
            public const nint  = 0x30002D00; // 
            public const nint  = 0x3A004400; // 
            public const nint  = 0x74007300; // 
            public const nint  = 0x64005C00; // 
            public const nint  = 0x5C006500; // 
            public const nint  = 0x55002D00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_WindForce {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_SetVariable {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderStandardLight {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_DistanceToTransform {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RemapControlPointOrientationToRotation {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointToCenter {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapAverageScalarValuetoCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapDotProductToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapCPtoCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointRotation {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_CurlNoiseForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_Orient2DRelToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetSimulationRate {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_FadeIn {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderScreenShake {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapBoundingVolumetoCP {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_HSVShiftToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapVectorToRotations {
        }
        // Parent: ______
        // Field count: 0
        public static class C_INIT_GlobalScale {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_RadiusFromCPObject {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InitialVelocityFromHitbox {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LerpVector {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointFieldToWater {
        }
        // Parent: None
        // Field count: 47
        public static class TextureGroup_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_TimeVaryingForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetCPOrientationToGroundNormal {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SnapshotSkinToBones {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateWithinSphereTransform {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RadiusDecay {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapNamedModelBodyPartToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapScalarToVector {
        }
        // Parent: ______
        // Field count: 0
        public static class C_INIT_InitialSequenceFromModel {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_NoiseEmitter {
        }
        // Parent: None
        // Field count: 942
        public static class CParticleFunctionInitializer {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SelectivelyEnableChildren {
        }
        // Parent: None
        // Field count: 47
        public static class ModelReference_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PlanarConstraint {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_CreateFromCPs {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LockPoints {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_CreateSpiralSphere {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_CPVelocityForce {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RemapNamedModelElementEndCap {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_ScaleVelocity {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_MoveToHitbox {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PinRopeSegmentParticleToParent {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_PointList {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LerpToOtherAttribute {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RandomColor {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetGravityToCP {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_RemapParticleCountToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_InheritFromParentParticles {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RampScalarLinearSimple {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_ChaoticAttractor {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_MovementRigidAttachToCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderFlattenGrass {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderLightBeam {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_EnableChildrenFromParentParticleCount {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_DistanceToCPInit {
        }
        // Parent: None
        // Field count: 47
        public static class CReplicationParameters {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_EndCapDecay {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ForceBasedOnDistanceToPlane {
        }
        // Parent: ______
        // Field count: 0
        public static class C_OP_RemapDensityToVector {
        }
        // Parent: None
        // Field count: 47
        public static class ParticleControlPointConfiguration_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_SetRigidAttachment {
        }
        // Parent: None
        // Field count: 47
        public static class MaterialVariable_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 942
        public static class CParticleFunctionConstraint {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapSpeed {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderModels {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderClientPhysicsImpulse {
        }
        // Parent: None
        // Field count: 942
        public static class CParticleFunctionEmitter {
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1B00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x700; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0xF00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x400100; // 
            public const nint  = 0xCCCCCC00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80000040; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x1F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0xB00; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10007208; // 
            public const nint  = 0x1000D20B; // 
            public const nint  = 0x100E0600; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x500; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x870A3D3F; // 
            public const nint  = 0x1000D20A; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x59999A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0xA00; // 
            public const nint  = 0xA00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x900; // 
            public const nint  = 0x12FF; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4EB85200; // 
            public const nint  = 0x300; // 
            public const nint  = 0xD555553E; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1000C20A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10010600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001F3F; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x2001; // 
            public const nint  = 0x10000A03; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10002208; // 
            public const nint  = 0x10002209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000C20F; // 
            public const nint 
             = 0x700000C0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10001A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x900; // 
            public const nint  = 0x107B4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x2F41; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003205; // 
            public const nint  = 0x900; // 
            public const nint  = 0x6147AEBD; // 
            public const nint  = 0x3B; // 
            public const nint  = 0x10080600; // 
            public const nint  = 0x400; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x10001207; // 
            public const nint  = 0x400; // 
            public const nint  = 0x4001F00; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10001208; // 
            public const nint  = 0x1000120A; // 
            public const nint  = 0x800; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x601; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x1000720C; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x23F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x32BF; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x200; // 
            public const nint  = 0x300; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x363F; // 
            public const nint  = 0x100AA600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10075600; // 
            public const nint  = 0x10084600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x10055600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x300; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x1000F20B; // 
            public const nint  = 0x8000003F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x1000F20A; // 
            public const nint 
             = 0x3F; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000F205; // 
            public const nint  = 0x1; // 
            public const nint  = 0x800000BF; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400; // 
            public const nint  = 0x700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0xE3F; // 
            public const nint  = 0x1000520F; // 
            public const nint  = 0xBF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000320C; // 
            public const nint  = 0x3F; // 
            public const nint  = 0x1800; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x500; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x800; // 
            public const nint  = 0x107E4600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x800; // 
            public const nint  = 0x400; // 
            public const nint  = 0x10004208; // 
            public const nint  = 0x10004207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x3601; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10090600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x900; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000820A; // 
            public const nint  = 0x3D; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x10003A03; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x1500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0xD0955600; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x600; // 
            public const nint  = 0x1900; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x6147AE00; // 
            public const nint  = 0x4D2E1C00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1000520A; // 
            public const nint 
             = 0xBF; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x1000520A; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10008600; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x10008208; // 
            public const nint  = 0x10008205; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x40; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5600; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10004205; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x800; // 
            public const nint  = 0x4001F01; // 
            public const nint  = 0x4500; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10101A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x500; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1DBD; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x10004600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10600000; // 
            public const nint  = 0x1000F209; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x3700; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000E208; // 
            public const nint  = 0x10007209; // 
            public const nint  = 0x3600; // 
            public const nint  = 0x500; // 
            public const nint  = 0x1645A23E; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x300; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x300; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10007205; // 
            public const nint  = 0x2000; // 
            public const nint  = 0x10002207; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10040600; // 
            public const nint  = 0x100E8600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x300; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x1000320A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400103; // 
            public const nint  = 0x400; // 
            public const nint  = 0x323F; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x601; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x80000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x200; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x0; // 
            public const nint  = 0x600; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x200; // 
            public const nint  = 0x601; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x93A92A3F; // 
            public const nint  = 0x300; // 
            public const nint  = 0x100E4600; // 
            public const nint  = 0x10000600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x1000720A; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20824600; // 
            public const nint  = 0x400; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10000A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10103A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1700; // 
            public const nint  = 0x20801A00; // 
            public const nint  = 0x10002A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x20802A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400200; // 
            public const nint  = 0x1000E20A; // 
            public const nint  = 0x1A00; // 
            public const nint  = 0x200; // 
            public const nint  = 0x4B00; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x10004209; // 
            public const nint  = 0x3200; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x200; // 
            public const nint  = 0x10003A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x100FF600; // 
            public const nint  = 0x100; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x75564E10; // 
            public const nint  = 0x2C00; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF7FF0001; // 
            public const nint  = 0x67FF15E0; // 
            public const nint  = 0x7F01201; // 
            public const nint  = 0x671404C0; // 
            public const nint  = 0x800000D8; // 
            public const nint  = 0x7091C49; // 
            public const nint  = 0xC7191F49; // 
            public const nint  = 0x671F1F5C; // 
            public const nint  = 0x6717175C; // 
            public const nint  = 0x671D1D5C; // 
            public const nint  = 0x701165C; // 
            public const nint  = 0xF7161E5C; // 
            public const nint  = 0x771E1E5C; // 
            public const nint  = 0x71B1B5C; // 
            public const nint  = 0xD717074C; // 
            public const nint  = 0x970D085C; // 
            public const nint  = 0xF7F01F5C; // 
            public const nint  = 0x8701085C; // 
            public const nint  = 0x17F0025C; // 
            public const nint  = 0x8705085C; // 
            public const nint  = 0x17F00A5C; // 
            public const nint  = 0x37161601; // 
            public const nint  = 0xF7140C5C; // 
            public const nint  = 0x271400C0; // 
            public const nint  = 0x37181D50; // 
            public const nint  = 0xA70E18F0; // 
            public const nint  = 0x71B0A49; // 
            public const nint  = 0xA090B39; // 
            public const nint  = 0x70F0A49; // 
            public const nint  = 0x8706185C; // 
            public const nint  = 0x7190732; // 
            public const nint  = 0x71D0732; // 
            public const nint  = 0xE7031E38; // 
            public const nint  = 0x770A0A49; // 
            public const nint  = 0xE7000E33; // 
            public const nint  = 0xE7131F32; // 
            public const nint  = 0xE7101032; // 
            public const nint  = 0xB7080732; // 
            public const nint  = 0x37FF1A01; // 
            public const nint  = 0x27170C58; // 
            public const nint  = 0x971B1B5C; // 
            public const nint  = 0xB7161C4B; // 
            public const nint  = 0x57FF045C; // 
            public const nint  = 0x70B0F5C; // 
            public const nint  = 0x37010F5C; // 
            public const nint  = 0x67090E5C; // 
            public const nint  = 0x4717175C; // 
            public const nint  = 0xEA0E02D8; // 
            public const nint  = 0x8000005C; // 
            public const nint  = 0xC7FF095C; // 
            public const nint  = 0x71B0F5C; // 
            public const nint  = 0x709095C; // 
            public const nint  = 0xE7000C38; // 
            public const nint  = 0x1713115C; // 
            public const nint  = 0xA011849; // 
            public const nint  = 0x9702FF5C; // 
            public const nint  = 0x97021A5B; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0xE3041E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x90319D8; // 
            public const nint  = 0xF71900E3; // 
            public const nint  = 0x97008F5C; // 
            public const nint  = 0x171CFF5B; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0x5B; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0xC80E0250; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0x9700875C; // 
            public const nint  = 0xF71C0058; // 
            public const nint  = 0xF71C1FE2; // 
            public const nint  = 0xF71C075B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D00D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E02E3; // 
            public const nint  = 0x9011849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80319D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0x8A000F5C; // 
            public const nint  = 0xC70E19F0; // 
            public const nint  = 0x770D18E3; // 
            public const nint  = 0x790D02D8; // 
            public const nint  = 0x7000F49; // 
            public const nint  = 0x9702005C; // 
            public const nint  = 0x3000F5C; // 
            public const nint  = 0x9703A7E2; // 
            public const nint  = 0xE3060E5B; // 
            public const nint  = 0x7404175C; // 
            public const nint  = 0xFE2; // 
            public const nint  = 0x1000FF0; // 
            public const nint  = 0xC80E00E3; // 
            public const nint  = 0x9031849; // 
            public const nint  = 0xF718035C; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0x8000005B; // 
            public const nint  = 0x97038F5B; // 
            public const nint  = 0xD4060D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x97030158; // 
            public const nint  = 0x97029FE2; // 
            public const nint  = 0x9702875B; // 
            public const nint  = 0xC3040C5C; // 
            public const nint  = 0xE7001B5C; // 
            public const nint  = 0x77001D5C; // 
            public const nint  = 0xC7000D5C; // 
            public const nint  = 0xE2; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x7000FD8; // 
            public const nint  = 0x80119D8; // 
            public const nint  = 0xF71902E3; // 
            public const nint  = 0x7F02750; // 
            public const nint  = 0x1700FF58; // 
            public const nint  = 0xA001F5B; // 
            public const nint  = 0xA3041A5C; // 
            public const nint  = 0xBA000E5C; // 
            public const nint  = 0xDA00175C; // 
            public const nint  = 0xE7140E5B; // 
            public const nint  = 0x70150D5C; // 
            public const nint  = 0x670F165C; // 
            public const nint  = 0x870F185C; // 
            public const nint  = 0x670D0D5B; // 
            public const nint  = 0xF70B015C; // 
            public const nint  = 0xA7150A5C; // 
            public const nint  = 0x77110449; // 
            public const nint  = 0x7130049; // 
            public const nint  = 0x37130359; // 
            public const nint  = 0x2704045C; // 
            public const nint  = 0x703875C; // 
            public const nint  = 0x87000FE3; // 
            public const nint  = 0x70F0050; // 
            public const nint  = 0xC122BB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8F800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xB5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCB000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x127000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x14A800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x160800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x189000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10001A00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x600; // 
            public const nint  = 0x3800; // 
            public const nint  = 0x10008209; // 
            public const nint  = 0x203600; // 
            public const nint  = 0x20803A00; // 
            public const nint  = 0x4180; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0x10001205; // 
            public const nint  = 0x100; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100; // 
            public const nint  = 0x10024600; // 
            public const nint  = 0xA1C91000; // 
            public const nint  = 0x8F16DC83; // 
            public const nint  = 0x1A5A16DD; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF14B0010; // 
            public const nint  = 0x8D00; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0xB0C62000; // 
            public const nint 8 = 0x26AE800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5E; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310F1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BC000; // 
            public const nint  = 0x100; // 
            public const nint  = 0x48000; // 
            public const nint  = 0x7094F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B4800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x5047B000; // 
            public const nint  = 0x2798800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3104F000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1AEA00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x838000; // 
            public const nint  = 0x6FA5F000; // 
            public const nint  = 0x440100; // 
            public const nint  = 0x20000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x383D600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7B00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3000; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2882800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDA; // 
            public const nint  = 0x0; // 
            public const nint  = 0x31115000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x78; // 
            public const nint  = 0x1B0400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x71033000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x12FC00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x4F8EB000; // 
            public const nint r = 0x296C800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x30B69000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BE300; // 
            public const nint  = 0x100; // 
            public const nint  = 0x480000; // 
            public const nint  = 0x6F98F000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x46D400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x783780BF; // 
            public const nint  = 0x40260000; // 
            public const nint  = 0x2A56800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFC; // 
            public const nint  = 0x0; // 
            public const nint  = 0x310B5000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF; // 
            public const nint  = 0x1BBE00; // 
            public const nint  = 0x100; // 
            public const nint  = 0x258000; // 
            public const nint  = 0x70F0B000; // 
            public const nint  = 0x2040100; // 
            public const nint  = 0x7B000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4200; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7834D094; // 
            public const nint �� = 0x40DC8000; // 
            public const nint  = 0x2B40800; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC5; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapNamedModelMeshGroupToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointOrientationToCPVelocity {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_RopeSpringConstraint {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_PositionWarpScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ForceControlPointStub {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_VectorNoise {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapParticleCountToScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_QuantizeFloat {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RemapModelVolumetoCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetToCP {
        }
        // Parent: None
        // Field count: 47
        public static class ParticleControlPointDriver_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ParentVortices {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetControlPointToCPVelocity {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ClientPhysics {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SpinYaw {
        }
        // Parent: None
        // Field count: 949
        public static class PointDefinitionWithTimeValues_t {
            public const nint  = 0x0; // 
            public const nint  = 0x208E46; // 
            public const nint  = 0x208E46; // 
            public const nint  = 0x1; // 
            public const nint  = 0x5555; // 
            public const nint  = 0x3001062; // 
            public const nint  = 0x4; // 
            public const nint  = 0x3000065; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x10001A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x7; // 
            public const nint  = 0x1; // 
            public const nint  = 0x8000000; // 
            public const nint  = 0x9000032; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7000001; // 
            public const nint  = 0x100042; // 
            public const nint  = 0x2; // 
            public const nint  = 0x107E46; // 
            public const nint  = 0x2; // 
            public const nint  = 0x100AA6; // 
            public const nint  = 0x4; // 
            public const nint  = 0x4; // 
            public const nint  = 0x4; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100006; // 
            public const nint  = 0x1; // 
            public const nint  = 0x5; // 
            public const nint  = 0x107E46; // 
            public const nint  = 0x5; // 
            public const nint  = 0x100556; // 
            public const nint  = 0x1; // 
            public const nint  = 0x7000055; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x4; // 
            public const nint  = 0x8; // 
            public const nint  = 0x2; // 
            public const nint  = 0x4; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x4; // 
            public const nint  = 0x0; // 
            public const nint  = 0x9000032; // 
            public const nint  = 0xBF000000; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x10001A; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106000; // 
            public const nint  = 0x9; // 
            public const nint  = 0x107E46; // 
            public const nint  = 0x5000036; // 
            public const nint  = 0x100046; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x1000015; // 
            public const nint  = 0x8000000; // 
            public const nint  = 0x9000032; // 
            public const nint  = 0xB; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8000000; // 
            public const nint  = 0x9000032; // 
            public const nint  = 0x3; // 
            public const nint  = 0x700004F; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x3; // 
            public const nint  = 0x3; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x9; // 
            public const nint  = 0x5; // 
            public const nint  = 0x5; // 
            public const nint  = 0x414EB852; // 
            public const nint  = 0xA000038; // 
            public const nint  = 0x3ED55555; // 
            public const nint  = 0x100072; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5; // 
            public const nint  = 0x9000037; // 
            public const nint  = 0xA; // 
            public const nint  = 0x3F372474; // 
            public const nint  = 0x10000A; // 
            public const nint  = 0x2; // 
            public const nint  = 0x7000018; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x1000015; // 
            public const nint  = 0x0; // 
            public const nint  = 0x106000; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x100E46; // 
            public const nint  = 0x7000020; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x2; // 
            public const nint  = 0x4; // 
            public const nint  = 0x0; // 
            public const nint  = 0xA000038; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100032; // 
            public const nint  = 0x0; // 
            public const nint  = 0x9; // 
            public const nint  = 0x9000037; // 
            public const nint  = 0x9; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100206; // 
            public const nint  = 0x3E000000; // 
            public const nint  = 0x5; // 
            public const nint  = 0x7000018; // 
            public const nint  = 0x10000A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x1000015; // 
            public const nint  = 0x100012; // 
            public const nint  = 0x4; // 
            public const nint  = 0x8; // 
            public const nint  = 0x2; // 
            public const nint  = 0x2; // 
            public const nint  = 0x100012; // 
            public const nint  = 0x4; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x10100A; // 
            public const nint  = 0x9; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x2; // 
            public const nint  = 0xA; // 
            public const nint  = 0x300001F; // 
            public const nint  = 0x9000045; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10101A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x9; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100906; // 
            public const nint  = 0x100006; // 
            public const nint  = 0x1; // 
            public const nint  = 0x1; // 
            public const nint  = 0x8010003A; // 
            public const nint  = 0x10000A; // 
            public const nint  = 0x4; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100806; // 
            public const nint  = 0x100032; // 
            public const nint  = 0x9; // 
            public const nint  = 0x9; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3B4D2E1C; // 
            public const nint  = 0x5; // 
            public const nint  = 0x100EA6; // 
            public const nint  = 0x1000C2; // 
            public const nint  = 0x1; // 
            public const nint  = 0x80100006; // 
            public const nint  = 0x10001A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x1000015; // 
            public const nint  = 0x100006; // 
            public const nint  = 0x1; // 
            public const nint  = 0x1; // 
            public const nint  = 0x300004C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100AA6; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x3; // 
            public const nint  = 0x3; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x3000006; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x100906; // 
            public const nint  = 0x100072; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x3; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x4; // 
            public const nint  = 0x0; // 
            public const nint  = 0xA000010; // 
            public const nint  = 0x3DE978D5; // 
            public const nint  = 0x3000006; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x9; // 
            public const nint  = 0x7000001; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x0; // 
            public const nint  = 0x3; // 
            public const nint  = 0x100042; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x41; // 
            public const nint  = 0x3; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x0; // 
            public const nint  = 0xA000032; // 
            public const nint  = 0x100AE6; // 
            public const nint  = 0x3000006; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x3; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x41; // 
            public const nint  = 0x3; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x80100AA6; // 
            public const nint  = 0x100AA6; // 
            public const nint  = 0x0; // 
            public const nint  = 0xA000000; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x3; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x100E46; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x3; // 
            public const nint  = 0x3; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x3; // 
            public const nint  = 0x3; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0xBF800000; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x300001F; // 
            public const nint  = 0x1000012; // 
            public const nint  = 0x9; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100042; // 
            public const nint  = 0x5; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x10100A; // 
            public const nint  = 0x8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x0; // 
            public const nint  = 0x9; // 
            public const nint  = 0x300001F; // 
            public const nint  = 0x9000045; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10101A; // 
            public const nint  = 0x3; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100E46; // 
            public const nint  = 0x100AA6; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8010003A; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x100042; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x1000D2; // 
            public const nint  = 0x8; // 
            public const nint  = 0xB000032; // 
            public const nint  = 0x8; // 
            public const nint  = 0x80100386; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x8; // 
            public const nint  = 0x500002F; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x100072; // 
            public const nint  = 0x9; // 
            public const nint  = 0xBD6147AE; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100E06; // 
            public const nint  = 0x0; // 
            public const nint  = 0x9000032; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5000056; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x9; // 
            public const nint  = 0xA000010; // 
            public const nint  = 0x3DE978D5; // 
            public const nint  = 0x100072; // 
            public const nint  = 0x8; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100042; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x5; // 
            public const nint  = 0x500002F; // 
            public const nint  = 0x100046; // 
            public const nint  = 0x100032; // 
            public const nint  = 0x6; // 
            public const nint  = 0xBD6147AE; // 
            public const nint  = 0x3B4D2E1C; // 
            public const nint  = 0x100106; // 
            public const nint  = 0x5; // 
            public const nint  = 0x7000000; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x5; // 
            public const nint  = 0x5000056; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x1000015; // 
            public const nint  = 0x5; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4002; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100A26; // 
            public const nint  = 0x100012; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5; // 
            public const nint  = 0x107B46; // 
            public const nint  = 0x5000036; // 
            public const nint  = 0x1005D6; // 
            public const nint  = 0x10000A; // 
            public const nint  = 0x1000F2; // 
            public const nint  = 0x1000015; // 
            public const nint  = 0x5; // 
            public const nint  = 0x106000; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x1; // 
            public const nint  = 0x90901A; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20803A; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x414EB852; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xF000032; // 
            public const nint  = 0x100032; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100AE6; // 
            public const nint  = 0xBF000000; // 
            public const nint  = 0x1; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x100406; // 
            public const nint  = 0x0; // 
            public const nint  = 0x304001F; // 
            public const nint  = 0x107E46; // 
            public const nint  = 0x9000045; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100012; // 
            public const nint  = 0x100012; // 
            public const nint  = 0x9; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x1; // 
            public const nint  = 0x10001A; // 
            public const nint  = 0x3000006; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100556; // 
            public const nint  = 0x3000006; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100082; // 
            public const nint  = 0xA000010; // 
            public const nint  = 0x3DE978D5; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x2; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x4; // 
            public const nint  = 0xA000032; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x3E991687; // 
            public const nint  = 0x4; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x1; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x1; // 
            public const nint  = 0xA000032; // 
            public const nint  = 0x100756; // 
            public const nint  = 0x3000006; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5000036; // 
            public const nint  = 0xC; // 
            public const nint  = 0x7000001; // 
            public const nint  = 0x100062; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x4; // 
            public const nint  = 0xA000032; // 
            public const nint  = 0x100556; // 
            public const nint  = 0x3000006; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x3; // 
            public const nint  = 0x100E46; // 
            public const nint  = 0x8000000; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x3; // 
            public const nint  = 0x100E46; // 
            public const nint  = 0x4001; // 
            public const nint  = 0x3; // 
            public const nint  = 0x3F000000; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1000002; // 
            public const nint  = 0x41; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xA000032; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x2; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x10103A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x41; // 
            public const nint  = 0x1; // 
            public const nint  = 0x1; // 
            public const nint  = 0x8020803A; // 
            public const nint  = 0x41; // 
            public const nint  = 0x41; // 
            public const nint  = 0x2; // 
            public const nint  = 0x8010002A; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x1; // 
            public const nint  = 0x10103A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x8000036; // 
            public const nint  = 0x100022; // 
            public const nint  = 0xA000000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x208246; // 
            public const nint  = 0x10002A; // 
            public const nint  = 0x3F800000; // 
            public const nint  = 0x1; // 
            public const nint  = 0x8000000; // 
            public const nint  = 0x7000010; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x41; // 
            public const nint  = 0x1; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x1; // 
            public const nint  = 0x2; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10000A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10003A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8020803A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x4002; // 
            public const nint  = 0xA000032; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x1; // 
            public const nint  = 0x1; // 
            public const nint  = 0x10; // 
            public const nint  = 0x8; // 
            public const nint  = 0x1B; // 
            public const nint  = 0x50; // 
            public const nint  = 0x1B80; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10; // 
            public const nint  = 0x618; // 
            public const nint  = 0x1; // 
            public const nint  = 0x3; // 
            public const nint  = 0x0; // 
            public const nint  = 0xE003FF87; // 
            public const nint  = 0xE043FF88; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0xC0F85007; // 
            public const nint  = 0xE2900001; // 
            public const nint  = 0x49A00440; // 
            public const nint  = 0x5C601380; // 
            public const nint  = 0x5C601380; // 
            public const nint  = 0x5C601780; // 
            public const nint  = 0x5C601380; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x5C601380; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x3868103E; // 
            public const nint  = 0x5BB68380; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x1000000; // 
            public const nint  = 0x5CA00500; // 
            public const nint  = 0xC0F85007; // 
            public const nint  = 0xC0F85007; // 
            public const nint  = 0x5CA00500; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x3968103F; // 
            public const nint  = 0x49A00440; // 
            public const nint  = 0x32A0053E; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x32A20B3F; // 
            public const nint  = 0x32A003BF; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x5C5A5000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x5C5A1000; // 
            public const nint  = 0x5C5B1000; // 
            public const nint  = 0x51A00D40; // 
            public const nint  = 0x5B450500; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x4B4B0540; // 
            public const nint  = 0x3868103F; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0xE2900001; // 
            public const nint  = 0x5C5B1000; // 
            public const nint  = 0x3868103F; // 
            public const nint  = 0x3868103E; // 
            public const nint  = 0x5C980780; // 
            public const nint  = 0x5C681000; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x58C68380; // 
            public const nint  = 0x58C60380; // 
            public const nint  = 0x5BB98380; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xE2900000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x49A00140; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x5BB68380; // 
            public const nint  = 0x5C433000; // 
            public const nint  = 0xE2A00000; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xE2900000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x5BB68380; // 
            public const nint  = 0x5B5A2000; // 
            public const nint  = 0x5B640400; // 
            public const nint  = 0x5B6A2000; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x58C62000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0x5BB98480; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x5C433000; // 
            public const nint  = 0xE2A00000; // 
            public const nint  = 0x5BB6A080; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xE2900000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x49A00140; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x58C62080; // 
            public const nint  = 0x5BB98400; // 
            public const nint  = 0x5BB6A000; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x58C62000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0x5BB98480; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x5C433000; // 
            public const nint  = 0xE2A00000; // 
            public const nint  = 0x5BB6A080; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xE2900000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x49A00140; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x58C62080; // 
            public const nint  = 0x5BB98400; // 
            public const nint  = 0x5BB6A000; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x58C62000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0x5BB98480; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x5C433000; // 
            public const nint  = 0xE2A00000; // 
            public const nint  = 0x5BB6A080; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xE2900000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x49A00140; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x58C62080; // 
            public const nint  = 0x5BB98400; // 
            public const nint  = 0x5BB6A000; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD84C500F; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x58C62000; // 
            public const nint  = 0xE2400000; // 
            public const nint  = 0x5BB98480; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0xD850500F; // 
            public const nint  = 0x49A00140; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x5C433000; // 
            public const nint  = 0xE2A00000; // 
            public const nint  = 0x5BB6A080; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0xE2900000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x58C62080; // 
            public const nint  = 0x5BB98400; // 
            public const nint  = 0x5BB6A000; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C980780; // 
            public const nint  = 0x5C980780; // 
            public const nint  = 0x5C980780; // 
            public const nint  = 0xE2900000; // 
            public const nint  = 0xF0F80000; // 
            public const nint  = 0xE3400000; // 
            public const nint  = 0x49A00040; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x50900380; // 
            public const nint  = 0x5C423000; // 
            public const nint  = 0x50900380; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x5C980780; // 
            public const nint  = 0x5C980780; // 
            public const nint  = 0x5C583000; // 
            public const nint  = 0x5C591000; // 
            public const nint  = 0x58A10B80; // 
            public const nint  = 0x58A10B80; // 
            public const nint  = 0x51A10040; // 
            public const nint  = 0x5B450500; // 
            public const nint  = 0x5C581000; // 
            public const nint  = 0x49A000C0; // 
            public const nint  = 0x59A10900; // 
            public const nint  = 0x59A10280; // 
            public const nint  = 0x5C601780; // 
            public const nint  = 0x36B383B3; // 
            public const nint  = 0xE2400FFF; // 
            public const nint  = 0x50B00000; // 
            public const nint  = 0x3B23D70A; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x808; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xA48; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xC78; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xDD8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1010; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1190; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1398; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15D0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1730; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1AF8; // 
            public const nint  = 0x100022; // 
            public const nint  = 0x1; // 
            public const nint  = 0x8000034; // 
            public const nint  = 0x7000038; // 
            public const nint  = 0x100082; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100072; // 
            public const nint  = 0x41; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8020803A; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x4002; // 
            public const nint  = 0xA000032; // 
            public const nint  = 0x100246; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xCAB1508F; // 
            public const nint  = 0xDD7F5579; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x2FFFF03; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint   = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x2FFFF03; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint   = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x20; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x2FFFF03; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x2FFFF03; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint   = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x2FFFF03; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFFFFFFFF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10001; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint   = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x2FFFF03; // 
        }
        // Parent: None
        // Field count: 47
        public static class RenderProjectedMaterial_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_SetFloatAttributeToVectorExpression {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_ExternalWindForce {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_ModelCull {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderSprites {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_PercentageBetweenTransformLerpCPs {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetPerChildControlPoint {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderTreeShake {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_WorldCollideConstraint {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_SetAttributeToScalarExpression {
        }
        // Parent: particles
        // Field count: 0
        public static class C_OP_CycleScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RenderMaterialProxy {
        }
        // Parent: None
        // Field count: 47
        public static class FloatInputMaterialVariable_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RampScalarLinear {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_RotateVector {
        }
        // Parent: particles
        // Field count: 0
        public static class C_INIT_InitVecCollection {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_RemapParticleCountToNamedModelMeshGroupScalar {
        }
        // Parent: None
        // Field count: 0
        public static class C_INIT_SequenceFromCP {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_CPOffsetToPercentageBetweenCPs {
        }
        // Parent: None
        // Field count: 0
        public static class C_OP_LerpEndCapScalar {
        }
    }
}
