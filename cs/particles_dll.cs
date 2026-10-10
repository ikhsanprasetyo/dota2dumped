// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-10 12:12:06.506827800 +07:00

namespace Source2Dumper.Schemas {
    // Module: particles.dll
    // Class count: 346
    // Enum count: 77
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
        // Member count: 2
        public enum PulseCursorWakePriority_t : uint {
            WakeElegantly = 0x0,
            WakeImmediate = 0x1
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
        // Member count: 4
        public enum ParticleEntityPos_t : uint {
            PARTICLE_ABS_ORIGIN = 0x0,
            PARTICLE_WORLDSPACE_CENTER = 0x1,
            PARTICLE_EYES = 0x2,
            PARTICLE_FLASHLIGHT = 0x3
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
        // Member count: 13
        public enum EventTypeSelection_t : uint {
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
        // Member count: 11
        public enum ParticleAttrBoxFlags_t : uint {
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
        // Member count: 5
        public enum ParticleOmni2LighOrientationChoiceList_t : uint {
            PARTICLE_OMNI2_LIGHT_ORIENTATION_ROTATIONS = 0x0,
            PARTICLE_OMNI2_LIGHT_ORIENTATION_NORMAL = 0x1,
            PARTICLE_OMNI2_LIGHT_ORIENTATION_NORMAL_ROLL = 0x2,
            PARTICLE_OMNI2_LIGHT_ORIENTATION_TARGET = 0x3,
            PARTICLE_OMNI2_LIGHT_ORIENTATION_TARGET_ROLL = 0x4
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
        // Member count: 5
        public enum BBoxVolumeType_t : uint {
            BBOX_VOLUME = 0x0,
            BBOX_DIMENSIONS = 0x1,
            BBOX_MINS_MAXS = 0x2,
            BBOX_RADIUS = 0x3,
            BBOX_SURFACE_AREA = 0x4
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
        // Member count: 4
        public enum ParticleVolumetricSmokeType_t : uint {
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_EMISSION = 0x0,
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_SINK = 0x1,
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_REPEL = 0x2,
            PARTICLE_VOLUMETRIC_SMOKE_TYPE_TRACE = 0x3
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
        // Member count: 3
        public enum ParticleOmni2LightTypeChoiceList_t : uint {
            PARTICLE_OMNI2_LIGHT_TYPE_POINT = 0x0,
            PARTICLE_OMNI2_LIGHT_TYPE_SPHERE = 0x1,
            PARTICLE_OMNI2_LIGHT_TYPE_BARN = 0x2
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
        // Field count: 0
        public static class IParticleCollection {
        }
        // Parent: None
        // Field count: 4
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapGravityToVector {
            public const nint m_vInput1 = 0x1E0; // CPerParticleVecInput
            public const nint m_nOutputField = 0x8B8; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x8BC; // ParticleSetMethod_t
            public const nint m_bNormalizedOutput = 0x8C0; // bool
        }
        // Parent: None
        // Field count: 16
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RenderDeferredLight {
            public const nint m_flRadiusScale = 0x230; // float32
            public const nint m_flAlphaScale = 0x234; // float32
            public const nint m_nAlpha2Field = 0x238; // ParticleAttributeIndex_t
            public const nint m_vecColorScale = 0x240; // CParticleCollectionVecInput
            public const nint m_nColorBlendType = 0x918; // ParticleColorBlendType_t
            public const nint m_bUseTexture = 0x91C; // bool
            public const nint m_bUseAlphaTestWindow = 0x91D; // bool
            public const nint m_hTexture = 0x920; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_nAlphaTestPointField = 0x928; // ParticleAttributeIndex_t
            public const nint m_nAlphaTestRangeField = 0x92C; // ParticleAttributeIndex_t
            public const nint m_nAlphaTestSharpnessField = 0x930; // ParticleAttributeIndex_t
            public const nint m_flLightDistance = 0x934; // float32
            public const nint m_flStartFalloff = 0x938; // float32
            public const nint m_flDistanceFalloff = 0x93C; // float32
            public const nint m_flSpotFoV = 0x940; // float32
            public const nint m_nHSVShiftControlPoint = 0x944; // int32
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
        // MPropertyAttributeChoiceName
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
        public static class C_OP_RemapTransformToVelocity {
            public const nint m_TransformInput = 0x1E0; // CParticleTransformInput
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        public static class CollisionGroupContext_t {
            public const nint m_nCollisionGroupNumber = 0x0; // int32
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_FadeOutSimple {
            public const nint m_flFadeOutTime = 0x1E0; // float32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MPropertySuppressExpr
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
        public static class C_OP_SpringToVectorConstraint {
            public const nint m_flRestLength = 0x1E0; // CPerParticleFloatInput
            public const nint m_flMinDistance = 0x358; // CPerParticleFloatInput
            public const nint m_flMaxDistance = 0x4D0; // CPerParticleFloatInput
            public const nint m_flRestingLength = 0x648; // CPerParticleFloatInput
            public const nint m_vecAnchorVector = 0x7C0; // CPerParticleVecInput
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_INIT_StatusEffectCitadel {
            public const nint m_flSFXColorWarpAmount = 0x1E8; // float32
            public const nint m_flSFXNormalAmount = 0x1EC; // float32
            public const nint m_flSFXMetalnessAmount = 0x1F0; // float32
            public const nint m_flSFXRoughnessAmount = 0x1F4; // float32
            public const nint m_flSFXSelfIllumAmount = 0x1F8; // float32
            public const nint m_flSFXSScale = 0x1FC; // float32
            public const nint m_flSFXSScrollX = 0x200; // float32
            public const nint m_flSFXSScrollY = 0x204; // float32
            public const nint m_flSFXSScrollZ = 0x208; // float32
            public const nint m_flSFXSOffsetX = 0x20C; // float32
            public const nint m_flSFXSOffsetY = 0x210; // float32
            public const nint m_flSFXSOffsetZ = 0x214; // float32
            public const nint m_nDetailCombo = 0x218; // DetailCombo_t
            public const nint m_flSFXSDetailAmount = 0x21C; // float32
            public const nint m_flSFXSDetailScale = 0x220; // float32
            public const nint m_flSFXSDetailScrollX = 0x224; // float32
            public const nint m_flSFXSDetailScrollY = 0x228; // float32
            public const nint m_flSFXSDetailScrollZ = 0x22C; // float32
            public const nint m_flSFXSUseModelUVs = 0x230; // float32
        }
        // Parent: None
        // Field count: 12
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        public static class C_OP_RenderSound {
            public const nint m_flDurationScale = 0x230; // float32
            public const nint m_flSndLvlScale = 0x234; // float32
            public const nint m_flPitchScale = 0x238; // float32
            public const nint m_flVolumeScale = 0x23C; // float32
            public const nint m_nSndLvlField = 0x240; // ParticleAttributeIndex_t
            public const nint m_nDurationField = 0x244; // ParticleAttributeIndex_t
            public const nint m_nPitchField = 0x248; // ParticleAttributeIndex_t
            public const nint m_nVolumeField = 0x24C; // ParticleAttributeIndex_t
            public const nint m_nChannel = 0x250; // int32
            public const nint m_nCPReference = 0x254; // int32
            public const nint m_pszSoundName = 0x258; // char[256]
            public const nint m_bSuppressStopSoundEvent = 0x358; // bool
        }
        // Parent: None
        // Field count: 19
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // COLLISION_MODE_PER_PARTICLE_TRACE
        // COLLISION_MODE_USE_NEAREST_TRACE
        public static class CParticleVisibilityInputs {
            public const nint m_flCameraBias = 0x0; // float32
            public const nint m_nCPin = 0x4; // int32
            public const nint m_flProxyRadius = 0x8; // float32
            public const nint m_flInputMin = 0xC; // float32
            public const nint m_flInputMax = 0x10; // float32
            public const nint m_flInputPixelVisFade = 0x14; // float32
            public const nint m_flNoPixelVisibilityFallback = 0x18; // float32
            public const nint m_flDistanceInputMin = 0x1C; // float32
            public const nint m_flDistanceInputMax = 0x20; // float32
            public const nint m_flDotInputMin = 0x24; // float32
            public const nint m_flDotInputMax = 0x28; // float32
            public const nint m_bDotCPAngles = 0x2C; // bool
            public const nint m_bDotCameraAngles = 0x2D; // bool
            public const nint m_flAlphaScaleMin = 0x30; // float32
            public const nint m_flAlphaScaleMax = 0x34; // float32
            public const nint m_flRadiusScaleMin = 0x38; // float32
            public const nint m_flRadiusScaleMax = 0x3C; // float32
            public const nint m_flRadiusScaleFOVBase = 0x40; // float32
            public const nint m_bRightEye = 0x44; // bool
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
        // MPropertyFriendlyName
        public static class C_OP_SetControlPointsToParticle {
            public const nint m_nChildGroupID = 0x1E0; // int32
            public const nint m_nFirstControlPoint = 0x1E4; // int32
            public const nint m_nNumControlPoints = 0x1E8; // int32
            public const nint m_nFirstSourcePoint = 0x1EC; // int32
            public const nint m_bReverse = 0x1F0; // bool
            public const nint m_bSetOrientation = 0x1F1; // bool
            public const nint m_nOrientationMode = 0x1F4; // ParticleOrientationSetMode_t
            public const nint m_nSetParent = 0x1F8; // ParticleParentSetMode_t
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
        // MParticleMinVersion
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
        // MGetKV3ClassDefaults
        public static class C_OP_RemapCPVelocityToVector {
            public const nint m_nControlPoint = 0x1E0; // int32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flScale = 0x1E8; // float32
            public const nint m_bNormalize = 0x1EC; // bool
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_PointVectorAtNextParticle {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInterpolation = 0x1E8; // CPerParticleFloatInput
            public const nint m_bPrevious = 0x360; // bool
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // PARTICLE_WORLDSPACE_CENTER
        // PARTICLE_EYES
        public static class ParticlePreviewBodyGroup_t {
            public const nint m_bodyGroupName = 0x0; // CUtlString
            public const nint m_nValue = 0x8; // int32
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
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_OscillateScalarSimple {
            public const nint m_Rate = 0x1E0; // float32
            public const nint m_Frequency = 0x1E4; // float32
            public const nint m_nField = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_flOscMult = 0x1EC; // float32
            public const nint m_flOscAdd = 0x1F0; // float32
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
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        public static class C_INIT_StatusEffect {
            public const nint m_nDetail2Combo = 0x1E8; // Detail2Combo_t
            public const nint m_flDetail2Rotation = 0x1EC; // float32
            public const nint m_flDetail2Scale = 0x1F0; // float32
            public const nint m_flDetail2BlendFactor = 0x1F4; // float32
            public const nint m_flColorWarpIntensity = 0x1F8; // float32
            public const nint m_flDiffuseWarpBlendToFull = 0x1FC; // float32
            public const nint m_flEnvMapIntensity = 0x200; // float32
            public const nint m_flAmbientScale = 0x204; // float32
            public const nint m_specularColor = 0x208; // Color
            public const nint m_flSpecularScale = 0x20C; // float32
            public const nint m_flSpecularExponent = 0x210; // float32
            public const nint m_flSpecularExponentBlendToFull = 0x214; // float32
            public const nint m_flSpecularBlendToFull = 0x218; // float32
            public const nint m_rimLightColor = 0x21C; // Color
            public const nint m_flRimLightScale = 0x220; // float32
            public const nint m_flReflectionsTintByBaseBlendToNone = 0x224; // float32
            public const nint m_flMetalnessBlendToFull = 0x228; // float32
            public const nint m_flSelfIllumBlendToFull = 0x22C; // float32
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
        // MPropertyAttributeEditor
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        public static class C_INIT_RandomVector {
            public const nint m_vecMin = 0x1E8; // Vector
            public const nint m_vecMax = 0x1F4; // Vector
            public const nint m_nFieldOutput = 0x200; // ParticleAttributeIndex_t
            public const nint m_randomnessParameters = 0x204; // CRandomNumberGeneratorParameters
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        public static class C_INIT_InitialVelocityNoise {
            public const nint m_vecAbsVal = 0x1E8; // Vector
            public const nint m_vecAbsValInv = 0x1F4; // Vector
            public const nint m_vecOffsetLoc = 0x200; // CPerParticleVecInput
            public const nint m_flOffset = 0x8D8; // CPerParticleFloatInput
            public const nint m_vecOutputMin = 0xA50; // CPerParticleVecInput
            public const nint m_vecOutputMax = 0x1128; // CPerParticleVecInput
            public const nint m_flNoiseScale = 0x1800; // CPerParticleFloatInput
            public const nint m_flNoiseScaleLoc = 0x1978; // CPerParticleFloatInput
            public const nint m_TransformInput = 0x1AF0; // CParticleTransformInput
            public const nint m_bIgnoreDt = 0x1B58; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapScalarOnceTimed {
            public const nint m_bProportional = 0x1E0; // bool
            public const nint m_nFieldInput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1EC; // float32
            public const nint m_flInputMax = 0x1F0; // float32
            public const nint m_flOutputMin = 0x1F4; // float32
            public const nint m_flOutputMax = 0x1F8; // float32
            public const nint m_flRemapTime = 0x1FC; // float32
        }
        // Parent: None
        // Field count: 0
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_RandomNamedModelSequence {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_PlaneCull {
            public const nint m_nPlaneControlPoint = 0x1E0; // int32
            public const nint m_vecPlaneDirection = 0x1E8; // CParticleCollectionVecInput
            public const nint m_bLocalSpace = 0x8C0; // bool
            public const nint m_flPlaneOffset = 0x8C4; // float32
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
        public static class C_OP_ModelDampenMovement {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_bBoundBox = 0x1E4; // bool
            public const nint m_bOutside = 0x1E5; // bool
            public const nint m_bUseBones = 0x1E6; // bool
            public const nint m_HitboxSetName = 0x1E7; // char[128]
            public const nint m_vecPosOffset = 0x268; // CPerParticleVecInput
            public const nint m_fDrag = 0x940; // float32
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
        // MPropertySuppressExpr
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
        public static class C_OP_TwistAroundAxis {
            public const nint m_fForceAmount = 0x1F0; // float32
            public const nint m_TwistAxis = 0x1F4; // Vector
            public const nint m_bLocalSpace = 0x200; // bool
            public const nint m_nControlPointNumber = 0x204; // int32
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class CSpinUpdateBase {
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_OrientTo2dDirection {
            public const nint m_vecInput = 0x1E0; // CPerParticleVecInput
            public const nint m_flRotOffset = 0x8B8; // float32
            public const nint m_flSpinStrength = 0x8BC; // float32
            public const nint m_nFieldOutput = 0x8C0; // ParticleAttributeIndex_t
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RemapDotProductToCP {
            public const nint m_nInputCP1 = 0x1E8; // int32
            public const nint m_nInputCP2 = 0x1EC; // int32
            public const nint m_nOutputCP = 0x1F0; // int32
            public const nint m_nOutVectorField = 0x1F4; // int32
            public const nint m_flInputMin = 0x1F8; // CParticleCollectionFloatInput
            public const nint m_flInputMax = 0x370; // CParticleCollectionFloatInput
            public const nint m_flOutputMin = 0x4E8; // CParticleCollectionFloatInput
            public const nint m_flOutputMax = 0x660; // CParticleCollectionFloatInput
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_RemapParticleCountToNamedModelElementScalar {
            public const nint m_hModel = 0x218; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_outputMinName = 0x220; // CUtlString
            public const nint m_outputMaxName = 0x228; // CUtlString
            public const nint m_bModelFromRenderer = 0x230; // bool
        }
        // Parent: None
        // Field count: 20
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RenderTrails {
            public const nint m_bEnableFadingAndClamping = 0x3358; // bool
            public const nint m_flStartFadeDot = 0x335C; // float32
            public const nint m_flEndFadeDot = 0x3360; // float32
            public const nint m_nPrevPntSource = 0x3364; // ParticleAttributeIndex_t
            public const nint m_flMaxLength = 0x3368; // float32
            public const nint m_flMinLength = 0x336C; // float32
            public const nint m_bIgnoreDT = 0x3370; // bool
            public const nint m_flConstrainRadiusToLengthRatio = 0x3374; // float32
            public const nint m_flLengthScale = 0x3378; // float32
            public const nint m_flLengthFadeInTime = 0x337C; // float32
            public const nint m_flRadiusHeadTaper = 0x3380; // CPerParticleFloatInput
            public const nint m_vecHeadColorScale = 0x34F8; // CParticleCollectionVecInput
            public const nint m_flHeadAlphaScale = 0x3BD0; // CPerParticleFloatInput
            public const nint m_flRadiusTaper = 0x3D48; // CPerParticleFloatInput
            public const nint m_vecTailColorScale = 0x3EC0; // CParticleCollectionVecInput
            public const nint m_flTailAlphaScale = 0x4598; // CPerParticleFloatInput
            public const nint m_nHorizCropField = 0x4710; // ParticleAttributeIndex_t
            public const nint m_nVertCropField = 0x4714; // ParticleAttributeIndex_t
            public const nint m_flForwardShift = 0x4718; // float32
            public const nint m_bFlipUVBasedOnPitchYaw = 0x471C; // bool
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetControlPointPositionToTimeOfDayValue {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_pszTimeOfDayParameter = 0x1EC; // char[128]
            public const nint m_vecDefaultValue = 0x26C; // Vector
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MGetKV3ClassDefaults
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
        // PARTICLE_MASSMODE_RADIUS_SQUARED
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DecayMaintainCount {
            public const nint m_nParticlesToMaintain = 0x1E0; // int32
            public const nint m_flDecayDelay = 0x1E4; // float32
            public const nint m_nSnapshotControlPoint = 0x1E8; // int32
            public const nint m_strSnapshotSubset = 0x1F0; // CUtlString
            public const nint m_bLifespanDecay = 0x1F8; // bool
            public const nint m_flScale = 0x200; // CParticleCollectionFloatInput
            public const nint m_bKillNewest = 0x378; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomModelSequence {
            public const nint m_ActivityName = 0x1E8; // char[256]
            public const nint m_SequenceName = 0x2E8; // char[256]
            public const nint m_hModel = 0x3E8; // CStrongHandle<InfoForResourceTypeCModel>
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_ExternalGameImpulseForce {
            public const nint m_flForceScale = 0x1F0; // CPerParticleFloatInput
            public const nint m_bRopes = 0x368; // bool
            public const nint m_bRopesZOnly = 0x369; // bool
            public const nint m_bExplosions = 0x36A; // bool
            public const nint m_bParticles = 0x36B; // bool
        }
        // Parent: None
        // Field count: 11
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        public static class C_OP_RemapAverageHitboxSpeedtoCP {
            public const nint m_nInControlPointNumber = 0x1E8; // int32
            public const nint m_nOutControlPointNumber = 0x1EC; // int32
            public const nint m_nField = 0x1F0; // int32
            public const nint m_nHitboxDataType = 0x1F4; // ParticleHitboxDataSelection_t
            public const nint m_flInputMin = 0x1F8; // CParticleCollectionFloatInput
            public const nint m_flInputMax = 0x370; // CParticleCollectionFloatInput
            public const nint m_flOutputMin = 0x4E8; // CParticleCollectionFloatInput
            public const nint m_flOutputMax = 0x660; // CParticleCollectionFloatInput
            public const nint m_nHeightControlPointNumber = 0x7D8; // int32
            public const nint m_vecComparisonVelocity = 0x7E0; // CParticleCollectionVecInput
            public const nint m_HitboxSetName = 0xEB8; // char[128]
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_INIT_RandomAlpha {
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_nAlphaMin = 0x1EC; // int32
            public const nint m_nAlphaMax = 0x1F0; // int32
            public const nint m_flAlphaRandExponent = 0x1FC; // float32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        public static class C_OP_NormalizeVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flScale = 0x1E4; // float32
        }
        // Parent: None
        // Field count: 5
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
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
        public static class C_OP_RepeatedTriggerChildGroup {
            public const nint m_nChildGroupID = 0x1E8; // int32
            public const nint m_flClusterRefireTime = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_flClusterSize = 0x368; // CParticleCollectionFloatInput
            public const nint m_flClusterCooldown = 0x4E0; // CParticleCollectionFloatInput
            public const nint m_bLimitChildCount = 0x658; // bool
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapVelocityToVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flScale = 0x1E4; // float32
            public const nint m_bNormalize = 0x1E8; // bool
        }
        // Parent: None
        // Field count: 12
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
        public static class C_INIT_RingWave {
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
            public const nint m_flParticlesPerOrbit = 0x250; // CParticleCollectionFloatInput
            public const nint m_flInitialRadius = 0x3C8; // CPerParticleFloatInput
            public const nint m_flThickness = 0x540; // CPerParticleFloatInput
            public const nint m_flInitialSpeedMin = 0x6B8; // CPerParticleFloatInput
            public const nint m_flInitialSpeedMax = 0x830; // CPerParticleFloatInput
            public const nint m_flRoll = 0x9A8; // CPerParticleFloatInput
            public const nint m_flPitch = 0xB20; // CPerParticleFloatInput
            public const nint m_flYaw = 0xC98; // CPerParticleFloatInput
            public const nint m_bEvenDistribution = 0xE10; // bool
            public const nint m_bXYVelocityOnly = 0xE11; // bool
            public const nint m_nControlPoint = 0x1E8; // int32
        }
        // Parent: None
        // Field count: 3
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomTrailLength {
            public const nint m_flMinLength = 0x1E8; // float32
            public const nint m_flMaxLength = 0x1EC; // float32
            public const nint m_flLengthRandExponent = 0x1F0; // float32
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_RemapScalar {
            public const nint m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E8; // float32
            public const nint m_flInputMax = 0x1EC; // float32
            public const nint m_flOutputMin = 0x1F0; // float32
            public const nint m_flOutputMax = 0x1F4; // float32
            public const nint m_bOldCode = 0x1F8; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DistanceBetweenTransforms {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_TransformStart = 0x1E8; // CParticleTransformInput
            public const nint m_TransformEnd = 0x250; // CParticleTransformInput
            public const nint m_flInputMin = 0x2B8; // CPerParticleFloatInput
            public const nint m_flInputMax = 0x430; // CPerParticleFloatInput
            public const nint m_flOutputMin = 0x5A8; // CPerParticleFloatInput
            public const nint m_flOutputMax = 0x720; // CPerParticleFloatInput
            public const nint m_flMaxTraceLength = 0x898; // float32
            public const nint m_flLOSScale = 0x89C; // float32
            public const nint m_CollisionGroupName = 0x8A0; // char[128]
            public const nint m_nTraceSet = 0x920; // ParticleTraceSet_t
            public const nint m_bLOS = 0x924; // bool
            public const nint m_nSetMethod = 0x928; // ParticleSetMethod_t
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_DecayOffscreen {
            public const nint m_flOffscreenTime = 0x1E0; // CParticleCollectionFloatInput
        }
        // Parent: None
        // Field count: 7
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
        // MPropertyAttributeEditor
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CreateSequentialPath {
            public const nint m_fMaxDistance = 0x1E8; // float32
            public const nint m_flNumToAssign = 0x1EC; // float32
            public const nint m_bLoop = 0x1F0; // bool
            public const nint m_bCPPairs = 0x1F1; // bool
            public const nint m_bSaveOffset = 0x1F2; // bool
            public const nint m_PathParams = 0x200; // CPathParameters
            public const nint m_bKillUnused = 0x1E8; // bool
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_OscillateVectorSimple {
            public const nint m_Rate = 0x1E0; // Vector
            public const nint m_Frequency = 0x1EC; // Vector
            public const nint m_nField = 0x1F8; // ParticleAttributeIndex_t
            public const nint m_flOscMult = 0x1FC; // float32
            public const nint m_flOscAdd = 0x200; // float32
            public const nint m_bOffset = 0x204; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_MoveBetweenPoints {
            public const nint m_flSpeedMin = 0x1E8; // CPerParticleFloatInput
            public const nint m_flSpeedMax = 0x360; // CPerParticleFloatInput
            public const nint m_flEndSpread = 0x4D8; // CPerParticleFloatInput
            public const nint m_flStartOffset = 0x650; // CPerParticleFloatInput
            public const nint m_flEndOffset = 0x7C8; // CPerParticleFloatInput
            public const nint m_nEndControlPointNumber = 0x940; // int32
            public const nint m_bTrailBias = 0x944; // bool
        }
        // Parent: None
        // Field count: 23
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_StatusEffectTf {
            public const nint m_flSFXColorWarpAmount = 0x1E8; // float32
            public const nint m_flSFXNormalAmount = 0x1EC; // float32
            public const nint m_flSFXMetalnessAmount = 0x1F0; // float32
            public const nint m_flSFXRoughnessAmount = 0x1F4; // float32
            public const nint m_flSFXSelfIllumAmount = 0x1F8; // float32
            public const nint m_flSFXSScale = 0x1FC; // float32
            public const nint m_flSFXSScrollX = 0x200; // float32
            public const nint m_flSFXSScrollY = 0x204; // float32
            public const nint m_flSFXSScrollZ = 0x208; // float32
            public const nint m_flSFXSOffsetX = 0x20C; // float32
            public const nint m_flSFXSOffsetY = 0x210; // float32
            public const nint m_flSFXSOffsetZ = 0x214; // float32
            public const nint m_nDetailCombo = 0x218; // DetailCombo_t
            public const nint m_flSFXSDetailAmount = 0x21C; // float32
            public const nint m_flSFXSDetailScale = 0x220; // float32
            public const nint m_flSFXSDetailScrollX = 0x224; // float32
            public const nint m_flSFXSDetailScrollY = 0x228; // float32
            public const nint m_flSFXSDetailScrollZ = 0x22C; // float32
            public const nint m_bAnimatedDetail = 0x230; // bool
            public const nint m_flSFXDetailAnimationTimePerFrame = 0x234; // float32
            public const nint m_flSFXDetailAnimationTimeOffset = 0x238; // float32
            public const nint m_flSFXSUseModelUVs = 0x23C; // float32
            public const nint m_flSFXEnvMapAmount = 0x240; // float32
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_SetUserEvent {
            public const nint m_flInput = 0x1E0; // CPerParticleFloatInput
            public const nint m_flRisingEdge = 0x358; // CPerParticleFloatInput
            public const nint m_nRisingEventType = 0x4D0; // EventTypeSelection_t
            public const nint m_flFallingEdge = 0x4D8; // CPerParticleFloatInput
            public const nint m_nFallingEventType = 0x650; // EventTypeSelection_t
        }
        // Parent: None
        // Field count: 2
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
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_QuantizeFloat {
            public const nint m_InputValue = 0x1E0; // CPerParticleFloatInput
            public const nint m_nOutputField = 0x358; // ParticleAttributeIndex_t
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomNamedModelElement {
            public const nint m_hModel = 0x1E8; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_names = 0x1F0; // CUtlVector<CUtlString>
            public const nint m_bShuffle = 0x208; // bool
            public const nint m_bLinear = 0x209; // bool
            public const nint m_bModelFromRenderer = 0x20A; // bool
            public const nint m_nFieldOutput = 0x20C; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        public static class C_OP_Callback {
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_GlobalLight {
            public const nint m_flScale = 0x1E0; // float32
            public const nint m_bClampLowerRange = 0x1E4; // bool
            public const nint m_bClampUpperRange = 0x1E5; // bool
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMinVersion
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_INIT_OffsetVectorToVector {
            public const nint m_nFieldInput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_vecOutputMin = 0x1F0; // Vector
            public const nint m_vecOutputMax = 0x1FC; // Vector
            public const nint m_randomnessParameters = 0x208; // CRandomNumberGeneratorParameters
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        public static class C_OP_SetPerChildControlPointFromAttribute {
            public const nint m_nChildGroupID = 0x1E0; // int32
            public const nint m_nFirstControlPoint = 0x1E4; // int32
            public const nint m_nNumControlPoints = 0x1E8; // int32
            public const nint m_nParticleIncrement = 0x1EC; // int32
            public const nint m_nFirstSourcePoint = 0x1F0; // int32
            public const nint m_bNumBasedOnParticleCount = 0x1F4; // bool
            public const nint m_nAttributeToRead = 0x1F8; // ParticleAttributeIndex_t
            public const nint m_nCPField = 0x1FC; // int32
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetParentControlPointsToChildCP {
            public const nint m_nChildGroupID = 0x1E8; // int32
            public const nint m_nChildControlPoint = 0x1EC; // int32
            public const nint m_nNumControlPoints = 0x1F0; // int32
            public const nint m_nFirstSourcePoint = 0x1F4; // int32
            public const nint m_bSetOrientation = 0x1F8; // bool
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
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_BoxConstraint {
            public const nint m_vecMin = 0x1E0; // CParticleCollectionVecInput
            public const nint m_vecMax = 0x8B8; // CParticleCollectionVecInput
            public const nint m_nCP = 0xF90; // int32
            public const nint m_bLocalSpace = 0xF94; // bool
            public const nint m_bAccountForRadius = 0xF95; // bool
        }
        // Parent: None
        // Field count: 14
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CreatePhyllotaxis {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_nScaleCP = 0x1EC; // int32
            public const nint m_nComponent = 0x1F0; // int32
            public const nint m_fRadCentCore = 0x1F4; // float32
            public const nint m_fRadPerPoint = 0x1F8; // float32
            public const nint m_fRadPerPointTo = 0x1FC; // float32
            public const nint m_fpointAngle = 0x200; // float32
            public const nint m_fsizeOverall = 0x204; // float32
            public const nint m_fRadBias = 0x208; // float32
            public const nint m_fMinRad = 0x20C; // float32
            public const nint m_fDistBias = 0x210; // float32
            public const nint m_bUseLocalCoords = 0x214; // bool
            public const nint m_bUseWithContEmit = 0x215; // bool
            public const nint m_bUseOrigRadius = 0x216; // bool
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
        public static class C_OP_AttractToControlPoint {
            public const nint m_vecComponentScale = 0x1F0; // Vector
            public const nint m_fForceAmount = 0x200; // CPerParticleFloatInput
            public const nint m_fMinimumDistance = 0x378; // CPerParticleFloatInput
            public const nint m_fFalloffPower = 0x4F0; // float32
            public const nint m_TransformInput = 0x4F8; // CParticleTransformInput
            public const nint m_fForceAmountMin = 0x560; // CPerParticleFloatInput
            public const nint m_bApplyMinForce = 0x6D8; // bool
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_INIT_RandomLifeTime {
            public const nint m_fLifetimeMin = 0x1E8; // float32
            public const nint m_fLifetimeMax = 0x1EC; // float32
            public const nint m_fLifetimeRandExponent = 0x1F0; // float32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RemapParticleCountToNamedModelSequenceScalar {
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_VelocityRadialRandom {
            public const nint m_bPerParticleCenter = 0x1E8; // bool
            public const nint m_nControlPointNumber = 0x1EC; // int32
            public const nint m_vecPosition = 0x1F0; // CPerParticleVecInput
            public const nint m_vecFwd = 0x8C8; // CPerParticleVecInput
            public const nint m_fSpeedMin = 0xFA0; // CPerParticleFloatInput
            public const nint m_fSpeedMax = 0x1118; // CPerParticleFloatInput
            public const nint m_vecLocalCoordinateSystemSpeedScale = 0x1290; // Vector
            public const nint m_bIgnoreDelta = 0x129D; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomRadius {
            public const nint m_flRadiusMin = 0x1E8; // float32
            public const nint m_flRadiusMax = 0x1EC; // float32
            public const nint m_flRadiusRandExponent = 0x1F0; // float32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_Orient2DRelToCP {
            public const nint m_flRotOffset = 0x1E0; // float32
            public const nint m_flSpinStrength = 0x1E4; // float32
            public const nint m_nCP = 0x1E8; // int32
            public const nint m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // PARTICLE_TOPOLOGY_LINES
        public static class ControlPointReference_t {
            public const nint m_controlPointNameString = 0x0; // int32
            public const nint m_vOffsetFromControlPoint = 0x4; // Vector
            public const nint m_bOffsetInLocalSpace = 0x10; // bool
        }
        // Parent: None
        // Field count: 20
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_LightningSnapshotGenerator {
            public const nint m_nCPSnapshot = 0x1E8; // int32
            public const nint m_nCPStartPnt = 0x1EC; // int32
            public const nint m_nCPEndPnt = 0x1F0; // int32
            public const nint m_flSegments = 0x1F8; // CParticleCollectionFloatInput
            public const nint m_flOffset = 0x370; // CParticleCollectionFloatInput
            public const nint m_flOffsetDecay = 0x4E8; // CParticleCollectionFloatInput
            public const nint m_flRecalcRate = 0x660; // CParticleCollectionFloatInput
            public const nint m_flUVScale = 0x7D8; // CParticleCollectionFloatInput
            public const nint m_flUVOffset = 0x950; // CParticleCollectionFloatInput
            public const nint m_flSplitRate = 0xAC8; // CParticleCollectionFloatInput
            public const nint m_flRecursionSplitScale = 0xC40; // CParticleCollectionFloatInput
            public const nint m_bScaleBranchDistance = 0xDB8; // bool
            public const nint m_flBranchDistanceScale = 0xDC0; // CParticleCollectionFloatInput
            public const nint m_bScaleBranchOffset = 0xF38; // bool
            public const nint m_flBranchOffsetScale = 0xF40; // CParticleCollectionFloatInput
            public const nint m_flBranchTwist = 0x10B8; // CParticleCollectionFloatInput
            public const nint m_nBranchBehavior = 0x1230; // ParticleLightnintBranchBehavior_t
            public const nint m_flRadiusStart = 0x1238; // CParticleCollectionFloatInput
            public const nint m_flRadiusEnd = 0x13B0; // CParticleCollectionFloatInput
            public const nint m_flDedicatedPool = 0x1528; // CParticleCollectionFloatInput
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        public static class C_OP_RemapNamedModelMeshGroupOnceTimed {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        public static class C_INIT_RemapQAnglesToRotation {
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_SetControlPointFieldToScalarExpression {
            public const nint m_nExpression = 0x1E8; // ScalarExpressionType_t
            public const nint m_flInput1 = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_flInput2 = 0x368; // CParticleCollectionFloatInput
            public const nint m_flOutputRemap = 0x4E0; // CParticleRemapFloatInput
            public const nint m_nOutputCP = 0x658; // int32
            public const nint m_nOutVectorField = 0x65C; // int32
            public const nint m_flInterpolation = 0x660; // CParticleCollectionFloatInput
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MGetKV3ClassDefaults
        public static class C_OP_CreateParticleSystemRenderer {
            public const nint m_hEffect = 0x230; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
            public const nint m_nEventType = 0x238; // EventTypeSelection_t
            public const nint m_vecCPs = 0x240; // CUtlLeanVector<CPAssignment_t>
            public const nint m_szParticleConfig = 0x250; // CUtlString
            public const nint m_AggregationPos = 0x258; // CPerParticleVecInput
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomVectorComponent {
            public const nint m_flMin = 0x1E8; // float32
            public const nint m_flMax = 0x1EC; // float32
            public const nint m_nFieldOutput = 0x1F0; // ParticleAttributeIndex_t
            public const nint m_nComponent = 0x1F4; // int32
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_InheritFromParentParticles {
            public const nint m_flScale = 0x1E0; // float32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nIncrement = 0x1E8; // int32
            public const nint m_bRandomDistribution = 0x1EC; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_SetVectorAttributeToVectorExpression {
            public const nint m_nExpression = 0x1E8; // VectorExpressionType_t
            public const nint m_vInput1 = 0x1F0; // CPerParticleVecInput
            public const nint m_vInput2 = 0x8C8; // CPerParticleVecInput
            public const nint m_flLerp = 0xFA0; // CPerParticleFloatInput
            public const nint m_nOutputField = 0x1118; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x111C; // ParticleSetMethod_t
            public const nint m_bNormalizedOutput = 0x1120; // bool
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        public static class C_OP_RemapTransformVisibilityToVector {
            public const nint m_nSetMethod = 0x1E0; // ParticleSetMethod_t
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
            public const nint m_nFieldOutput = 0x250; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x254; // float32
            public const nint m_flInputMax = 0x258; // float32
            public const nint m_vecOutputMin = 0x25C; // Vector
            public const nint m_vecOutputMax = 0x268; // Vector
            public const nint m_flRadius = 0x274; // float32
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
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DirectionBetweenVecsToVec {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_vecPoint1 = 0x1E8; // CPerParticleVecInput
            public const nint m_vecPoint2 = 0x8C0; // CPerParticleVecInput
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MParticleMinVersion
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_MovementLoopInsideSphere {
            public const nint m_nCP = 0x1E0; // int32
            public const nint m_flDistance = 0x1E8; // CParticleCollectionFloatInput
            public const nint m_vecScale = 0x360; // CParticleCollectionVecInput
            public const nint m_nDistSqrAttr = 0xA38; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 9
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // UNUSED
        // F_NORMAL_MAP
        // F_TEXTURE_LAYERS
        // F_REFRACT_BLUR
        // F_PARTICLE_SHADOWS
        // F_DIRECTIONAL_LIGHTING
        // F_REVERSE_DEPTH
        // F_LIGHTEN_MODE
        public static class C_OP_RenderSimpleModelCollection {
            public const nint m_bCenterOffset = 0x230; // bool
            public const nint m_hModel = 0x238; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_modelInput = 0x240; // CParticleModelInput
            public const nint m_fSizeCullScale = 0x2A0; // CParticleCollectionFloatInput
            public const nint m_bDisableShadows = 0x418; // bool
            public const nint m_bDisableMotionBlur = 0x419; // bool
            public const nint m_bAcceptsDecals = 0x41A; // bool
            public const nint m_fDrawFilter = 0x420; // CPerParticleFloatInput
            public const nint m_nAngularVelocityField = 0x598; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 2
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_InitFloatCollection {
            public const nint m_InputValue = 0x1E8; // CParticleCollectionFloatInput
            public const nint m_nOutputField = 0x360; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 9
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
        public static class CPathParameters {
            public const nint m_nStartControlPointNumber = 0x0; // int32
            public const nint m_nMidControlPointNumber = 0x4; // int32
            public const nint m_nEndControlPointNumber = 0x8; // int32
            public const nint m_nBulgeControl = 0xC; // int32
            public const nint m_flBulge = 0x10; // float32
            public const nint m_flMidPoint = 0x14; // float32
            public const nint m_vStartPointOffset = 0x18; // Vector
            public const nint m_vMidPointOffset = 0x24; // Vector
            public const nint m_vEndOffset = 0x30; // Vector
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapScalarEndCap {
            public const nint m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E8; // float32
            public const nint m_flInputMax = 0x1EC; // float32
            public const nint m_flOutputMin = 0x1F0; // float32
            public const nint m_flOutputMax = 0x1F4; // float32
        }
        // Parent: None
        // Field count: 3
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
        public static class C_INIT_CreateFromPlaneCache {
            public const nint m_vecOffsetMin = 0x1E8; // Vector
            public const nint m_vecOffsetMax = 0x1F4; // Vector
            public const nint m_bUseNormal = 0x201; // bool
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
        // PARTICLE_MASSMODE_RADIUS_SQUARED
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_LazyCullCompareFloat {
            public const nint m_flComparsion1 = 0x1E0; // CPerParticleFloatInput
            public const nint m_flComparsion2 = 0x358; // CPerParticleFloatInput
            public const nint m_flCullTime = 0x4D0; // CPerParticleFloatInput
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_ControlPointToRadialScreenSpace {
            public const nint m_nCPIn = 0x1E8; // int32
            public const nint m_vecCP1Pos = 0x1EC; // Vector
            public const nint m_nCPOut = 0x1F8; // int32
            public const nint m_nCPOutField = 0x1FC; // int32
            public const nint m_nCPSSPosOut = 0x200; // int32
        }
        // Parent: None
        // Field count: 0
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SpinUpdate {
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_INIT_NormalOffset {
            public const nint m_OffsetMin = 0x1E8; // Vector
            public const nint m_OffsetMax = 0x1F4; // Vector
            public const nint m_nControlPointNumber = 0x200; // int32
            public const nint m_bLocalCoords = 0x204; // bool
            public const nint m_bNormalize = 0x205; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_INIT_CreationNoise {
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_bAbsVal = 0x1EC; // bool
            public const nint m_bAbsValInv = 0x1ED; // bool
            public const nint m_flOffset = 0x1F0; // float32
            public const nint m_flOutputMin = 0x1F4; // float32
            public const nint m_flOutputMax = 0x1F8; // float32
            public const nint m_flNoiseScale = 0x1FC; // float32
            public const nint m_flNoiseScaleLoc = 0x200; // float32
            public const nint m_vecOffsetLoc = 0x204; // Vector
            public const nint m_flWorldTimeScale = 0x210; // float32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_Spin {
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        public static class C_OP_GameLiquidSpill {
            public const nint m_flLiquidContentsField = 0x230; // CParticleCollectionFloatInput
            public const nint m_flExpirationTime = 0x3A8; // CParticleCollectionFloatInput
            public const nint m_flRadius = 0x520; // CParticleCollectionFloatInput
            public const nint m_bCheckExposedToSky = 0x698; // bool
            public const nint m_nAmountAttribute = 0x69C; // ParticleAttributeIndex_t
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_ConstrainLineLength {
            public const nint m_flMinDistance = 0x1E0; // float32
            public const nint m_flMaxDistance = 0x1E4; // float32
        }
        // Parent: None
        // Field count: 8
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_LifespanFromVelocity {
            public const nint m_vecComponentScale = 0x1E8; // Vector
            public const nint m_flTraceOffset = 0x1F4; // float32
            public const nint m_flMaxTraceLength = 0x1F8; // float32
            public const nint m_flTraceTolerance = 0x1FC; // float32
            public const nint m_nMaxPlanes = 0x200; // int32
            public const nint m_CollisionGroupName = 0x208; // char[128]
            public const nint m_nTraceSet = 0x288; // ParticleTraceSet_t
            public const nint m_bIncludeWater = 0x298; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_VelocityFromCP {
            public const nint m_velocityInput = 0x1E8; // CParticleCollectionVecInput
            public const nint m_transformInput = 0x8C0; // CParticleTransformInput
            public const nint m_flVelocityScale = 0x928; // float32
            public const nint m_bDirectionOnly = 0x92C; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_MovementSkinnedPositionFromCPSnapshot {
            public const nint m_nSnapshotControlPointNumber = 0x1E0; // int32
            public const nint m_nControlPointNumber = 0x1E4; // int32
            public const nint m_bRandom = 0x1E8; // bool
            public const nint m_nRandomSeed = 0x1EC; // int32
            public const nint m_bSetNormal = 0x1F0; // bool
            public const nint m_bSetRadius = 0x1F1; // bool
            public const nint m_nIndexType = 0x1F4; // SnapshotIndexType_t
            public const nint m_flReadIndex = 0x1F8; // CPerParticleFloatInput
            public const nint m_flIncrement = 0x370; // CParticleCollectionFloatInput
            public const nint m_nFullLoopIncrement = 0x4E8; // CParticleCollectionFloatInput
            public const nint m_nSnapShotStartPoint = 0x660; // CParticleCollectionFloatInput
            public const nint m_flInterpolation = 0x7D8; // CPerParticleFloatInput
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
        public static class C_OP_OscillateVector {
            public const nint m_RateMin = 0x1E0; // Vector
            public const nint m_RateMax = 0x1EC; // Vector
            public const nint m_FrequencyMin = 0x1F8; // Vector
            public const nint m_FrequencyMax = 0x204; // Vector
            public const nint m_nField = 0x210; // ParticleAttributeIndex_t
            public const nint m_bProportional = 0x214; // bool
            public const nint m_bProportionalOp = 0x215; // bool
            public const nint m_bOffset = 0x216; // bool
            public const nint m_flStartTime_min = 0x218; // float32
            public const nint m_flStartTime_max = 0x21C; // float32
            public const nint m_flEndTime_min = 0x220; // float32
            public const nint m_flEndTime_max = 0x224; // float32
            public const nint m_flOscMult = 0x228; // CPerParticleFloatInput
            public const nint m_flOscAdd = 0x3A0; // CPerParticleFloatInput
            public const nint m_flRateScale = 0x518; // CPerParticleFloatInput
        }
        // Parent: None
        // Field count: 15
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_PositionLock {
            public const nint m_TransformInput = 0x1E0; // CParticleTransformInput
            public const nint m_flStartTime_min = 0x248; // float32
            public const nint m_flStartTime_max = 0x24C; // float32
            public const nint m_flStartTime_exp = 0x250; // float32
            public const nint m_flEndTime_min = 0x254; // float32
            public const nint m_flEndTime_max = 0x258; // float32
            public const nint m_flEndTime_exp = 0x25C; // float32
            public const nint m_flRange = 0x260; // float32
            public const nint m_flRangeBias = 0x268; // CParticleCollectionFloatInput
            public const nint m_flJumpThreshold = 0x3E0; // float32
            public const nint m_flPrevPosScale = 0x3E4; // float32
            public const nint m_bLockRot = 0x3E8; // bool
            public const nint m_vecScale = 0x3F0; // CParticleCollectionVecInput
            public const nint m_nFieldOutput = 0xAC8; // ParticleAttributeIndex_t
            public const nint m_nFieldOutputPrev = 0xACC; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RenderVRHapticEvent {
            public const nint m_nHand = 0x230; // ParticleVRHandChoiceList_t
            public const nint m_nOutputHandCP = 0x234; // int32
            public const nint m_nOutputField = 0x238; // int32
            public const nint m_flAmplitude = 0x240; // CPerParticleFloatInput
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_OP_SetControlPointToImpactPoint {
            public const nint m_nCPOut = 0x1E8; // int32
            public const nint m_nCPIn = 0x1EC; // int32
            public const nint m_flUpdateRate = 0x1F0; // float32
            public const nint m_flTraceLength = 0x1F8; // CParticleCollectionFloatInput
            public const nint m_flStartOffset = 0x370; // float32
            public const nint m_flOffset = 0x374; // float32
            public const nint m_vecTraceDir = 0x378; // Vector
            public const nint m_CollisionGroupName = 0x384; // char[128]
            public const nint m_nTraceSet = 0x404; // ParticleTraceSet_t
            public const nint m_bSetToEndpoint = 0x408; // bool
            public const nint m_bTraceToClosestSurface = 0x409; // bool
            public const nint m_bIncludeWater = 0x40A; // bool
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
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_ReinitializeScalarEndCap {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flOutputMin = 0x1E4; // float32
            public const nint m_flOutputMax = 0x1E8; // float32
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
        // MPropertySuppressExpr
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
        public static class C_OP_TurbulenceForce {
            public const nint m_flNoiseCoordScale0 = 0x1F0; // float32
            public const nint m_flNoiseCoordScale1 = 0x1F4; // float32
            public const nint m_flNoiseCoordScale2 = 0x1F8; // float32
            public const nint m_flNoiseCoordScale3 = 0x1FC; // float32
            public const nint m_vecNoiseAmount0 = 0x200; // Vector
            public const nint m_vecNoiseAmount1 = 0x20C; // Vector
            public const nint m_vecNoiseAmount2 = 0x218; // Vector
            public const nint m_vecNoiseAmount3 = 0x224; // Vector
        }
        // Parent: None
        // Field count: 9
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_RemapNamedModelElementOnceTimed {
            public const nint m_hModel = 0x1E0; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_inNames = 0x1E8; // CUtlVector<CUtlString>
            public const nint m_outNames = 0x200; // CUtlVector<CUtlString>
            public const nint m_fallbackNames = 0x218; // CUtlVector<CUtlString>
            public const nint m_bModelFromRenderer = 0x230; // bool
            public const nint m_bProportional = 0x231; // bool
            public const nint m_nFieldInput = 0x234; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x238; // ParticleAttributeIndex_t
            public const nint m_flRemapTime = 0x23C; // float32
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
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        public static class C_OP_SetControlPointToPlayer {
            public const nint m_nCP1 = 0x1E8; // int32
            public const nint m_vecCP1Pos = 0x1EC; // Vector
            public const nint m_bOrientToEyes = 0x1F8; // bool
            public const nint m_nPosition = 0x1FC; // ParticleEntityPos_t
            public const nint m_nRadiusCP = 0x200; // int32
            public const nint m_nRadiusCPField = 0x204; // int32
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_EndCapTimedFreeze {
            public const nint m_flFreezeTime = 0x1E0; // CParticleCollectionFloatInput
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_RenderGpuImplicit {
            public const nint m_bUsePerParticleRadius = 0x230; // bool
            public const nint m_nVertexCountKb = 0x234; // uint32
            public const nint m_nIndexCountKb = 0x238; // uint32
            public const nint m_fGridSize = 0x240; // CParticleCollectionRendererFloatInput
            public const nint m_fRadiusScale = 0x3B8; // CParticleCollectionRendererFloatInput
            public const nint m_fIsosurfaceThreshold = 0x530; // CParticleCollectionRendererFloatInput
            public const nint m_nScaleCP = 0x6A8; // int32
            public const nint m_hMaterial = 0x6B0; // CStrongHandle<InfoForResourceTypeIMaterial2>
        }
        // Parent: None
        // Field count: 15
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyAttributeEditor
        // MPropertyEditContextOverrideKey
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RenderVolumetricEmitter {
            public const nint m_strChannelType = 0x230; // CUtlString
            public const nint m_nType = 0x238; // ParticleVolumetricSmokeType_t
            public const nint m_nCreationType = 0x23C; // ParticleVolumetricSmokeCreationType_t
            public const nint m_nEventType = 0x240; // EventTypeSelection_t
            public const nint m_vecPos = 0x248; // CPerParticleVecInput
            public const nint m_vecVelocity = 0x920; // CPerParticleVecInput
            public const nint m_vPrevPosition = 0xFF8; // CPerParticleVecInput
            public const nint m_flSpeed = 0x16D0; // CPerParticleFloatInput
            public const nint m_flRadius = 0x1848; // CPerParticleFloatInput
            public const nint m_flDensity = 0x19C0; // CPerParticleFloatInput
            public const nint m_flTemperature = 0x1B38; // CPerParticleFloatInput
            public const nint m_flMagnitude = 0x1CB0; // CPerParticleFloatInput
            public const nint m_flKillRadius = 0x1E28; // CPerParticleFloatInput
            public const nint m_flKillDensityScale = 0x1FA0; // CPerParticleFloatInput
            public const nint m_flFalloff = 0x2118; // CPerParticleFloatInput
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        public static class C_OP_RemapTransformVisibilityToScalar {
            public const nint m_nSetMethod = 0x1E0; // ParticleSetMethod_t
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
            public const nint m_nFieldOutput = 0x250; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x254; // float32
            public const nint m_flInputMax = 0x258; // float32
            public const nint m_flOutputMin = 0x25C; // float32
            public const nint m_flOutputMax = 0x260; // float32
            public const nint m_flRadius = 0x264; // float32
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
        public static class C_OP_RemapControlPointDirectionToVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flScale = 0x1E4; // float32
            public const nint m_nControlPointNumber = 0x1E8; // int32
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_OP_ScreenSpacePositionOfTarget {
            public const nint m_vecTargetPosition = 0x1E0; // CPerParticleVecInput
            public const nint m_bOututBehindness = 0x8B8; // bool
            public const nint m_nBehindFieldOutput = 0x8BC; // ParticleAttributeIndex_t
            public const nint m_flBehindOutputRemap = 0x8C0; // CParticleRemapFloatInput
            public const nint m_nBehindSetMethod = 0xA38; // ParticleSetMethod_t
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
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DragRelativeToPlane {
            public const nint m_flDragAtPlane = 0x1E0; // CParticleCollectionFloatInput
            public const nint m_flFalloff = 0x358; // CParticleCollectionFloatInput
            public const nint m_bDirectional = 0x4D0; // bool
            public const nint m_vecPlaneNormal = 0x4D8; // CParticleCollectionVecInput
            public const nint m_nControlPointNumber = 0xBB0; // int32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // PARTICLE_MASSMODE_RADIUS_SQUARED
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_SetCPtoVector {
            public const nint m_nCPInput = 0x1E0; // int32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        public static class C_INIT_RandomYaw {
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SnapshotRigidSkinToBones {
            public const nint m_bTransformNormals = 0x1E0; // bool
            public const nint m_bTransformRadii = 0x1E1; // bool
            public const nint m_nControlPointNumber = 0x1E4; // int32
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
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetSingleControlPointPosition {
            public const nint m_bSetOnce = 0x1E8; // bool
            public const nint m_nCP1 = 0x1EC; // int32
            public const nint m_vecCP1Pos = 0x1F0; // CParticleCollectionVecInput
            public const nint m_transformInput = 0x8C8; // CParticleTransformInput
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapCPtoScalar {
            public const nint m_nCPInput = 0x1E0; // int32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nField = 0x1E8; // int32
            public const nint m_flInputMin = 0x1EC; // float32
            public const nint m_flInputMax = 0x1F0; // float32
            public const nint m_flOutputMin = 0x1F4; // float32
            public const nint m_flOutputMax = 0x1F8; // float32
            public const nint m_flStartTime = 0x1FC; // float32
            public const nint m_flEndTime = 0x200; // float32
            public const nint m_flInterpRate = 0x204; // float32
            public const nint m_nSetMethod = 0x208; // ParticleSetMethod_t
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
        // MPropertyFriendlyName
        public static class C_OP_RemapNamedModelMeshGroupEndCap {
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_PercentageBetweenTransformsVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E4; // float32
            public const nint m_flInputMax = 0x1E8; // float32
            public const nint m_vecOutputMin = 0x1EC; // Vector
            public const nint m_vecOutputMax = 0x1F8; // Vector
            public const nint m_TransformStart = 0x208; // CParticleTransformInput
            public const nint m_TransformEnd = 0x270; // CParticleTransformInput
            public const nint m_nSetMethod = 0x2D8; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x2DC; // bool
            public const nint m_bRadialCheck = 0x2DD; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySortPriority
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        public static class C_OP_RenderScreenVelocityRotate {
            public const nint m_flRotateRateDegrees = 0x230; // float32
            public const nint m_flForwardDegrees = 0x234; // float32
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_UpdateLightSource {
            public const nint m_vColorTint = 0x1E0; // Color
            public const nint m_flBrightnessScale = 0x1E4; // float32
            public const nint m_flRadiusScale = 0x1E8; // float32
            public const nint m_flMinimumLightingRadius = 0x1EC; // float32
            public const nint m_flMaximumLightingRadius = 0x1F0; // float32
            public const nint m_flPositionDampingConstant = 0x1F4; // float32
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_INIT_CreateWithinBox {
            public const nint m_vecMin = 0x1E8; // CPerParticleVecInput
            public const nint m_vecMax = 0x8C0; // CPerParticleVecInput
            public const nint m_nControlPointNumber = 0xF98; // int32
            public const nint m_bLocalSpace = 0xF9C; // bool
            public const nint m_randomnessParameters = 0xFA0; // CRandomNumberGeneratorParameters
            public const nint m_bUseNewCode = 0xFA8; // bool
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_ChooseRandomChildrenInGroup {
            public const nint m_nChildGroupID = 0x1E8; // int32
            public const nint m_flNumberOfChildren = 0x1F0; // CParticleCollectionFloatInput
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_ControlpointLight {
            public const nint m_flScale = 0x1E0; // float32
            public const nint m_nControlPoint1 = 0x670; // int32
            public const nint m_nControlPoint2 = 0x674; // int32
            public const nint m_nControlPoint3 = 0x678; // int32
            public const nint m_nControlPoint4 = 0x67C; // int32
            public const nint m_vecCPOffset1 = 0x680; // Vector
            public const nint m_vecCPOffset2 = 0x68C; // Vector
            public const nint m_vecCPOffset3 = 0x698; // Vector
            public const nint m_vecCPOffset4 = 0x6A4; // Vector
            public const nint m_LightFiftyDist1 = 0x6B0; // float32
            public const nint m_LightZeroDist1 = 0x6B4; // float32
            public const nint m_LightFiftyDist2 = 0x6B8; // float32
            public const nint m_LightZeroDist2 = 0x6BC; // float32
            public const nint m_LightFiftyDist3 = 0x6C0; // float32
            public const nint m_LightZeroDist3 = 0x6C4; // float32
            public const nint m_LightFiftyDist4 = 0x6C8; // float32
            public const nint m_LightZeroDist4 = 0x6CC; // float32
            public const nint m_LightColor1 = 0x6D0; // Color
            public const nint m_LightColor2 = 0x6D4; // Color
            public const nint m_LightColor3 = 0x6D8; // Color
            public const nint m_LightColor4 = 0x6DC; // Color
            public const nint m_bLightType1 = 0x6E0; // bool
            public const nint m_bLightType2 = 0x6E1; // bool
            public const nint m_bLightType3 = 0x6E2; // bool
            public const nint m_bLightType4 = 0x6E3; // bool
            public const nint m_bLightDynamic1 = 0x6E4; // bool
            public const nint m_bLightDynamic2 = 0x6E5; // bool
            public const nint m_bLightDynamic3 = 0x6E6; // bool
            public const nint m_bLightDynamic4 = 0x6E7; // bool
            public const nint m_bUseNormal = 0x6E8; // bool
            public const nint m_bUseHLambert = 0x6E9; // bool
            public const nint m_bClampLowerRange = 0x6EE; // bool
            public const nint m_bClampUpperRange = 0x6EF; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_VectorFieldSnapshot {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_nAttributeToWrite = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nLocalSpaceCP = 0x1E8; // int32
            public const nint m_flInterpolation = 0x1F0; // CPerParticleFloatInput
            public const nint m_vecScale = 0x368; // CPerParticleVecInput
            public const nint m_flBoundaryDampening = 0xA40; // float32
            public const nint m_bSetVelocity = 0xA44; // bool
            public const nint m_bLockToSurface = 0xA45; // bool
            public const nint m_flGridSpacing = 0xA48; // float32
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_CylindricalDistanceToTransform {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E8; // CPerParticleFloatInput
            public const nint m_flInputMax = 0x360; // CPerParticleFloatInput
            public const nint m_flOutputMin = 0x4D8; // CPerParticleFloatInput
            public const nint m_flOutputMax = 0x650; // CPerParticleFloatInput
            public const nint m_TransformStart = 0x7C8; // CParticleTransformInput
            public const nint m_TransformEnd = 0x830; // CParticleTransformInput
            public const nint m_nSetMethod = 0x898; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x89C; // bool
            public const nint m_bAdditive = 0x89D; // bool
            public const nint m_bCapsule = 0x89E; // bool
        }
        // Parent: None
        // Field count: 15
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_PositionPlaceOnGround {
            public const nint m_flOffset = 0x1E8; // CPerParticleFloatInput
            public const nint m_flMaxTraceLength = 0x360; // CPerParticleFloatInput
            public const nint m_vecTraceDir = 0x4D8; // CPerParticleVecInput
            public const nint m_CollisionGroupName = 0xBB0; // char[128]
            public const nint m_nTraceSet = 0xC30; // ParticleTraceSet_t
            public const nint m_nTraceMissBehavior = 0xC40; // ParticleTraceMissBehavior_t
            public const nint m_bIncludeWater = 0xC44; // bool
            public const nint m_nAttribute = 0xC48; // ParticleAttributeIndex_t
            public const nint m_bSetPXYZOnly = 0xC4C; // bool
            public const nint m_bSetNormal = 0xC4D; // bool
            public const nint m_nGroundNormalAttribute = 0xC50; // ParticleAttributeIndex_t
            public const nint m_bOffsetonColOnly = 0xC54; // bool
            public const nint m_flOffsetByRadiusFactor = 0xC58; // float32
            public const nint m_nPreserveOffsetCP = 0xC5C; // int32
            public const nint m_nIgnoreCP = 0xC60; // int32
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomScalar {
            public const nint m_flMin = 0x1E8; // float32
            public const nint m_flMax = 0x1EC; // float32
            public const nint m_flExponent = 0x1F0; // float32
            public const nint m_nFieldOutput = 0x1F4; // ParticleAttributeIndex_t
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
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyAttributeEditor
        // MPropertyEditContextOverrideKey
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_OP_RenderPostProcessing {
            public const nint m_flPostProcessStrength = 0x230; // CPerParticleFloatInput
            public const nint m_hPostTexture = 0x3A8; // CStrongHandle<InfoForResourceTypeCPostProcessingResource>
            public const nint m_nPriority = 0x3B0; // ParticlePostProcessPriorityGroup_t
        }
        // Parent: None
        // Field count: 13
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_OscillateScalar {
            public const nint m_RateMin = 0x1E0; // float32
            public const nint m_RateMax = 0x1E4; // float32
            public const nint m_FrequencyMin = 0x1E8; // float32
            public const nint m_FrequencyMax = 0x1EC; // float32
            public const nint m_nField = 0x1F0; // ParticleAttributeIndex_t
            public const nint m_bProportional = 0x1F4; // bool
            public const nint m_bProportionalOp = 0x1F5; // bool
            public const nint m_flStartTime_min = 0x1F8; // float32
            public const nint m_flStartTime_max = 0x1FC; // float32
            public const nint m_flEndTime_min = 0x200; // float32
            public const nint m_flEndTime_max = 0x204; // float32
            public const nint m_flOscMult = 0x208; // float32
            public const nint m_flOscAdd = 0x20C; // float32
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
        // MPropertyAttributeChoiceName
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_FadeOut {
            public const nint m_flFadeOutTimeMin = 0x1E0; // float32
            public const nint m_flFadeOutTimeMax = 0x1E4; // float32
            public const nint m_flFadeOutTimeExp = 0x1E8; // float32
            public const nint m_flFadeBias = 0x1EC; // float32
            public const nint m_bProportional = 0x220; // bool
            public const nint m_bEaseInAndOut = 0x221; // bool
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
        // MPropertySortPriority
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_WaterImpulseRenderer {
            public const nint m_vecPos = 0x230; // CPerParticleVecInput
            public const nint m_flRadius = 0x908; // CPerParticleFloatInput
            public const nint m_flMagnitude = 0xA80; // CPerParticleFloatInput
            public const nint m_flShape = 0xBF8; // CPerParticleFloatInput
            public const nint m_flWindSpeed = 0xD70; // CPerParticleFloatInput
            public const nint m_flWobble = 0xEE8; // CPerParticleFloatInput
            public const nint m_bIsRadialWind = 0x1060; // bool
            public const nint m_nEventType = 0x1064; // EventTypeSelection_t
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RampScalarSplineSimple {
            public const nint m_Rate = 0x1E0; // float32
            public const nint m_flStartTime = 0x1E4; // float32
            public const nint m_flEndTime = 0x1E8; // float32
            public const nint m_nField = 0x210; // ParticleAttributeIndex_t
            public const nint m_bEaseOut = 0x214; // bool
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_INIT_DistanceCull {
            public const nint m_nControlPoint = 0x1E8; // int32
            public const nint m_flDistance = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_bCullInside = 0x368; // bool
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_InitFromVectorFieldSnapshot {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_nLocalSpaceCP = 0x1EC; // int32
            public const nint m_nWeightUpdateCP = 0x1F0; // int32
            public const nint m_bUseVerticalVelocity = 0x1F4; // bool
            public const nint m_vecScale = 0x1F8; // CPerParticleVecInput
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
        // MPropertyAttributeRange
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_SetVectorAttributeToVectorExpression {
            public const nint m_nExpression = 0x1E0; // VectorExpressionType_t
            public const nint m_vInput1 = 0x1E8; // CPerParticleVecInput
            public const nint m_vInput2 = 0x8C0; // CPerParticleVecInput
            public const nint m_flLerp = 0xF98; // CPerParticleFloatInput
            public const nint m_nOutputField = 0x1110; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x1114; // ParticleSetMethod_t
            public const nint m_bNormalizedOutput = 0x1118; // bool
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
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
        // MPropertyAttributeEditor
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_AddVectorToVector {
            public const nint m_vecScale = 0x1E8; // Vector
            public const nint m_nFieldOutput = 0x1F4; // ParticleAttributeIndex_t
            public const nint m_nFieldInput = 0x1F8; // ParticleAttributeIndex_t
            public const nint m_vOffsetMin = 0x1FC; // Vector
            public const nint m_vOffsetMax = 0x208; // Vector
            public const nint m_randomnessParameters = 0x214; // CRandomNumberGeneratorParameters
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
        public static class C_INIT_RemapInitialVisibilityScalar {
            public const nint m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1F0; // float32
            public const nint m_flInputMax = 0x1F4; // float32
            public const nint m_flOutputMin = 0x1F8; // float32
            public const nint m_flOutputMax = 0x1FC; // float32
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
        // MPropertyAttributeChoiceName
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
        // MVectorIsSometimesCoordinate
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
        public static class C_OP_RemapTransformOrientationToYaw {
            public const nint m_TransformInput = 0x1E0; // CParticleTransformInput
            public const nint m_nFieldOutput = 0x248; // ParticleAttributeIndex_t
            public const nint m_flRotOffset = 0x24C; // float32
            public const nint m_flSpinStrength = 0x250; // float32
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySortPriority
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RenderStatusEffect {
            public const nint m_pTextureColorWarp = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureDetail2 = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureDiffuseWarp = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureFresnelColorWarp = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureFresnelWarp = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureSpecularWarp = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureEnvMap = 0x260; // CStrongHandle<InfoForResourceTypeCTextureBase>
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
        // MPropertySuppressExpr
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
        public static class C_OP_RandomForce {
            public const nint m_MinForce = 0x1F0; // Vector
            public const nint m_MaxForce = 0x1FC; // Vector
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_RemapParticleCountOnScalarEndCap {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nInputMin = 0x1E4; // int32
            public const nint m_nInputMax = 0x1E8; // int32
            public const nint m_flOutputMin = 0x1EC; // float32
            public const nint m_flOutputMax = 0x1F0; // float32
            public const nint m_bBackwards = 0x1F4; // bool
            public const nint m_nSetMethod = 0x1F8; // ParticleSetMethod_t
        }
        // Parent: None
        // Field count: 18
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // INHERITABLE_BOOL_FALSE
        // INHERITABLE_BOOL_TRUE
        public static class ParticlePreviewState_t {
            public const nint m_previewModel = 0x0; // CUtlString
            public const nint m_nModSpecificData = 0x8; // uint32
            public const nint m_groundType = 0xC; // PetGroundType_t
            public const nint m_sequenceName = 0x10; // CUtlString
            public const nint m_nFireParticleOnSequenceFrame = 0x18; // int32
            public const nint m_hitboxSetName = 0x20; // CUtlString
            public const nint m_materialGroupName = 0x28; // CUtlString
            public const nint m_vecBodyGroups = 0x30; // CUtlVector<ParticlePreviewBodyGroup_t>
            public const nint m_flPlaybackSpeed = 0x48; // float32
            public const nint m_flParticleSimulationRate = 0x4C; // float32
            public const nint m_bShouldDrawHitboxes = 0x50; // bool
            public const nint m_bShouldDrawAttachments = 0x51; // bool
            public const nint m_bShouldDrawAttachmentNames = 0x52; // bool
            public const nint m_bShouldDrawControlPointAxes = 0x53; // bool
            public const nint m_bAnimationNonLooping = 0x54; // bool
            public const nint m_bSequenceNameIsAnimClipPath = 0x55; // bool
            public const nint m_vecPreviewGravity = 0x58; // Vector
            public const nint m_vecPreviewWind = 0x64; // Vector
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_LocalAccelerationForce {
            public const nint m_nCP = 0x1F0; // int32
            public const nint m_nScaleCP = 0x1F4; // int32
            public const nint m_vecAccel = 0x1F8; // CParticleCollectionVecInput
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_ModelCull {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_bBoundBox = 0x1E4; // bool
            public const nint m_bCullOutside = 0x1E5; // bool
            public const nint m_bUseBones = 0x1E6; // bool
            public const nint m_HitboxSetName = 0x1E7; // char[128]
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        public static class C_OP_SetFloat {
            public const nint m_InputValue = 0x1E0; // CPerParticleFloatInput
            public const nint m_nOutputField = 0x358; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x35C; // ParticleSetMethod_t
            public const nint m_Lerp = 0x360; // CPerParticleFloatInput
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 13
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RemapTransformToVector {
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_vInputMin = 0x1EC; // Vector
            public const nint m_vInputMax = 0x1F8; // Vector
            public const nint m_vOutputMin = 0x204; // Vector
            public const nint m_vOutputMax = 0x210; // Vector
            public const nint m_TransformInput = 0x220; // CParticleTransformInput
            public const nint m_LocalSpaceTransform = 0x288; // CParticleTransformInput
            public const nint m_flStartTime = 0x2F0; // float32
            public const nint m_flEndTime = 0x2F4; // float32
            public const nint m_nSetMethod = 0x2F8; // ParticleSetMethod_t
            public const nint m_bOffset = 0x2FC; // bool
            public const nint m_bAccelerate = 0x2FD; // bool
            public const nint m_flRemapBias = 0x300; // float32
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
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_ScreenSpaceDistanceToEdge {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flMaxDistFromEdge = 0x1E8; // CPerParticleFloatInput
            public const nint m_flOutputRemap = 0x360; // CParticleRemapFloatInput
            public const nint m_nSetMethod = 0x4D8; // ParticleSetMethod_t
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        public static class C_OP_RenderStatusEffectTf {
            public const nint m_pTextureColorWarp = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureNormal = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureMetalness = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureRoughness = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureSelfIllum = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureDetail = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureEnvMap = 0x260; // CStrongHandle<InfoForResourceTypeCTextureBase>
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_RemapVectortoCP {
            public const nint m_nOutControlPointNumber = 0x1E0; // int32
            public const nint m_nFieldInput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nParticleNumber = 0x1E8; // int32
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
        // MPropertyAttributeRange
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_SetFromCPSnapshot {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_strSnapshotSubset = 0x1E8; // CUtlString
            public const nint m_nAttributeToRead = 0x1F0; // ParticleAttributeIndex_t
            public const nint m_nAttributeToWrite = 0x1F4; // ParticleAttributeIndex_t
            public const nint m_nLocalSpaceCP = 0x1F8; // int32
            public const nint m_bRandom = 0x1FC; // bool
            public const nint m_bReverse = 0x1FD; // bool
            public const nint m_nRandomSeed = 0x200; // int32
            public const nint m_nSnapShotStartPoint = 0x208; // CParticleCollectionFloatInput
            public const nint m_nSnapShotIncrement = 0x380; // CParticleCollectionFloatInput
            public const nint m_flInterpolation = 0x4F8; // CPerParticleFloatInput
            public const nint m_bSubSample = 0x670; // bool
            public const nint m_bPrev = 0x671; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_DistanceBetweenCPsToCP {
            public const nint m_nStartCP = 0x1E8; // int32
            public const nint m_nEndCP = 0x1EC; // int32
            public const nint m_nOutputCP = 0x1F0; // int32
            public const nint m_nOutputCPField = 0x1F4; // int32
            public const nint m_bSetOnce = 0x1F8; // bool
            public const nint m_flInputMin = 0x1FC; // float32
            public const nint m_flInputMax = 0x200; // float32
            public const nint m_flOutputMin = 0x204; // float32
            public const nint m_flOutputMax = 0x208; // float32
            public const nint m_flMaxTraceLength = 0x20C; // float32
            public const nint m_flLOSScale = 0x210; // float32
            public const nint m_bLOS = 0x214; // bool
            public const nint m_CollisionGroupName = 0x215; // char[128]
            public const nint m_nTraceSet = 0x298; // ParticleTraceSet_t
            public const nint m_nSetParent = 0x29C; // ParticleParentSetMode_t
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
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
        public static class C_OP_SetControlPointToHand {
            public const nint m_nCP1 = 0x1E8; // int32
            public const nint m_nHand = 0x1EC; // int32
            public const nint m_vecCP1Pos = 0x1F0; // Vector
            public const nint m_bOrientToHand = 0x1FC; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_DistanceCull {
            public const nint m_nControlPoint = 0x1E0; // int32
            public const nint m_vecPointOffset = 0x1E4; // Vector
            public const nint m_flDistance = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_bCullInside = 0x368; // bool
            public const nint m_nAttribute = 0x36C; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CreateAlongPath {
            public const nint m_fMaxDistance = 0x1E8; // CPerParticleFloatInput
            public const nint m_fT = 0x360; // CPerParticleFloatInput
            public const nint m_PathParams = 0x4E0; // CPathParameters
            public const nint m_bUseRandomCPs = 0x520; // bool
            public const nint m_vEndOffset = 0x524; // Vector
            public const nint m_bSaveOffset = 0x530; // bool
            public const nint m_nXCount = 0x1E8; // CParticleCollectionFloatInput
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        public static class C_OP_SetControlPointsToModelParticles {
            public const nint m_HitboxSetName = 0x1E0; // char[128]
            public const nint m_AttachmentName = 0x260; // char[128]
            public const nint m_nFirstControlPoint = 0x2E0; // int32
            public const nint m_nNumControlPoints = 0x2E4; // int32
            public const nint m_nFirstSourcePoint = 0x2E8; // int32
            public const nint m_bSkin = 0x2EC; // bool
            public const nint m_bAttachment = 0x2ED; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_ColorInterpolateRandom {
            public const nint m_ColorFadeMin = 0x1E0; // Color
            public const nint m_ColorFadeMax = 0x1FC; // Color
            public const nint m_flFadeStartTime = 0x20C; // float32
            public const nint m_flFadeEndTime = 0x210; // float32
            public const nint m_nFieldOutput = 0x214; // ParticleAttributeIndex_t
            public const nint m_bEaseInOut = 0x218; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        public static class C_INIT_RemapNamedModelSequenceToScalar {
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyAttributeEditor
        // MPropertyEditContextOverrideKey
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_OP_RenderLights {
            public const nint m_flAnimationRate = 0x238; // float32
            public const nint m_nAnimationType = 0x23C; // AnimationType_t
            public const nint m_bAnimateInFPS = 0x240; // bool
            public const nint m_flMinSize = 0x244; // float32
            public const nint m_flMaxSize = 0x248; // float32
            public const nint m_flStartFadeSize = 0x24C; // float32
            public const nint m_flEndFadeSize = 0x250; // float32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DecayClampCount {
            public const nint m_nCount = 0x1E0; // CParticleCollectionFloatInput
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CRandomNumberGeneratorParameters {
            public const nint m_bDistributeEvenly = 0x0; // bool
            public const nint m_nSeed = 0x4; // int32
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
        public static class C_INIT_ColorLitPerParticle {
            public const nint m_ColorMin = 0x200; // Color
            public const nint m_ColorMax = 0x204; // Color
            public const nint m_TintMin = 0x208; // Color
            public const nint m_TintMax = 0x20C; // Color
            public const nint m_flTintPerc = 0x210; // float32
            public const nint m_nTintBlendMode = 0x214; // ParticleColorBlendMode_t
            public const nint m_flLightAmplification = 0x218; // float32
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CreateOnGrid {
            public const nint m_nXCount = 0x1E8; // CParticleCollectionFloatInput
            public const nint m_nYCount = 0x360; // CParticleCollectionFloatInput
            public const nint m_nZCount = 0x4D8; // CParticleCollectionFloatInput
            public const nint m_nXSpacing = 0x650; // CParticleCollectionFloatInput
            public const nint m_nYSpacing = 0x7C8; // CParticleCollectionFloatInput
            public const nint m_nZSpacing = 0x940; // CParticleCollectionFloatInput
            public const nint m_nControlPointNumber = 0xAB8; // int32
            public const nint m_bLocalSpace = 0xABC; // bool
            public const nint m_bCenter = 0xABD; // bool
            public const nint m_bHollow = 0xABE; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_RampCPLinearRandom {
            public const nint m_nOutControlPointNumber = 0x1E8; // int32
            public const nint m_vecRateMin = 0x1EC; // Vector
            public const nint m_vecRateMax = 0x1F8; // Vector
        }
        // Parent: None
        // Field count: 6
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_VelocityMatchingForce {
            public const nint m_flDirScale = 0x1E0; // float32
            public const nint m_flSpdScale = 0x1E4; // float32
            public const nint m_flNeighborDistance = 0x1E8; // float32
            public const nint m_flFacingStrength = 0x1EC; // float32
            public const nint m_bUseAABB = 0x1F0; // bool
            public const nint m_nCPBroadcast = 0x1F4; // int32
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_INIT_RandomAlphaWindowThreshold {
            public const nint m_flMin = 0x1E8; // float32
            public const nint m_flMax = 0x1EC; // float32
            public const nint m_flExponent = 0x1F0; // float32
        }
        // Parent: None
        // Field count: 14
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_CreateOnModelAtHeight {
            public const nint m_bUseBones = 0x1E8; // bool
            public const nint m_bForceZ = 0x1E9; // bool
            public const nint m_nControlPointNumber = 0x1EC; // int32
            public const nint m_nHeightCP = 0x1F0; // int32
            public const nint m_bUseWaterHeight = 0x1F4; // bool
            public const nint m_flDesiredHeight = 0x1F8; // CParticleCollectionFloatInput
            public const nint m_vecHitBoxScale = 0x370; // CParticleCollectionVecInput
            public const nint m_vecDirectionBias = 0xA48; // CParticleCollectionVecInput
            public const nint m_nBiasType = 0x1120; // ParticleHitboxBiasType_t
            public const nint m_bLocalCoords = 0x1124; // bool
            public const nint m_bPreferMovingBoxes = 0x1125; // bool
            public const nint m_HitboxSetName = 0x1126; // char[128]
            public const nint m_flHitboxVelocityScale = 0x11A8; // CParticleCollectionFloatInput
            public const nint m_flMaxBoneVelocity = 0x1320; // CParticleCollectionFloatInput
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        public static class C_OP_ModelSurfaceSnapshotGenerator {
            public const nint m_nCPSnapshot = 0x1E8; // int32
            public const nint m_modelInput = 0x1F0; // CParticleModelInput
            public const nint m_flRecalcRate = 0x250; // CParticleCollectionFloatInput
            public const nint m_flUSpacing = 0x3C8; // CParticleCollectionFloatInput
            public const nint m_flVSpacing = 0x540; // CParticleCollectionFloatInput
            public const nint m_flSurfaceOffset = 0x6B8; // CParticleCollectionFloatInput
            public const nint m_bSetNormal = 0x830; // bool
            public const nint m_bSetUp = 0x831; // bool
            public const nint m_bSetGravity = 0x832; // bool
            public const nint m_bSetUV = 0x833; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_RestartAfterDuration {
            public const nint m_flDurationMin = 0x1E0; // float32
            public const nint m_flDurationMax = 0x1E4; // float32
            public const nint m_nCP = 0x1E8; // int32
            public const nint m_nCPField = 0x1EC; // int32
            public const nint m_nChildGroupID = 0x1F0; // int32
            public const nint m_bOnlyChildren = 0x1F4; // bool
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        public static class C_OP_RenderClothForce {
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RemapVisibilityScalar {
            public const nint m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E8; // float32
            public const nint m_flInputMax = 0x1EC; // float32
            public const nint m_flOutputMin = 0x1F0; // float32
            public const nint m_flOutputMax = 0x1F4; // float32
            public const nint m_flRadiusScale = 0x1F8; // float32
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        public static class C_INIT_CreateSequentialPathV2 {
            public const nint m_fMaxDistance = 0x1E8; // CPerParticleFloatInput
            public const nint m_flNumToAssign = 0x360; // CParticleCollectionFloatInput
            public const nint m_bLoop = 0x4D8; // bool
            public const nint m_bCPPairs = 0x4D9; // bool
            public const nint m_bSaveOffset = 0x4DA; // bool
            public const nint m_PathParams = 0x4E0; // CPathParameters
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        public static class VecInputMaterialVariable_t {
            public const nint m_strVariable = 0x0; // CUtlString
            public const nint m_vecInput = 0x8; // CParticleCollectionVecInput
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_RemapInitialDirectionToTransformToVector {
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
            public const nint m_nFieldOutput = 0x250; // ParticleAttributeIndex_t
            public const nint m_flScale = 0x254; // float32
            public const nint m_flOffsetRot = 0x258; // float32
            public const nint m_vecOffsetAxis = 0x25C; // Vector
            public const nint m_bNormalize = 0x268; // bool
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MParticleMinVersion
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_LockToSavedSequentialPathV2 {
            public const nint m_flFadeStart = 0x1E0; // float32
            public const nint m_flFadeEnd = 0x1E4; // float32
            public const nint m_bCPPairs = 0x1E8; // bool
            public const nint m_PathParams = 0x1F0; // CPathParameters
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_NormalLock {
            public const nint m_nControlPointNumber = 0x1E0; // int32
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
        // MPropertySuppressExpr
        public static class C_INIT_RemapTransformOrientationToRotations {
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
            public const nint m_vecRotation = 0x250; // Vector
            public const nint m_bUseQuat = 0x25C; // bool
            public const nint m_bWriteNormal = 0x25D; // bool
        }
        // Parent: None
        // Field count: 4
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_Cull {
            public const nint m_flCullPerc = 0x1E0; // float32
            public const nint m_flCullStart = 0x1E4; // float32
            public const nint m_flCullEnd = 0x1E8; // float32
            public const nint m_flCullExp = 0x1EC; // float32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class SequenceWeightedList_t {
            public const nint m_nSequence = 0x0; // int32
            public const nint m_flRelativeWeight = 0x4; // float32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        public static class C_OP_ReadFromNeighboringParticle {
            public const nint m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nIncrement = 0x1E8; // int32
            public const nint m_DistanceCheck = 0x1F0; // CPerParticleFloatInput
            public const nint m_flInterpolation = 0x368; // CPerParticleFloatInput
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySortPriority
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RenderText {
            public const nint m_OutlineColor = 0x230; // Color
            public const nint m_DefaultText = 0x238; // CUtlString
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_LerpToInitialPosition {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_flInterpolation = 0x1E8; // CPerParticleFloatInput
            public const nint m_nCacheField = 0x360; // ParticleAttributeIndex_t
            public const nint m_flScale = 0x368; // CParticleCollectionFloatInput
            public const nint m_vecScale = 0x4E0; // CParticleCollectionVecInput
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomRotation {
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        public static class C_OP_LerpEndCapVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_vecOutput = 0x1E4; // Vector
            public const nint m_flLerpTime = 0x1F0; // float32
        }
        // Parent: None
        // Field count: 1
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_VelocityDecay {
            public const nint m_flMinVelocity = 0x1E0; // float32
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetCPOrientationToPointAtCP {
            public const nint m_nInputCP = 0x1E8; // int32
            public const nint m_nOutputCP = 0x1EC; // int32
            public const nint m_flInterpolation = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_b2DOrientation = 0x368; // bool
            public const nint m_bAvoidSingularity = 0x369; // bool
            public const nint m_bPointAway = 0x36A; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_LockToPointList {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_pointList = 0x1E8; // CUtlVector<PointDefinition_t>
            public const nint m_bPlaceAlongPath = 0x200; // bool
            public const nint m_bClosedLoop = 0x201; // bool
            public const nint m_nNumPointsAlongPath = 0x204; // int32
        }
        // Parent: None
        // Field count: 18
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_MovementPlaceOnGround {
            public const nint m_flOffset = 0x1E0; // CPerParticleFloatInput
            public const nint m_flMaxTraceLength = 0x358; // float32
            public const nint m_flTolerance = 0x35C; // float32
            public const nint m_vecTraceDir = 0x360; // CPerParticleVecInput
            public const nint m_flTraceOffset = 0xA38; // float32
            public const nint m_flLerpRate = 0xA3C; // float32
            public const nint m_CollisionGroupName = 0xA40; // char[128]
            public const nint m_nTraceSet = 0xAC0; // ParticleTraceSet_t
            public const nint m_nRefCP1 = 0xAC4; // int32
            public const nint m_nRefCP2 = 0xAC8; // int32
            public const nint m_nLerpCP = 0xACC; // int32
            public const nint m_nTraceMissBehavior = 0xAD8; // ParticleTraceMissBehavior_t
            public const nint m_bIncludeShotHull = 0xADC; // bool
            public const nint m_bIncludeWater = 0xADD; // bool
            public const nint m_bSetNormal = 0xAE0; // bool
            public const nint m_bScaleOffset = 0xAE1; // bool
            public const nint m_nPreserveOffsetCP = 0xAE4; // int32
            public const nint m_nIgnoreCP = 0xAE8; // int32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetCPOrientationToDirection {
            public const nint m_nInputControlPoint = 0x1E0; // int32
            public const nint m_nOutputControlPoint = 0x1E4; // int32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_OP_RemapCrossProductOfTwoVectorsToVector {
            public const nint m_InputVec1 = 0x1E0; // CPerParticleVecInput
            public const nint m_InputVec2 = 0x8B8; // CPerParticleVecInput
            public const nint m_nFieldOutput = 0xF90; // ParticleAttributeIndex_t
            public const nint m_bNormalize = 0xF94; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapTransformOrientationToRotations {
            public const nint m_TransformInput = 0x1E0; // CParticleTransformInput
            public const nint m_vecRotation = 0x248; // Vector
            public const nint m_bUseQuat = 0x254; // bool
            public const nint m_bWriteNormal = 0x255; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RandomRotationSpeed {
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MParticleMinVersion
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_InheritFromParentParticlesV2 {
            public const nint m_flScale = 0x1E0; // CPerParticleFloatInput
            public const nint m_nFieldOutput = 0x358; // ParticleAttributeIndex_t
            public const nint m_nIncrement = 0x360; // CPerParticleFloatInput
            public const nint m_bSubSample = 0x4D8; // bool
            public const nint m_bRandomDistribution = 0x4D9; // bool
            public const nint m_bReverse = 0x4DA; // bool
            public const nint m_nMissingParentBehavior = 0x4DC; // MissingParentInheritBehavior_t
            public const nint m_flInterpolation = 0x4E0; // CPerParticleFloatInput
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_RandomSecondSequence {
            public const nint m_nSequenceMin = 0x1E8; // int32
            public const nint m_nSequenceMax = 0x1EC; // int32
        }
        // Parent: None
        // Field count: 4
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
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_SetFloatCollection {
            public const nint m_InputValue = 0x1E0; // CParticleCollectionFloatInput
            public const nint m_nOutputField = 0x358; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x35C; // ParticleSetMethod_t
            public const nint m_Lerp = 0x360; // CParticleCollectionFloatInput
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class PointDefinition_t {
            public const nint m_nControlPoint = 0x0; // int32
            public const nint m_bLocalCoords = 0x4; // bool
            public const nint m_vOffset = 0x8; // Vector
        }
        // Parent: None
        // Field count: 3
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertySuppressExpr
        public static class C_OP_Diffusion {
            public const nint m_flRadiusScale = 0x1E0; // float32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nVoxelGridResolution = 0x1E8; // int32
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_INIT_AgeNoise {
            public const nint m_bAbsVal = 0x1E8; // bool
            public const nint m_bAbsValInv = 0x1E9; // bool
            public const nint m_flOffset = 0x1EC; // float32
            public const nint m_flAgeMin = 0x1F0; // float32
            public const nint m_flAgeMax = 0x1F4; // float32
            public const nint m_flNoiseScale = 0x1F8; // float32
            public const nint m_flNoiseScaleLoc = 0x1FC; // float32
            public const nint m_vecOffsetLoc = 0x200; // Vector
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapVectorComponentToScalar {
            public const nint m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nComponent = 0x1E8; // int32
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class CGeneralRandomRotation {
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_flDegrees = 0x1EC; // float32
            public const nint m_flDegreesMin = 0x1F0; // float32
            public const nint m_flDegreesMax = 0x1F4; // float32
            public const nint m_flRotationRandExponent = 0x1F8; // float32
            public const nint m_bRandomlyFlipDirection = 0x1FC; // bool
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_DistanceBetweenVecs {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_vecPoint1 = 0x1E8; // CPerParticleVecInput
            public const nint m_vecPoint2 = 0x8C0; // CPerParticleVecInput
            public const nint m_flInputMin = 0xF98; // CPerParticleFloatInput
            public const nint m_flInputMax = 0x1110; // CPerParticleFloatInput
            public const nint m_flOutputMin = 0x1288; // CPerParticleFloatInput
            public const nint m_flOutputMax = 0x1400; // CPerParticleFloatInput
            public const nint m_nSetMethod = 0x1578; // ParticleSetMethod_t
            public const nint m_bDeltaTime = 0x157C; // bool
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_DampenToCP {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_flRange = 0x1E4; // float32
            public const nint m_flScale = 0x1E8; // float32
        }
        // Parent: None
        // Field count: 0
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapNamedModelBodyPartOnceTimed {
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_ScreenSpaceRotateTowardTarget {
            public const nint m_vecTargetPosition = 0x1E0; // CPerParticleVecInput
            public const nint m_flOutputRemap = 0x8B8; // CParticleRemapFloatInput
            public const nint m_nSetMethod = 0xA30; // ParticleSetMethod_t
            public const nint m_flScreenEdgeAlignmentDistance = 0xA38; // CPerParticleFloatInput
        }
        // Parent: None
        // Field count: 3
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        public static class C_OP_MovementMaintainOffset {
            public const nint m_vecOffset = 0x1E0; // Vector
            public const nint m_nCP = 0x1EC; // int32
            public const nint m_bRadiusScale = 0x1F0; // bool
        }
        // Parent: None
        // Field count: 11
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_CreateWithinCapsuleTransform {
            public const nint m_fRadiusMin = 0x1E8; // CPerParticleFloatInput
            public const nint m_fRadiusMax = 0x360; // CPerParticleFloatInput
            public const nint m_fHeight = 0x4D8; // CPerParticleFloatInput
            public const nint m_TransformInput = 0x650; // CParticleTransformInput
            public const nint m_fSpeedMin = 0x6B8; // CPerParticleFloatInput
            public const nint m_fSpeedMax = 0x830; // CPerParticleFloatInput
            public const nint m_fSpeedRandExp = 0x9A8; // float32
            public const nint m_LocalCoordinateSystemSpeedMin = 0x9B0; // CPerParticleVecInput
            public const nint m_LocalCoordinateSystemSpeedMax = 0x1088; // CPerParticleVecInput
            public const nint m_nFieldOutput = 0x1760; // ParticleAttributeIndex_t
            public const nint m_nFieldVelocity = 0x1764; // ParticleAttributeIndex_t
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetVec {
            public const nint m_InputValue = 0x1E0; // CPerParticleVecInput
            public const nint m_nOutputField = 0x8B8; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x8BC; // ParticleSetMethod_t
            public const nint m_Lerp = 0x8C0; // CPerParticleFloatInput
            public const nint m_bNormalizedOutput = 0xA38; // bool
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_CreateFromParentParticles {
            public const nint m_flVelocityScale = 0x1E8; // float32
            public const nint m_flIncrement = 0x1EC; // float32
            public const nint m_bRandomDistribution = 0x1F0; // bool
            public const nint m_nRandomSeed = 0x1F4; // int32
            public const nint m_bSubFrame = 0x1F8; // bool
            public const nint m_bSetRopeSegmentID = 0x1F9; // bool
        }
        // Parent: None
        // Field count: 4
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
        // MPropertyFriendlyName
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CheckParticleForWater {
            public const nint m_flRadius = 0x1E8; // CPerParticleFloatInput
            public const nint m_nFieldOutput = 0x360; // ParticleAttributeIndex_t
            public const nint m_flOutputRemap = 0x368; // CParticleRemapFloatInput
            public const nint m_nSetMethod = 0x4E0; // ParticleSetMethod_t
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MPropertyAttributeEditor
        // MGetKV3ClassDefaults
        public static class C_INIT_RandomNamedModelBodyPart {
        }
        // Parent: None
        // Field count: 30
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_RenderOmni2Light {
            public const nint m_nLightType = 0x230; // ParticleOmni2LightTypeChoiceList_t
            public const nint m_nMaxAllowed = 0x234; // uint16
            public const nint m_vColorBlend = 0x238; // CParticleCollectionVecInput
            public const nint m_nColorBlendType = 0x910; // ParticleColorBlendType_t
            public const nint m_strLightStyle = 0x918; // CUtlString
            public const nint m_flLightStyleTime = 0x920; // CPerParticleFloatInput
            public const nint m_nBrightnessUnit = 0xA98; // ParticleLightUnitChoiceList_t
            public const nint m_flBrightnessLumens = 0xAA0; // CPerParticleFloatInput
            public const nint m_flBrightnessCandelas = 0xC18; // CPerParticleFloatInput
            public const nint m_bCastShadows = 0xD90; // bool
            public const nint m_bDynamicBounce = 0xD91; // bool
            public const nint m_flBounceScale = 0xD98; // CParticleCollectionFloatInput
            public const nint m_bFog = 0xF10; // bool
            public const nint m_flFogScale = 0xF18; // CPerParticleFloatInput
            public const nint m_flLuminaireRadius = 0x1090; // CPerParticleFloatInput
            public const nint m_nOrientationType = 0x1208; // ParticleOmni2LighOrientationChoiceList_t
            public const nint m_vNormal = 0x1210; // CPerParticleVecInput
            public const nint m_vTarget = 0x18E8; // CPerParticleVecInput
            public const nint m_flFOVAngle = 0x1FC0; // CPerParticleFloatInput
            public const nint m_flBarnShape = 0x2138; // CPerParticleFloatInput
            public const nint m_flBarnNearSizeX = 0x22B0; // CPerParticleFloatInput
            public const nint m_flBarnNearSizeY = 0x2428; // CPerParticleFloatInput
            public const nint m_flBarnSoftX = 0x25A0; // CPerParticleFloatInput
            public const nint m_flBarnSoftY = 0x2718; // CPerParticleFloatInput
            public const nint m_flSkirt = 0x2890; // CPerParticleFloatInput
            public const nint m_flRange = 0x2A08; // CPerParticleFloatInput
            public const nint m_flInnerConeAngle = 0x2B80; // CPerParticleFloatInput
            public const nint m_flOuterConeAngle = 0x2CF8; // CPerParticleFloatInput
            public const nint m_hLightCookie = 0x2E70; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_bSphericalCookie = 0x2E78; // bool
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPAssignment_t {
            public const nint m_nCPNumber = 0x0; // int32
            public const nint m_Pos = 0x8; // CPerParticleVecInput
            public const nint m_nOrientationMode = 0x6E0; // ParticleOrientationSetMode_t
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
        public static class C_INIT_RemapParticleCountToNamedModelBodyPartScalar {
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_LagCompensation {
            public const nint m_nDesiredVelocityCP = 0x1E0; // int32
            public const nint m_nLatencyCP = 0x1E4; // int32
            public const nint m_nLatencyCPField = 0x1E8; // int32
            public const nint m_nDesiredVelocityCPField = 0x1EC; // int32
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MPropertySuppressExpr
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
        public static class C_OP_CollideWithSelf {
            public const nint m_flRadiusScale = 0x1E0; // CPerParticleFloatInput
            public const nint m_flMinimumSpeed = 0x358; // CPerParticleFloatInput
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
        public static class C_OP_Noise {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flOutputMin = 0x1E4; // float32
            public const nint m_flOutputMax = 0x1E8; // float32
            public const nint m_fl4NoiseScale = 0x1EC; // float32
            public const nint m_bAdditive = 0x1F0; // bool
            public const nint m_flNoiseAnimationTimeScale = 0x1F4; // float32
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_FadeAndKillForTracers {
            public const nint m_flStartFadeInTime = 0x1E0; // float32
            public const nint m_flEndFadeInTime = 0x1E4; // float32
            public const nint m_flStartFadeOutTime = 0x1E8; // float32
            public const nint m_flEndFadeOutTime = 0x1EC; // float32
            public const nint m_flStartAlpha = 0x1F0; // float32
            public const nint m_flEndAlpha = 0x1F4; // float32
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class CParticleMassCalculationParameters {
            public const nint m_nMassMode = 0x0; // ParticleMassMode_t
            public const nint m_flRadius = 0x8; // CPerParticleFloatInput
            public const nint m_flNominalRadius = 0x180; // CPerParticleFloatInput
            public const nint m_flScale = 0x2F8; // CPerParticleFloatInput
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_SequenceFromModel {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nFieldOutputAnim = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1EC; // float32
            public const nint m_flInputMax = 0x1F0; // float32
            public const nint m_flOutputMin = 0x1F4; // float32
            public const nint m_flOutputMax = 0x1F8; // float32
            public const nint m_nSetMethod = 0x1FC; // ParticleSetMethod_t
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        public static class C_OP_AlphaDecay {
            public const nint m_flMinAlpha = 0x1E0; // float32
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_InitVec {
            public const nint m_InputValue = 0x1E8; // CPerParticleVecInput
            public const nint m_nOutputField = 0x8C0; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x8C4; // ParticleSetMethod_t
            public const nint m_bNormalizedOutput = 0x8C8; // bool
            public const nint m_bWritePreviousPosition = 0x8C9; // bool
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
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        public static class C_INIT_SetHitboxToModel {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_nForceInModel = 0x1EC; // int32
            public const nint m_bEvenDistribution = 0x1F0; // bool
            public const nint m_nDesiredHitbox = 0x1F4; // int32
            public const nint m_vecHitBoxScale = 0x1F8; // CParticleCollectionVecInput
            public const nint m_vecDirectionBias = 0x8D0; // Vector
            public const nint m_bMaintainHitbox = 0x8DC; // bool
            public const nint m_bUseBones = 0x8DD; // bool
            public const nint m_HitboxSetName = 0x8DE; // char[128]
            public const nint m_flShellSize = 0x960; // CParticleCollectionFloatInput
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        public static class C_OP_MovementMoveAlongSkinnedCPSnapshot {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_nSnapshotControlPointNumber = 0x1E4; // int32
            public const nint m_bSetNormal = 0x1E8; // bool
            public const nint m_bSetRadius = 0x1E9; // bool
            public const nint m_flInterpolation = 0x1F0; // CPerParticleFloatInput
            public const nint m_flTValue = 0x368; // CPerParticleFloatInput
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_LerpScalar {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flOutput = 0x1E8; // CPerParticleFloatInput
            public const nint m_flStartTime = 0x360; // float32
            public const nint m_flEndTime = 0x364; // float32
        }
        // Parent: None
        // Field count: 13
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_InitialRepulsionVelocity {
            public const nint m_CollisionGroupName = 0x1E8; // char[128]
            public const nint m_nTraceSet = 0x268; // ParticleTraceSet_t
            public const nint m_vecOutputMin = 0x26C; // Vector
            public const nint m_vecOutputMax = 0x278; // Vector
            public const nint m_nControlPointNumber = 0x284; // int32
            public const nint m_bPerParticle = 0x288; // bool
            public const nint m_bTranslate = 0x289; // bool
            public const nint m_bProportional = 0x28A; // bool
            public const nint m_flTraceLength = 0x28C; // float32
            public const nint m_bPerParticleTR = 0x290; // bool
            public const nint m_bInherit = 0x291; // bool
            public const nint m_nChildCP = 0x294; // int32
            public const nint m_nChildGroupID = 0x298; // int32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_ClampScalar {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flOutputMin = 0x1E8; // CPerParticleFloatInput
            public const nint m_flOutputMax = 0x360; // CPerParticleFloatInput
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_SetControlPointToHMD {
            public const nint m_nCP1 = 0x1E8; // int32
            public const nint m_vecCP1Pos = 0x1EC; // Vector
            public const nint m_bOrientToHMD = 0x1F8; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DifferencePreviousParticle {
            public const nint m_nFieldInput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E8; // float32
            public const nint m_flInputMax = 0x1EC; // float32
            public const nint m_flOutputMin = 0x1F0; // float32
            public const nint m_flOutputMax = 0x1F4; // float32
            public const nint m_nSetMethod = 0x1F8; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x1FC; // bool
            public const nint m_bSetPreviousParticle = 0x1FD; // bool
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_PercentageBetweenTransforms {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E4; // float32
            public const nint m_flInputMax = 0x1E8; // float32
            public const nint m_flOutputMin = 0x1EC; // float32
            public const nint m_flOutputMax = 0x1F0; // float32
            public const nint m_TransformStart = 0x1F8; // CParticleTransformInput
            public const nint m_TransformEnd = 0x260; // CParticleTransformInput
            public const nint m_nSetMethod = 0x2C8; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x2CC; // bool
            public const nint m_bRadialCheck = 0x2CD; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_PlaneCull {
            public const nint m_nControlPoint = 0x1E8; // int32
            public const nint m_flDistance = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_bCullInside = 0x368; // bool
        }
        // Parent: None
        // Field count: 0
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
        // PARTICLE_MASSMODE_RADIUS_SQUARED
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_RemapNamedModelSequenceEndCap {
        }
        // Parent: None
        // Field count: 12
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_InitFromCPSnapshot {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_strSnapshotSubset = 0x1F0; // CUtlString
            public const nint m_nAttributeToRead = 0x1F8; // ParticleAttributeIndex_t
            public const nint m_nAttributeToWrite = 0x1FC; // ParticleAttributeIndex_t
            public const nint m_nLocalSpaceCP = 0x200; // int32
            public const nint m_bRandom = 0x204; // bool
            public const nint m_bReverse = 0x205; // bool
            public const nint m_nSnapShotIncrement = 0x208; // CParticleCollectionFloatInput
            public const nint m_bResetSnapshotIndexOnChanges = 0x380; // bool
            public const nint m_nManualSnapshotIndex = 0x388; // CPerParticleFloatInput
            public const nint m_nRandomSeed = 0x500; // int32
            public const nint m_bLocalSpaceAngles = 0x504; // bool
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // TEXTURE_REPETITION_PATH
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RenderCables {
            public const nint m_flRadiusScale = 0x230; // CParticleCollectionFloatInput
            public const nint m_flAlphaScale = 0x3A8; // CParticleCollectionFloatInput
            public const nint m_vecColorScale = 0x520; // CParticleCollectionVecInput
            public const nint m_nColorBlendType = 0xBF8; // ParticleColorBlendType_t
            public const nint m_hMaterial = 0xC00; // CStrongHandle<InfoForResourceTypeIMaterial2>
            public const nint m_nTextureRepetitionMode = 0xC08; // TextureRepetitionMode_t
            public const nint m_flTextureRepeatsPerSegment = 0xC10; // CParticleCollectionFloatInput
            public const nint m_flTextureRepeatsCircumference = 0xD88; // CParticleCollectionFloatInput
            public const nint m_flColorMapOffsetV = 0xF00; // CParticleCollectionFloatInput
            public const nint m_flColorMapOffsetU = 0x1078; // CParticleCollectionFloatInput
            public const nint m_flNormalMapOffsetV = 0x11F0; // CParticleCollectionFloatInput
            public const nint m_flNormalMapOffsetU = 0x1368; // CParticleCollectionFloatInput
            public const nint m_bDrawCableCaps = 0x14E0; // bool
            public const nint m_flCapRoundness = 0x14E4; // float32
            public const nint m_flCapOffsetAmount = 0x14E8; // float32
            public const nint m_flTessScale = 0x14EC; // float32
            public const nint m_nMinTesselation = 0x14F0; // int32
            public const nint m_nMaxTesselation = 0x14F4; // int32
            public const nint m_nRoundness = 0x14F8; // int32
            public const nint m_nForceRoundnessFixed = 0x14FC; // bool
            public const nint m_bOnlyRenderInEffectsBloomPass = 0x14FD; // bool
            public const nint m_LightingTransform = 0x1500; // CParticleTransformInput
            public const nint m_MaterialFloatVars = 0x1568; // CUtlLeanVector<FloatInputMaterialVariable_t>
            public const nint m_MaterialVecVars = 0x1588; // CUtlLeanVector<VecInputMaterialVariable_t>
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetChildControlPoints {
            public const nint m_nChildGroupID = 0x1E0; // int32
            public const nint m_nFirstControlPoint = 0x1E4; // int32
            public const nint m_nNumControlPoints = 0x1E8; // int32
            public const nint m_nFirstSourcePoint = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_bReverse = 0x368; // bool
            public const nint m_bSetOrientation = 0x369; // bool
            public const nint m_nOrientation = 0x36C; // ParticleOrientationType_t
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        public static class C_OP_ChladniWave {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E8; // CPerParticleFloatInput
            public const nint m_flInputMax = 0x360; // CPerParticleFloatInput
            public const nint m_flOutputMin = 0x4D8; // CPerParticleFloatInput
            public const nint m_flOutputMax = 0x650; // CPerParticleFloatInput
            public const nint m_vecWaveLength = 0x7C8; // CPerParticleVecInput
            public const nint m_vecHarmonics = 0xEA0; // CPerParticleVecInput
            public const nint m_nSetMethod = 0x1578; // ParticleSetMethod_t
            public const nint m_nLocalSpaceControlPoint = 0x157C; // int32
            public const nint m_b3D = 0x1580; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        public static class C_OP_RemapDirectionToCPToVector {
            public const nint m_nCP = 0x1E0; // int32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flScale = 0x1E8; // float32
            public const nint m_flOffsetRot = 0x1EC; // float32
            public const nint m_vecOffsetAxis = 0x1F0; // Vector
            public const nint m_bNormalize = 0x1FC; // bool
            public const nint m_nFieldStrength = 0x200; // ParticleAttributeIndex_t
        }
        // Parent: None
        // Field count: 9
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_OP_DriveCPFromGlobalSoundFloat {
            public const nint m_nOutputControlPoint = 0x1E8; // int32
            public const nint m_nOutputField = 0x1EC; // int32
            public const nint m_flInputMin = 0x1F0; // float32
            public const nint m_flInputMax = 0x1F4; // float32
            public const nint m_flOutputMin = 0x1F8; // float32
            public const nint m_flOutputMax = 0x1FC; // float32
            public const nint m_StackName = 0x200; // CUtlString
            public const nint m_OperatorName = 0x208; // CUtlString
            public const nint m_FieldName = 0x210; // CUtlString
        }
        // Parent: None
        // Field count: 4
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        public static class C_INIT_ScreenSpacePositionOfTarget {
            public const nint m_vecTargetPosition = 0x1E8; // CPerParticleVecInput
            public const nint m_bOututBehindness = 0x8C0; // bool
            public const nint m_nBehindFieldOutput = 0x8C4; // ParticleAttributeIndex_t
            public const nint m_flBehindOutputRemap = 0x8C8; // CParticleRemapFloatInput
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RtEnvCull {
            public const nint m_vecTestDir = 0x1E0; // Vector
            public const nint m_vecTestNormal = 0x1EC; // Vector
            public const nint m_bCullOnMiss = 0x1F8; // bool
            public const nint m_bStickInsteadOfCull = 0x1F9; // bool
            public const nint m_RtEnvName = 0x1FA; // char[128]
            public const nint m_nRTEnvCP = 0x27C; // int32
            public const nint m_nComponent = 0x280; // int32
        }
        // Parent: None
        // Field count: 14
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_PinParticleToCP {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_vecOffset = 0x1E8; // CParticleCollectionVecInput
            public const nint m_bOffsetLocal = 0x8C0; // bool
            public const nint m_nParticleSelection = 0x8C4; // ParticleSelection_t
            public const nint m_nParticleNumber = 0x8C8; // CParticleCollectionFloatInput
            public const nint m_nPinBreakType = 0xA40; // ParticlePinDistance_t
            public const nint m_flBreakDistance = 0xA48; // CParticleCollectionFloatInput
            public const nint m_flBreakSpeed = 0xBC0; // CParticleCollectionFloatInput
            public const nint m_flAge = 0xD38; // CParticleCollectionFloatInput
            public const nint m_nBreakControlPointNumber = 0xEB0; // int32
            public const nint m_nBreakControlPointNumber2 = 0xEB4; // int32
            public const nint m_flBreakValue = 0xEB8; // CParticleCollectionFloatInput
            public const nint m_flInterpolation = 0x1030; // CPerParticleFloatInput
            public const nint m_bRetainInitialVelocity = 0x11A8; // bool
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_RemapCPtoVector {
            public const nint m_nCPInput = 0x1E0; // int32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nLocalSpaceCP = 0x1E8; // int32
            public const nint m_vInputMin = 0x1EC; // Vector
            public const nint m_vInputMax = 0x1F8; // Vector
            public const nint m_vOutputMin = 0x204; // Vector
            public const nint m_vOutputMax = 0x210; // Vector
            public const nint m_flStartTime = 0x21C; // float32
            public const nint m_flEndTime = 0x220; // float32
            public const nint m_flInterpRate = 0x224; // float32
            public const nint m_nSetMethod = 0x228; // ParticleSetMethod_t
            public const nint m_bOffset = 0x22C; // bool
            public const nint m_bAccelerate = 0x22D; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DensityForce {
            public const nint m_flRadiusScale = 0x1F0; // float32
            public const nint m_flForceScale = 0x1F4; // float32
            public const nint m_flTargetDensity = 0x1F8; // float32
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
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CreateInEpitrochoid {
            public const nint m_nComponent1 = 0x1E8; // int32
            public const nint m_nComponent2 = 0x1EC; // int32
            public const nint m_TransformInput = 0x1F0; // CParticleTransformInput
            public const nint m_flParticleDensity = 0x258; // CPerParticleFloatInput
            public const nint m_flOffset = 0x3D0; // CPerParticleFloatInput
            public const nint m_flRadius1 = 0x548; // CPerParticleFloatInput
            public const nint m_flRadius2 = 0x6C0; // CPerParticleFloatInput
            public const nint m_bUseCount = 0x838; // bool
            public const nint m_bUseLocalCoords = 0x839; // bool
            public const nint m_bOffsetExistingPos = 0x83A; // bool
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_SetControlPointPositions {
            public const nint m_bUseWorldLocation = 0x1E8; // bool
            public const nint m_bOrient = 0x1E9; // bool
            public const nint m_bSetOnce = 0x1EA; // bool
            public const nint m_nCP1 = 0x1EC; // int32
            public const nint m_nCP2 = 0x1F0; // int32
            public const nint m_nCP3 = 0x1F4; // int32
            public const nint m_nCP4 = 0x1F8; // int32
            public const nint m_vecCP1Pos = 0x1FC; // Vector
            public const nint m_vecCP2Pos = 0x208; // Vector
            public const nint m_vecCP3Pos = 0x214; // Vector
            public const nint m_vecCP4Pos = 0x220; // Vector
            public const nint m_nHeadLocation = 0x22C; // int32
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class C_OP_SetFloatAttributeToVectorExpression {
            public const nint m_nExpression = 0x1E0; // VectorFloatExpressionType_t
            public const nint m_vInput1 = 0x1E8; // CPerParticleVecInput
            public const nint m_vInput2 = 0x8C0; // CPerParticleVecInput
            public const nint m_flOutputRemap = 0xF98; // CParticleRemapFloatInput
            public const nint m_nOutputField = 0x1110; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x1114; // ParticleSetMethod_t
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_MovementRotateParticleAroundAxis {
            public const nint m_vecRotAxis = 0x1E0; // CParticleCollectionVecInput
            public const nint m_flRotRate = 0x8B8; // CParticleCollectionFloatInput
            public const nint m_TransformInput = 0xA30; // CParticleTransformInput
            public const nint m_bLocalSpace = 0xA98; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_IntraParticleForce {
            public const nint m_flAttractionMinDistance = 0x1F0; // float32
            public const nint m_flAttractionMaxDistance = 0x1F4; // float32
            public const nint m_flAttractionMaxStrength = 0x1F8; // float32
            public const nint m_flRepulsionMinDistance = 0x1FC; // float32
            public const nint m_flRepulsionMaxDistance = 0x200; // float32
            public const nint m_flRepulsionMaxStrength = 0x204; // float32
            public const nint m_bUseAABB = 0x208; // bool
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
        public static class C_INIT_InitFloat {
            public const nint m_InputValue = 0x1E8; // CPerParticleFloatInput
            public const nint m_nOutputField = 0x360; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x364; // ParticleSetMethod_t
            public const nint m_InputStrength = 0x368; // CPerParticleFloatInput
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_InheritFromPeerSystem {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nFieldInput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_nIncrement = 0x1E8; // int32
            public const nint m_nGroupID = 0x1EC; // int32
        }
        // Parent: None
        // Field count: 3
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_PerParticleForce {
            public const nint m_flForceScale = 0x1F0; // CPerParticleFloatInput
            public const nint m_vForce = 0x368; // CPerParticleVecInput
            public const nint m_nCP = 0xA40; // int32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_MaxVelocity {
            public const nint m_flMaxVelocity = 0x1E0; // CPerParticleFloatInput
            public const nint m_flMinVelocity = 0x358; // CPerParticleFloatInput
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_VelocityFromNormal {
            public const nint m_fSpeedMin = 0x1E8; // float32
            public const nint m_fSpeedMax = 0x1EC; // float32
            public const nint m_bIgnoreDt = 0x1F0; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_INIT_PositionOffsetToCP {
            public const nint m_nControlPointNumberStart = 0x1E8; // int32
            public const nint m_nControlPointNumberEnd = 0x1EC; // int32
            public const nint m_bLocalCoords = 0x1F0; // bool
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
        // MPropertyAttributeRange
        public static class C_INIT_RemapInitialTransformDirectionToRotation {
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
            public const nint m_nFieldOutput = 0x250; // ParticleAttributeIndex_t
            public const nint m_flOffsetRot = 0x254; // float32
            public const nint m_nComponent = 0x258; // int32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        public static class C_OP_FadeAndKill {
            public const nint m_flStartFadeInTime = 0x1E0; // float32
            public const nint m_flEndFadeInTime = 0x1E4; // float32
            public const nint m_flStartFadeOutTime = 0x1E8; // float32
            public const nint m_flEndFadeOutTime = 0x1EC; // float32
            public const nint m_flStartAlpha = 0x1F0; // float32
            public const nint m_flEndAlpha = 0x1F4; // float32
            public const nint m_bForcePreserveParticleOrder = 0x1F8; // bool
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_ColorInterpolate {
            public const nint m_ColorFade = 0x1E0; // Color
            public const nint m_flFadeStartTime = 0x1F0; // float32
            public const nint m_flFadeEndTime = 0x1F4; // float32
            public const nint m_nFieldOutput = 0x1F8; // ParticleAttributeIndex_t
            public const nint m_bEaseInOut = 0x1FC; // bool
        }
        // Parent: None
        // Field count: 10
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_RampScalarSpline {
            public const nint m_RateMin = 0x1E0; // float32
            public const nint m_RateMax = 0x1E4; // float32
            public const nint m_flStartTime_min = 0x1E8; // float32
            public const nint m_flStartTime_max = 0x1EC; // float32
            public const nint m_flEndTime_min = 0x1F0; // float32
            public const nint m_flEndTime_max = 0x1F4; // float32
            public const nint m_flBias = 0x1F8; // float32
            public const nint m_nField = 0x220; // ParticleAttributeIndex_t
            public const nint m_bProportionalOp = 0x224; // bool
            public const nint m_bEaseOut = 0x225; // bool
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
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        public static class C_OP_RemapNamedModelSequenceOnceTimed {
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_MaintainSequentialPath {
            public const nint m_fMaxDistance = 0x1E0; // CParticleCollectionFloatInput
            public const nint m_flNumToAssign = 0x358; // CParticleCollectionFloatInput
            public const nint m_flCohesionStrength = 0x4D0; // CParticleCollectionFloatInput
            public const nint m_flTolerance = 0x648; // float32
            public const nint m_bLoop = 0x64C; // bool
            public const nint m_bUseParticleCount = 0x64D; // bool
            public const nint m_PathParams = 0x650; // CPathParameters
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_RemapNamedModelBodyPartEndCap {
        }
        // Parent: None
        // Field count: 3
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_StopAfterCPDuration {
            public const nint m_flDuration = 0x1E8; // CParticleCollectionFloatInput
            public const nint m_bDestroyImmediately = 0x360; // bool
            public const nint m_bPlayEndCap = 0x361; // bool
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
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class CGeneralSpin {
            public const nint m_nSpinRateDegrees = 0x1E0; // int32
            public const nint m_nSpinRateMinDegrees = 0x1E4; // int32
            public const nint m_fSpinRateStopTime = 0x1EC; // float32
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_INIT_RemapNamedModelElementToScalar {
            public const nint m_hModel = 0x1E8; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_names = 0x1F0; // CUtlVector<CUtlString>
            public const nint m_values = 0x208; // CUtlVector<float32>
            public const nint m_nFieldInput = 0x220; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x224; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x228; // ParticleSetMethod_t
            public const nint m_bModelFromRenderer = 0x22C; // bool
        }
        // Parent: None
        // Field count: 3
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_ClampVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_vecOutputMin = 0x1E8; // CPerParticleVecInput
            public const nint m_vecOutputMax = 0x8C0; // CPerParticleVecInput
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        public static class C_OP_RenderStatusEffectCitadel {
            public const nint m_pTextureColorWarp = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureNormal = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureMetalness = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureRoughness = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureSelfIllum = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_pTextureDetail = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_VelocityFromWind {
            public const nint m_vecSamplePosition = 0x1E8; // CPerParticleVecInput
            public const nint m_flScale = 0x8C0; // CPerParticleFloatInput
            public const nint m_bSampleFans = 0xA38; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_WindForce {
            public const nint m_vForce = 0x1F0; // Vector
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
        // MPropertyFriendlyName
        // MPropertySortPriority
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        public static class C_OP_RenderStandardLight {
            public const nint m_nLightType = 0x230; // ParticleLightTypeChoiceList_t
            public const nint m_nMaxAllowed = 0x234; // uint16
            public const nint m_vecColorScale = 0x238; // CParticleCollectionVecInput
            public const nint m_nColorBlendType = 0x910; // ParticleColorBlendType_t
            public const nint m_strLightStyle = 0x918; // CUtlString
            public const nint m_flLightStyleTime = 0x920; // CPerParticleFloatInput
            public const nint m_flIntensity = 0xA98; // CPerParticleFloatInput
            public const nint m_bCastShadows = 0xC10; // bool
            public const nint m_bDynamicBounce = 0xC11; // bool
            public const nint m_flBounceScale = 0xC18; // CParticleCollectionFloatInput
            public const nint m_flTheta = 0xD90; // CParticleCollectionFloatInput
            public const nint m_flPhi = 0xF08; // CParticleCollectionFloatInput
            public const nint m_flRadiusMultiplier = 0x1080; // CParticleCollectionFloatInput
            public const nint m_nAttenuationStyle = 0x11F8; // StandardLightingAttenuationStyle_t
            public const nint m_flFalloffLinearity = 0x1200; // CParticleCollectionFloatInput
            public const nint m_flFiftyPercentFalloff = 0x1378; // CParticleCollectionFloatInput
            public const nint m_flZeroPercentFalloff = 0x14F0; // CParticleCollectionFloatInput
            public const nint m_bRenderDiffuse = 0x1668; // bool
            public const nint m_bRenderSpecular = 0x1669; // bool
            public const nint m_lightCookie = 0x1670; // CUtlString
            public const nint m_nPriority = 0x1678; // int32
            public const nint m_nFogLightingMode = 0x167C; // ParticleLightFogLightingMode_t
            public const nint m_flFogContribution = 0x1680; // CParticleCollectionRendererFloatInput
            public const nint m_nCapsuleLightBehavior = 0x17F8; // ParticleLightBehaviorChoiceList_t
            public const nint m_flCapsuleLength = 0x17FC; // float32
            public const nint m_bReverseOrder = 0x1800; // bool
            public const nint m_bClosedLoop = 0x1801; // bool
            public const nint m_nPrevPntSource = 0x1804; // ParticleAttributeIndex_t
            public const nint m_flMaxLength = 0x1808; // float32
            public const nint m_flMinLength = 0x180C; // float32
            public const nint m_bIgnoreDT = 0x1810; // bool
            public const nint m_flConstrainRadiusToLengthRatio = 0x1814; // float32
            public const nint m_flLengthScale = 0x1818; // float32
            public const nint m_flLengthFadeInTime = 0x181C; // float32
        }
        // Parent: None
        // Field count: 15
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_DistanceToTransform {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E8; // CPerParticleFloatInput
            public const nint m_flInputMax = 0x360; // CPerParticleFloatInput
            public const nint m_flOutputMin = 0x4D8; // CPerParticleFloatInput
            public const nint m_flOutputMax = 0x650; // CPerParticleFloatInput
            public const nint m_TransformStart = 0x7C8; // CParticleTransformInput
            public const nint m_bLOS = 0x830; // bool
            public const nint m_CollisionGroupName = 0x831; // char[128]
            public const nint m_nTraceSet = 0x8B4; // ParticleTraceSet_t
            public const nint m_flMaxTraceLength = 0x8B8; // float32
            public const nint m_flLOSScale = 0x8BC; // float32
            public const nint m_nSetMethod = 0x8C0; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x8C4; // bool
            public const nint m_bAdditive = 0x8C5; // bool
            public const nint m_vecComponentScale = 0x8C8; // CPerParticleVecInput
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
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_RemapControlPointOrientationToRotation {
            public const nint m_nCP = 0x1E0; // int32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flOffsetRot = 0x1E8; // float32
            public const nint m_nComponent = 0x1EC; // int32
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_SetControlPointToCenter {
            public const nint m_nCP1 = 0x1E8; // int32
            public const nint m_vecCP1Pos = 0x1EC; // Vector
            public const nint m_bUseAvgParticlePos = 0x1F8; // bool
            public const nint m_nSetParent = 0x1FC; // ParticleParentSetMode_t
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
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
        // MPropertySuppressExpr
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
        public static class C_OP_RemapAverageScalarValuetoCP {
            public const nint m_nExpression = 0x1E8; // SetStatisticExpressionType_t
            public const nint m_flDecimalPlaces = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_nOutControlPointNumber = 0x368; // int32
            public const nint m_nOutVectorField = 0x36C; // int32
            public const nint m_nField = 0x370; // ParticleAttributeIndex_t
            public const nint m_flOutputRemap = 0x378; // CParticleRemapFloatInput
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RemapDotProductToScalar {
            public const nint m_nInputCP1 = 0x1E0; // int32
            public const nint m_nInputCP2 = 0x1E4; // int32
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1EC; // float32
            public const nint m_flInputMax = 0x1F0; // float32
            public const nint m_flOutputMin = 0x1F4; // float32
            public const nint m_flOutputMax = 0x1F8; // float32
            public const nint m_bUseParticleVelocity = 0x1FC; // bool
            public const nint m_nSetMethod = 0x200; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x204; // bool
            public const nint m_bUseParticleNormal = 0x205; // bool
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_CurlNoiseForce {
            public const nint m_nNoiseType = 0x1F0; // ParticleDirectionNoiseType_t
            public const nint m_vecNoiseFreq = 0x1F8; // CPerParticleVecInput
            public const nint m_vecNoiseScale = 0x8D0; // CPerParticleVecInput
            public const nint m_vecOffset = 0xFA8; // CPerParticleVecInput
            public const nint m_vecOffsetRate = 0x1680; // CPerParticleVecInput
            public const nint m_flWorleySeed = 0x1D58; // CPerParticleFloatInput
            public const nint m_flWorleyJitter = 0x1ED0; // CPerParticleFloatInput
            public const nint m_nCP = 0x1F0; // int32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        public static class C_INIT_Orient2DRelToCP {
            public const nint m_nCP = 0x1E8; // int32
            public const nint m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_flRotOffset = 0x1F0; // float32
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
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
        public static class C_OP_FadeIn {
            public const nint m_flFadeInTimeMin = 0x1E0; // float32
            public const nint m_flFadeInTimeMax = 0x1E4; // float32
            public const nint m_flFadeInTimeExp = 0x1E8; // float32
            public const nint m_bProportional = 0x1EC; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_RemapVectorToRotations {
            public const nint m_vecInput = 0x1E0; // CPerParticleVecInput
            public const nint m_vecRotation = 0x8B8; // CPerParticleVecInput
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
        // MPropertySuppressExpr
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
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
        public static class C_INIT_GlobalScale {
            public const nint m_flScale = 0x1E8; // float32
            public const nint m_nScaleControlPointNumber = 0x1EC; // int32
            public const nint m_nControlPointNumber = 0x1F0; // int32
            public const nint m_bScaleRadius = 0x1F4; // bool
            public const nint m_bScalePosition = 0x1F5; // bool
            public const nint m_bScaleVelocity = 0x1F6; // bool
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        public static class C_INIT_RadiusFromCPObject {
            public const nint m_nControlPoint = 0x1E8; // int32
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        public static class C_INIT_InitialVelocityFromHitbox {
            public const nint m_flVelocityMin = 0x1E8; // float32
            public const nint m_flVelocityMax = 0x1EC; // float32
            public const nint m_nControlPointNumber = 0x1F0; // int32
            public const nint m_HitboxSetName = 0x1F4; // char[128]
            public const nint m_bUseBones = 0x274; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_OP_LerpVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_vecOutput = 0x1E4; // Vector
            public const nint m_flStartTime = 0x1F0; // float32
            public const nint m_flEndTime = 0x1F4; // float32
            public const nint m_nSetMethod = 0x1F8; // ParticleSetMethod_t
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
        // MGetKV3ClassDefaults
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
        public static class C_OP_SetControlPointFieldToWater {
            public const nint m_nSourceCP = 0x1E8; // int32
            public const nint m_nDestCP = 0x1EC; // int32
            public const nint m_nCPField = 0x1F0; // int32
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
        public static class TextureGroup_t {
            public const nint m_bEnabled = 0x0; // bool
            public const nint m_bReplaceTextureWithGradient = 0x1; // bool
            public const nint m_hTexture = 0x8; // CStrongHandle<InfoForResourceTypeCTextureBase>
            public const nint m_Gradient = 0x10; // CColorGradient
            public const nint m_nTextureType = 0x28; // SpriteCardTextureType_t
            public const nint m_nTextureChannels = 0x2C; // SpriteCardTextureChannel_t
            public const nint m_nTextureBlendMode = 0x30; // ParticleTextureLayerBlendType_t
            public const nint m_flTextureBlend = 0x38; // CParticleCollectionRendererFloatInput
            public const nint m_TextureControls = 0x1B0; // TextureControls_t
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_TimeVaryingForce {
            public const nint m_flStartLerpTime = 0x1F0; // float32
            public const nint m_StartingForce = 0x1F4; // Vector
            public const nint m_flEndLerpTime = 0x200; // float32
            public const nint m_EndingForce = 0x204; // Vector
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
        public static class C_OP_SetCPOrientationToGroundNormal {
            public const nint m_flInterpRate = 0x1E0; // float32
            public const nint m_flMaxTraceLength = 0x1E4; // float32
            public const nint m_flTolerance = 0x1E8; // float32
            public const nint m_flTraceOffset = 0x1EC; // float32
            public const nint m_CollisionGroupName = 0x1F0; // char[128]
            public const nint m_nTraceSet = 0x270; // ParticleTraceSet_t
            public const nint m_nInputCP = 0x274; // int32
            public const nint m_nOutputCP = 0x278; // int32
            public const nint m_bIncludeWater = 0x288; // bool
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CreateWithinSphereTransform {
            public const nint m_fRadiusMin = 0x1E8; // CPerParticleFloatInput
            public const nint m_fRadiusMax = 0x360; // CPerParticleFloatInput
            public const nint m_vecDistanceBias = 0x4D8; // CPerParticleVecInput
            public const nint m_vecDistanceBiasAbs = 0xBB0; // Vector
            public const nint m_TransformInput = 0xBC0; // CParticleTransformInput
            public const nint m_fSpeedMin = 0xC28; // CPerParticleFloatInput
            public const nint m_fSpeedMax = 0xDA0; // CPerParticleFloatInput
            public const nint m_fSpeedRandExp = 0xF18; // float32
            public const nint m_bLocalCoords = 0xF1C; // bool
            public const nint m_LocalCoordinateSystemSpeedMin = 0xF20; // CPerParticleVecInput
            public const nint m_LocalCoordinateSystemSpeedMax = 0x15F8; // CPerParticleVecInput
            public const nint m_nFieldOutput = 0x1CD0; // ParticleAttributeIndex_t
            public const nint m_nFieldVelocity = 0x1CD4; // ParticleAttributeIndex_t
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
        // MParticleMinVersion
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class C_OP_RadiusDecay {
            public const nint m_flMinRadius = 0x1E0; // float32
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_RemapNamedModelBodyPartToScalar {
        }
        // Parent: None
        // Field count: 12
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_RemapScalarToVector {
            public const nint m_nFieldInput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1F0; // float32
            public const nint m_flInputMax = 0x1F4; // float32
            public const nint m_vecOutputMin = 0x1F8; // Vector
            public const nint m_vecOutputMax = 0x204; // Vector
            public const nint m_flStartTime = 0x210; // float32
            public const nint m_flEndTime = 0x214; // float32
            public const nint m_nSetMethod = 0x218; // ParticleSetMethod_t
            public const nint m_nControlPointNumber = 0x21C; // int32
            public const nint m_bLocalCoords = 0x220; // bool
            public const nint m_flRemapBias = 0x224; // float32
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_InitialSequenceFromModel {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_nFieldOutputAnim = 0x1F0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1F4; // float32
            public const nint m_flInputMax = 0x1F8; // float32
            public const nint m_flOutputMin = 0x1FC; // float32
            public const nint m_flOutputMax = 0x200; // float32
            public const nint m_nSetMethod = 0x204; // ParticleSetMethod_t
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
        public static class C_OP_SelectivelyEnableChildren {
            public const nint m_nChildGroupID = 0x1E8; // CParticleCollectionFloatInput
            public const nint m_nFirstChild = 0x360; // CParticleCollectionFloatInput
            public const nint m_nNumChildrenToEnable = 0x4D8; // CParticleCollectionFloatInput
            public const nint m_bPlayEndcapOnStop = 0x650; // bool
            public const nint m_bDestroyImmediately = 0x651; // bool
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
        // MPropertyFriendlyName
        public static class ModelReference_t {
            public const nint m_model = 0x0; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_flRelativeProbabilityOfSpawn = 0x8; // float32
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        public static class C_INIT_CreateFromCPs {
            public const nint m_nIncrement = 0x1E8; // int32
            public const nint m_nMinCP = 0x1EC; // int32
            public const nint m_nMaxCP = 0x1F0; // int32
            public const nint m_nDynamicCPCount = 0x1F8; // CParticleCollectionFloatInput
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MPropertyAttributeEditor
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_CreateSpiralSphere {
            public const nint m_TransformInput = 0x1E8; // CParticleTransformInput
            public const nint m_flDensity = 0x250; // CPerParticleFloatInput
            public const nint m_flInitialRadius = 0x3C8; // CPerParticleFloatInput
            public const nint m_flInitialSpeedMin = 0x540; // CPerParticleFloatInput
            public const nint m_flInitialSpeedMax = 0x6B8; // CPerParticleFloatInput
            public const nint m_bUseParticleCount = 0x830; // bool
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_CPVelocityForce {
            public const nint m_nControlPointNumber = 0x1F0; // int32
            public const nint m_flScale = 0x1F8; // CPerParticleFloatInput
        }
        // Parent: None
        // Field count: 1
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
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_ScaleVelocity {
            public const nint m_vecScale = 0x1E8; // CPerParticleVecInput
        }
        // Parent: None
        // Field count: 9
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_MoveToHitbox {
            public const nint m_modelInput = 0x1E0; // CParticleModelInput
            public const nint m_transformInput = 0x240; // CParticleTransformInput
            public const nint m_flLifeTimeLerpStart = 0x2AC; // float32
            public const nint m_flLifeTimeLerpEnd = 0x2B0; // float32
            public const nint m_flPrevPosScale = 0x2B4; // float32
            public const nint m_HitboxSetName = 0x2B8; // char[128]
            public const nint m_bUseBones = 0x338; // bool
            public const nint m_nLerpType = 0x33C; // HitboxLerpType_t
            public const nint m_flInterpolation = 0x340; // CPerParticleFloatInput
        }
        // Parent: None
        // Field count: 3
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_PinRopeSegmentParticleToParent {
            public const nint m_nParticleSelection = 0x1E0; // ParticleSelection_t
            public const nint m_nParticleNumber = 0x1E8; // CParticleCollectionFloatInput
            public const nint m_flInterpolation = 0x360; // CPerParticleFloatInput
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_PointList {
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_pointList = 0x1F0; // CUtlVector<PointDefinition_t>
            public const nint m_bPlaceAlongPath = 0x208; // bool
            public const nint m_bClosedLoop = 0x209; // bool
            public const nint m_nNumPointsAlongPath = 0x20C; // int32
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_LerpToOtherAttribute {
            public const nint m_flInterpolation = 0x1E0; // CPerParticleFloatInput
            public const nint m_nFieldInputFrom = 0x358; // ParticleAttributeIndex_t
            public const nint m_nFieldInput = 0x35C; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x360; // ParticleAttributeIndex_t
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
        // MPropertyFriendlyName
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_INIT_RandomColor {
            public const nint m_ColorMin = 0x204; // Color
            public const nint m_ColorMax = 0x208; // Color
            public const nint m_TintMin = 0x20C; // Color
            public const nint m_TintMax = 0x210; // Color
            public const nint m_flTintPerc = 0x214; // float32
            public const nint m_flUpdateThreshold = 0x218; // float32
            public const nint m_nTintCP = 0x21C; // int32
            public const nint m_nFieldOutput = 0x220; // ParticleAttributeIndex_t
            public const nint m_nTintBlendMode = 0x224; // ParticleColorBlendMode_t
            public const nint m_flLightAmplification = 0x228; // float32
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_InheritFromParentParticles {
            public const nint m_flScale = 0x1E8; // float32
            public const nint m_nFieldOutput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_nIncrement = 0x1F0; // int32
            public const nint m_bRandomDistribution = 0x1F4; // bool
            public const nint m_nRandomSeed = 0x1F8; // int32
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_RampScalarLinearSimple {
            public const nint m_Rate = 0x1E0; // float32
            public const nint m_flStartTime = 0x1E4; // float32
            public const nint m_flEndTime = 0x1E8; // float32
            public const nint m_nField = 0x210; // ParticleAttributeIndex_t
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
        // MPropertyAttributeChoiceName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        public static class C_INIT_ChaoticAttractor {
            public const nint m_flAParm = 0x1E8; // float32
            public const nint m_flBParm = 0x1EC; // float32
            public const nint m_flCParm = 0x1F0; // float32
            public const nint m_flDParm = 0x1F4; // float32
            public const nint m_flScale = 0x1F8; // float32
            public const nint m_flSpeedMin = 0x1FC; // float32
            public const nint m_flSpeedMax = 0x200; // float32
            public const nint m_nBaseCP = 0x204; // int32
            public const nint m_bUniformSpeed = 0x208; // bool
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        public static class C_OP_MovementRigidAttachToCP {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_nScaleControlPoint = 0x1E4; // int32
            public const nint m_nScaleCPField = 0x1E8; // int32
            public const nint m_nFieldInput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1F0; // ParticleAttributeIndex_t
            public const nint m_bOffsetLocal = 0x1F4; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        public static class C_INIT_SkyVisCull {
            public const nint m_vecTestPosition = 0x1E8; // CPerParticleVecInput
            public const nint m_vecTestDir = 0x8C0; // CParticleCollectionVecInput
            public const nint m_nTraceSet = 0xF98; // ParticleTraceSet_t
            public const nint m_bCullOnSky = 0xF9C; // bool
        }
        // Parent: None
        // Field count: 15
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_DistanceToCPInit {
            public const nint m_nFieldOutput = 0x1E8; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1F0; // CPerParticleFloatInput
            public const nint m_flInputMax = 0x368; // CPerParticleFloatInput
            public const nint m_flOutputMin = 0x4E0; // CPerParticleFloatInput
            public const nint m_flOutputMax = 0x658; // CPerParticleFloatInput
            public const nint m_nStartCP = 0x7D0; // int32
            public const nint m_bLOS = 0x7D4; // bool
            public const nint m_CollisionGroupName = 0x7D5; // char[128]
            public const nint m_nTraceSet = 0x858; // ParticleTraceSet_t
            public const nint m_flMaxTraceLength = 0x860; // CPerParticleFloatInput
            public const nint m_flLOSScale = 0x9D8; // float32
            public const nint m_nSetMethod = 0x9DC; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x9E0; // bool
            public const nint m_vecDistanceScale = 0x9E4; // Vector
            public const nint m_flRemapBias = 0x9F0; // float32
        }
        // Parent: None
        // Field count: 0
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        public static class C_OP_EndCapDecay {
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
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_ForceBasedOnDistanceToPlane {
            public const nint m_flMinDist = 0x1F0; // float32
            public const nint m_vecForceAtMinDist = 0x1F4; // Vector
            public const nint m_flMaxDist = 0x200; // float32
            public const nint m_vecForceAtMaxDist = 0x204; // Vector
            public const nint m_vecPlaneNormal = 0x210; // Vector
            public const nint m_nControlPointNumber = 0x21C; // int32
            public const nint m_flExponent = 0x220; // float32
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
        // MPropertyFriendlyName
        public static class C_OP_RemapDensityToVector {
            public const nint m_flRadiusScale = 0x1E0; // float32
            public const nint m_nFieldOutput = 0x1E4; // ParticleAttributeIndex_t
            public const nint m_flDensityMin = 0x1E8; // float32
            public const nint m_flDensityMax = 0x1EC; // float32
            public const nint m_vecOutputMin = 0x1F0; // Vector
            public const nint m_vecOutputMax = 0x1FC; // Vector
            public const nint m_bUseParentDensity = 0x208; // bool
            public const nint m_nVoxelGridResolution = 0x20C; // int32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // PARTICLE_WORLDSPACE_CENTER
        // PARTICLE_EYES
        public static class ParticleControlPointConfiguration_t {
            public const nint m_name = 0x0; // CUtlString
            public const nint m_drivers = 0x8; // CUtlVector<ParticleControlPointDriver_t>
            public const nint m_previewState = 0x20; // ParticlePreviewState_t
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        public static class C_INIT_SetRigidAttachment {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_nFieldInput = 0x1EC; // ParticleAttributeIndex_t
            public const nint m_nFieldOutput = 0x1F0; // ParticleAttributeIndex_t
            public const nint m_bLocalSpace = 0x1F4; // bool
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
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        public static class MaterialVariable_t {
            public const nint m_strVariable = 0x0; // CUtlString
            public const nint m_nVariableField = 0x8; // ParticleAttributeIndex_t
            public const nint m_flScale = 0xC; // float32
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
        // MPropertySuppressExpr
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
        public static class C_OP_RemapSpeed {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E4; // float32
            public const nint m_flInputMax = 0x1E8; // float32
            public const nint m_flOutputMin = 0x1EC; // float32
            public const nint m_flOutputMax = 0x1F0; // float32
            public const nint m_nSetMethod = 0x1F4; // ParticleSetMethod_t
            public const nint m_bIgnoreDelta = 0x1F8; // bool
        }
        // Parent: None
        // Field count: 58
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RenderModels {
            public const nint m_bOnlyRenderInEffectsBloomPass = 0x230; // bool
            public const nint m_bOnlyRenderInEffectsWaterPass = 0x231; // bool
            public const nint m_bUseMixedResolutionRendering = 0x232; // bool
            public const nint m_bOnlyRenderInEffecsGameOverlay = 0x233; // bool
            public const nint m_ModelList = 0x238; // CUtlVector<ModelReference_t>
            public const nint m_nBodyGroupField = 0x250; // ParticleAttributeIndex_t
            public const nint m_nSubModelField = 0x254; // ParticleAttributeIndex_t
            public const nint m_bIgnoreNormal = 0x258; // bool
            public const nint m_bOrientZ = 0x259; // bool
            public const nint m_bCenterOffset = 0x25A; // bool
            public const nint m_vecLocalOffset = 0x260; // CPerParticleVecInput
            public const nint m_vecLocalRotation = 0x938; // CPerParticleVecInput
            public const nint m_bIgnoreRadius = 0x1010; // bool
            public const nint m_nModelScaleCP = 0x1014; // int32
            public const nint m_vecComponentScale = 0x1018; // CPerParticleVecInput
            public const nint m_bLocalScale = 0x16F0; // bool
            public const nint m_nSizeCullBloat = 0x16F4; // int32
            public const nint m_bAnimated = 0x16F8; // bool
            public const nint m_flAnimationRate = 0x1700; // CPerParticleFloatInput
            public const nint m_bScaleAnimationRate = 0x1878; // bool
            public const nint m_bForceLoopingAnimation = 0x1879; // bool
            public const nint m_bResetAnimOnStop = 0x187A; // bool
            public const nint m_bManualAnimFrame = 0x187B; // bool
            public const nint m_nAnimationScaleField = 0x187C; // ParticleAttributeIndex_t
            public const nint m_nAnimationField = 0x1880; // ParticleAttributeIndex_t
            public const nint m_nManualFrameField = 0x1884; // ParticleAttributeIndex_t
            public const nint m_ActivityName = 0x1888; // char[256]
            public const nint m_SequenceName = 0x1988; // char[256]
            public const nint m_bEnableClothSimulation = 0x1A88; // bool
            public const nint m_bDisableClothGroundCollision = 0x1A89; // bool
            public const nint m_ClothEffectName = 0x1A8A; // char[64]
            public const nint m_hOverrideMaterial = 0x1AD0; // CStrongHandle<InfoForResourceTypeIMaterial2>
            public const nint m_bOverrideTranslucentMaterials = 0x1AD8; // bool
            public const nint m_nSkin = 0x1AE0; // CPerParticleFloatInput
            public const nint m_MaterialVars = 0x1C58; // CUtlVector<MaterialVariable_t>
            public const nint m_flRenderFilter = 0x1C70; // CPerParticleFloatInput
            public const nint m_flManualModelSelection = 0x1DE8; // CPerParticleFloatInput
            public const nint m_modelInput = 0x1F60; // CParticleModelInput
            public const nint m_nLOD = 0x1FC0; // int32
            public const nint m_EconSlotName = 0x1FC4; // char[256]
            public const nint m_bOriginalModel = 0x20C4; // bool
            public const nint m_bSuppressTint = 0x20C5; // bool
            public const nint m_nSubModelFieldType = 0x20C8; // RenderModelSubModelFieldType_t
            public const nint m_bDisableShadows = 0x20CC; // bool
            public const nint m_bDisableDepthPrepass = 0x20CD; // bool
            public const nint m_bAcceptsDecals = 0x20CE; // bool
            public const nint m_bForceDrawInterlevedWithSiblings = 0x20CF; // bool
            public const nint m_bDoNotDrawInParticlePass = 0x20D0; // bool
            public const nint m_bAllowApproximateTransforms = 0x20D1; // bool
            public const nint m_szRenderAttribute = 0x20D2; // char[260]
            public const nint m_flRadiusScale = 0x21D8; // CParticleCollectionFloatInput
            public const nint m_flAlphaScale = 0x2350; // CParticleCollectionFloatInput
            public const nint m_flRollScale = 0x24C8; // CParticleCollectionFloatInput
            public const nint m_nAlpha2Field = 0x2640; // ParticleAttributeIndex_t
            public const nint m_vecColorScale = 0x2648; // CParticleCollectionVecInput
            public const nint m_nColorBlendType = 0x2D20; // ParticleColorBlendType_t
            public const nint m_strLightStyle = 0x2D28; // CUtlString
            public const nint m_flLightStyleTime = 0x2D30; // CPerParticleFloatInput
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_RemapNamedModelMeshGroupToScalar {
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MPropertySuppressExpr
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
        public static class C_OP_RopeSpringConstraint {
            public const nint m_flRestLength = 0x1E0; // CParticleCollectionFloatInput
            public const nint m_flMinDistance = 0x358; // CParticleCollectionFloatInput
            public const nint m_flMaxDistance = 0x4D0; // CParticleCollectionFloatInput
            public const nint m_flAdjustmentScale = 0x648; // float32
            public const nint m_flInitialRestingLength = 0x650; // CParticleCollectionFloatInput
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
        // MParticleMinVersion
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        public static class C_INIT_PositionWarpScalar {
            public const nint m_vecWarpMin = 0x1E8; // Vector
            public const nint m_vecWarpMax = 0x1F4; // Vector
            public const nint m_InputValue = 0x200; // CPerParticleFloatInput
            public const nint m_flPrevPosScale = 0x378; // float32
            public const nint m_nScaleControlPointNumber = 0x37C; // int32
            public const nint m_nControlPointNumber = 0x380; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        public static class C_OP_ForceControlPointStub {
            public const nint m_ControlPoint = 0x1E8; // int32
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MPropertyFriendlyName
        public static class C_OP_VectorNoise {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_vecOutputMin = 0x1E4; // Vector
            public const nint m_vecOutputMax = 0x1F0; // Vector
            public const nint m_fl4NoiseScale = 0x1FC; // float32
            public const nint m_bAdditive = 0x200; // bool
            public const nint m_bOffset = 0x201; // bool
            public const nint m_flNoiseAnimationTimeScale = 0x204; // float32
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_RemapParticleCountToScalar {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_nInputMin = 0x1E8; // CParticleCollectionFloatInput
            public const nint m_nInputMax = 0x360; // CParticleCollectionFloatInput
            public const nint m_flOutputMin = 0x4D8; // CParticleCollectionFloatInput
            public const nint m_flOutputMax = 0x650; // CParticleCollectionFloatInput
            public const nint m_bActiveRange = 0x7C8; // bool
            public const nint m_nSetMethod = 0x7CC; // ParticleSetMethod_t
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyFriendlyName
        public static class C_INIT_QuantizeFloat {
            public const nint m_InputValue = 0x1E8; // CPerParticleFloatInput
            public const nint m_nOutputField = 0x360; // ParticleAttributeIndex_t
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetToCP {
            public const nint m_nControlPointNumber = 0x1E0; // int32
            public const nint m_vecOffset = 0x1E4; // Vector
            public const nint m_bOffsetLocal = 0x1F0; // bool
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // PARTICLE_FAN_TYPE_ROTOR_WASH
        // PARTICLE_FAN_TYPE_RADIAL
        public static class ParticleControlPointDriver_t {
            public const nint m_iControlPoint = 0x0; // ParticleParamID_t
            public const nint m_iAttachType = 0x10; // ParticleAttachment_t
            public const nint m_attachmentName = 0x18; // CUtlString
            public const nint m_vecOffset = 0x20; // Vector
            public const nint m_angOffset = 0x2C; // QAngle
            public const nint m_entityName = 0x38; // CUtlString
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
        public static class C_OP_ParentVortices {
            public const nint m_flForceScale = 0x1F0; // float32
            public const nint m_vecTwistAxis = 0x1F4; // Vector
            public const nint m_bFlipBasedOnYaw = 0x200; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_SetControlPointToCPVelocity {
            public const nint m_nCPInput = 0x1E8; // int32
            public const nint m_nCPOutputVel = 0x1EC; // int32
            public const nint m_bNormalize = 0x1F0; // bool
            public const nint m_nCPOutputMag = 0x1F4; // int32
            public const nint m_nCPField = 0x1F8; // int32
            public const nint m_vecComparisonVelocity = 0x200; // CParticleCollectionVecInput
        }
        // Parent: None
        // Field count: 0
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
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
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
        public static class C_OP_SpinYaw {
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
        // MPropertySuppressExpr
        public static class RenderProjectedMaterial_t {
            public const nint m_hMaterial = 0x0; // CStrongHandle<InfoForResourceTypeIMaterial2>
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_SetFloatAttributeToVectorExpression {
            public const nint m_nExpression = 0x1E8; // VectorFloatExpressionType_t
            public const nint m_vInput1 = 0x1F0; // CPerParticleVecInput
            public const nint m_vInput2 = 0x8C8; // CPerParticleVecInput
            public const nint m_flOutputRemap = 0xFA0; // CParticleRemapFloatInput
            public const nint m_nOutputField = 0x1118; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x111C; // ParticleSetMethod_t
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_ModelCull {
            public const nint m_nControlPointNumber = 0x1E8; // int32
            public const nint m_bBoundBox = 0x1EC; // bool
            public const nint m_bCullOutside = 0x1ED; // bool
            public const nint m_bUseBones = 0x1EE; // bool
            public const nint m_HitboxSetName = 0x1EF; // char[128]
        }
        // Parent: None
        // Field count: 12
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_PercentageBetweenTransformLerpCPs {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_flInputMin = 0x1E4; // float32
            public const nint m_flInputMax = 0x1E8; // float32
            public const nint m_TransformStart = 0x1F0; // CParticleTransformInput
            public const nint m_TransformEnd = 0x258; // CParticleTransformInput
            public const nint m_nOutputStartCP = 0x2C0; // int32
            public const nint m_nOutputStartField = 0x2C4; // int32
            public const nint m_nOutputEndCP = 0x2C8; // int32
            public const nint m_nOutputEndField = 0x2CC; // int32
            public const nint m_nSetMethod = 0x2D0; // ParticleSetMethod_t
            public const nint m_bActiveRange = 0x2D4; // bool
            public const nint m_bRadialCheck = 0x2D5; // bool
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_SetPerChildControlPoint {
            public const nint m_nChildGroupID = 0x1E0; // int32
            public const nint m_nFirstControlPoint = 0x1E4; // int32
            public const nint m_nNumControlPoints = 0x1E8; // int32
            public const nint m_nParticleIncrement = 0x1F0; // CParticleCollectionFloatInput
            public const nint m_nFirstSourcePoint = 0x368; // CParticleCollectionFloatInput
            public const nint m_bSetOrientation = 0x4E0; // bool
            public const nint m_nOrientationField = 0x4E4; // ParticleAttributeIndex_t
            public const nint m_bNumBasedOnParticleCount = 0x4E8; // bool
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        public static class C_OP_WorldCollideConstraint {
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
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
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        public static class C_OP_SetAttributeToScalarExpression {
            public const nint m_nExpression = 0x1E0; // ScalarExpressionType_t
            public const nint m_flInput1 = 0x1E8; // CPerParticleFloatInput
            public const nint m_flInput2 = 0x360; // CPerParticleFloatInput
            public const nint m_flOutputRemap = 0x4D8; // CParticleRemapFloatInput
            public const nint m_nOutputField = 0x650; // ParticleAttributeIndex_t
            public const nint m_nSetMethod = 0x654; // ParticleSetMethod_t
        }
        // Parent: None
        // Field count: 8
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
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
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
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertySortPriority
        // MPropertyStartGroup
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertyAttributeEditor
        // MPropertyFriendlyName
        // MPropertySortPriority
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RenderMaterialProxy {
            public const nint m_nMaterialControlPoint = 0x230; // int32
            public const nint m_nProxyType = 0x234; // MaterialProxyType_t
            public const nint m_MaterialVars = 0x238; // CUtlVector<MaterialVariable_t>
            public const nint m_hOverrideMaterial = 0x250; // CStrongHandle<InfoForResourceTypeIMaterial2>
            public const nint m_flMaterialOverrideEnabled = 0x258; // CParticleCollectionFloatInput
            public const nint m_vecColorScale = 0x3D0; // CParticleCollectionVecInput
            public const nint m_flAlpha = 0xAA8; // CPerParticleFloatInput
            public const nint m_nColorBlendType = 0xC20; // ParticleColorBlendType_t
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        public static class FloatInputMaterialVariable_t {
            public const nint m_strVariable = 0x0; // CUtlString
            public const nint m_flInput = 0x8; // CParticleCollectionFloatInput
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
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        public static class C_OP_RampScalarLinear {
            public const nint m_RateMin = 0x1E0; // float32
            public const nint m_RateMax = 0x1E4; // float32
            public const nint m_flStartTime_min = 0x1E8; // float32
            public const nint m_flStartTime_max = 0x1EC; // float32
            public const nint m_flEndTime_min = 0x1F0; // float32
            public const nint m_flEndTime_max = 0x1F4; // float32
            public const nint m_nField = 0x220; // ParticleAttributeIndex_t
            public const nint m_bProportionalOp = 0x224; // bool
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
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_OP_RotateVector {
            public const nint m_nFieldOutput = 0x1E0; // ParticleAttributeIndex_t
            public const nint m_vecRotAxisMin = 0x1E4; // Vector
            public const nint m_vecRotAxisMax = 0x1F0; // Vector
            public const nint m_flRotRateMin = 0x1FC; // float32
            public const nint m_flRotRateMax = 0x200; // float32
            public const nint m_bNormalize = 0x204; // bool
            public const nint m_flScale = 0x208; // CPerParticleFloatInput
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
        // MVectorIsSometimesCoordinate
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
        // MPropertyFriendlyName
        // MPropertySuppressExpr
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
        // MPropertyAttributeRange
        // MPropertyFriendlyName
        // MPropertyAttributeChoiceName
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
        public static class C_INIT_InitVecCollection {
            public const nint m_InputValue = 0x1E8; // CParticleCollectionVecInput
            public const nint m_nOutputField = 0x8C0; // ParticleAttributeIndex_t
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MVectorIsSometimesCoordinate
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
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertySuppressExpr
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        public static class C_INIT_RemapParticleCountToNamedModelMeshGroupScalar {
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyAttributeRange
        public static class C_INIT_SequenceFromCP {
            public const nint m_bKillUnused = 0x1E8; // bool
            public const nint m_bRadiusScale = 0x1E9; // bool
            public const nint m_nCP = 0x1EC; // int32
            public const nint m_vecOffset = 0x1F0; // Vector
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
        // MPropertyAttributeChoiceName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MPropertyAttributeChoiceName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MParticleMaxVersion
        // MParticleReplacementOp
        // MGetKV3ClassDefaults
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
        // MPropertyAttributeChoiceName
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
        public static class C_OP_CPOffsetToPercentageBetweenCPs {
            public const nint m_flInputMin = 0x1E0; // float32
            public const nint m_flInputMax = 0x1E4; // float32
            public const nint m_flInputBias = 0x1E8; // float32
            public const nint m_nStartCP = 0x1EC; // int32
            public const nint m_nEndCP = 0x1F0; // int32
            public const nint m_nOffsetCP = 0x1F4; // int32
            public const nint m_nOuputCP = 0x1F8; // int32
            public const nint m_nInputCP = 0x1FC; // int32
            public const nint m_bRadialCheck = 0x200; // bool
            public const nint m_bScaleOffset = 0x201; // bool
            public const nint m_vecOffset = 0x204; // Vector
        }
    }
}
