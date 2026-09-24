// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-24 17:35:37.601127800 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: particles.dll
        // Class count: 338
        // Enum count: 77
        pub const particles_dll = struct {
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
            // Member count: 2
            pub const PulseCursorWakePriority_t = enum(u32) {
                WakeElegantly = 0x0,
                WakeImmediate = 0x1
            };
            // Alignment: 4
            // Member count: 7
            pub const Detail2Combo_t = enum(u32) {
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
            pub const MissingParentInheritBehavior_t = enum(u32) {
                MISSING_PARENT_DO_NOTHING = 0xFFFFFFFF,
                MISSING_PARENT_KILL = 0x0,
                MISSING_PARENT_FIND_NEW = 0x1,
                MISSING_PARENT_SAME_INDEX = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleTraceMissBehavior_t = enum(u32) {
                PARTICLE_TRACE_MISS_BEHAVIOR_NONE = 0x0,
                PARTICLE_TRACE_MISS_BEHAVIOR_KILL = 0x1,
                PARTICLE_TRACE_MISS_BEHAVIOR_TRACE_END = 0x2
            };
            // Alignment: 4
            // Member count: 7
            pub const PFuncVisualizationType_t = enum(u32) {
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
            pub const ParticleVRHandChoiceList_t = enum(u32) {
                PARTICLE_VRHAND_LEFT = 0x0,
                PARTICLE_VRHAND_RIGHT = 0x1,
                PARTICLE_VRHAND_CP = 0x2,
                PARTICLE_VRHAND_CP_OBJECT = 0x3
            };
            // Alignment: 4
            // Member count: 2
            pub const ParticleReplicationMode_t = enum(u32) {
                PARTICLE_REPLICATIONMODE_NONE = 0x0,
                PARTICLE_REPLICATIONMODE_REPLICATE_FOR_EACH_PARENT_PARTICLE = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleEntityPos_t = enum(u32) {
                PARTICLE_ABS_ORIGIN = 0x0,
                PARTICLE_WORLDSPACE_CENTER = 0x1,
                PARTICLE_EYES = 0x2,
                PARTICLE_FLASHLIGHT = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleFanType_t = enum(u32) {
                PARTICLE_FAN_TYPE_FAN = 0x0,
                PARTICLE_FAN_TYPE_ROTOR_WASH = 0x1,
                PARTICLE_FAN_TYPE_RADIAL = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const PetGroundType_t = enum(u32) {
                PET_GROUND_NONE = 0x0,
                PET_GROUND_GRID = 0x1,
                PET_GROUND_PLANE = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const InheritableBoolType_t = enum(u32) {
                INHERITABLE_BOOL_INHERIT = 0x0,
                INHERITABLE_BOOL_FALSE = 0x1,
                INHERITABLE_BOOL_TRUE = 0x2
            };
            // Alignment: 4
            // Member count: 6
            pub const ParticlePostProcessPriorityGroup_t = enum(u32) {
                PARTICLE_POST_PROCESS_PRIORITY_LEVEL_VOLUME = 0x0,
                PARTICLE_POST_PROCESS_PRIORITY_LEVEL_OVERRIDE = 0x1,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_EFFECT = 0x2,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_LOW = 0x3,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_HIGH = 0x4,
                PARTICLE_POST_PROCESS_PRIORITY_GLOBAL_UI = 0x5
            };
            // Alignment: 4
            // Member count: 7
            pub const ParticleCollisionGroup_t = enum(u32) {
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
            pub const DetailCombo_t = enum(u32) {
                DETAIL_COMBO_OFF = 0x0,
                DETAIL_COMBO_ADD = 0x1,
                DETAIL_COMBO_ADD_SELF_ILLUM = 0x2,
                DETAIL_COMBO_MOD2X = 0x3
            };
            // Alignment: 4
            // Member count: 12
            pub const ScalarExpressionType_t = enum(u32) {
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
            pub const SpriteCardPerParticleScale_t = enum(u32) {
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
            pub const BlurFilterType_t = enum(u32) {
                BLURFILTER_GAUSSIAN = 0x0,
                BLURFILTER_BOX = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const StandardLightingAttenuationStyle_t = enum(u32) {
                LIGHT_STYLE_OLD = 0x0,
                LIGHT_STYLE_NEW = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleParentSetMode_t = enum(u32) {
                PARTICLE_SET_PARENT_NO = 0x0,
                PARTICLE_SET_PARENT_IMMEDIATE = 0x1,
                PARTICLE_SET_PARENT_ROOT = 0x2
            };
            // Alignment: 4
            // Member count: 6
            pub const ParticleLightingQuality_t = enum(u32) {
                PARTICLE_LIGHTING_PER_PARTICLE = 0x0,
                PARTICLE_LIGHTING_PER_VERTEX = 0x1,
                PARTICLE_LIGHTING_PER_PIXEL = 0xFFFFFFFF,
                PARTICLE_LIGHTING_OVERRIDE_POSITION = 0x2,
                PARTICLE_LIGHTING_OVERRIDE_COLOR = 0x3,
                PARTICLE_LIGHTING_ADD_EXTRA_LIGHT = 0x4
            };
            // Alignment: 4
            // Member count: 2
            pub const ParticleVolumetricSmokeCreationType_t = enum(u32) {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_CONTINUOUS = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_IMPULSE = 0x1
            };
            // Alignment: 4
            // Member count: 8
            pub const SetStatisticExpressionType_t = enum(u32) {
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
            pub const EventTypeSelection_t = enum(u32) {
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
            pub const ParticleMassMode_t = enum(u32) {
                PARTICLE_MASSMODE_RADIUS_CUBED = 0x0,
                PARTICLE_MASSMODE_RADIUS_SQUARED = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const ParticleHitboxBiasType_t = enum(u32) {
                PARTICLE_HITBOX_BIAS_ENTITY = 0x0,
                PARTICLE_HITBOX_BIAS_HITBOX = 0x1
            };
            // Alignment: 4
            // Member count: 6
            pub const ParticleControlPointAxis_t = enum(u32) {
                PARTICLE_CP_AXIS_X = 0x0,
                PARTICLE_CP_AXIS_Y = 0x1,
                PARTICLE_CP_AXIS_Z = 0x2,
                PARTICLE_CP_AXIS_NEGATIVE_X = 0x3,
                PARTICLE_CP_AXIS_NEGATIVE_Y = 0x4,
                PARTICLE_CP_AXIS_NEGATIVE_Z = 0x5
            };
            // Alignment: 4
            // Member count: 12
            pub const ParticlePinDistance_t = enum(u32) {
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
            pub const VectorFloatExpressionType_t = enum(u32) {
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
            pub const ParticleFogType_t = enum(u32) {
                PARTICLE_FOG_GAME_DEFAULT = 0x0,
                PARTICLE_FOG_ENABLED = 0x1,
                PARTICLE_FOG_DISABLED = 0x2
            };
            // Alignment: 4
            // Member count: 10
            pub const VectorExpressionType_t = enum(u32) {
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
            pub const ParticleMultiSegmentInputSelection_t = enum(u32) {
                PARTICLE_MULTISEGMENT_SELECTION_FLOAT = 0x0,
                PARTICLE_MULTISEGMENT_SELECTION_STRING = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleRotationLockType_t = enum(u32) {
                PARTICLE_ROTATION_LOCK_NONE = 0x0,
                PARTICLE_ROTATION_LOCK_ROTATIONS = 0x1,
                PARTICLE_ROTATION_LOCK_NORMAL = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const HitboxLerpType_t = enum(u32) {
                HITBOX_LERP_LIFETIME = 0x0,
                HITBOX_LERP_CONSTANT = 0x1
            };
            // Alignment: 4
            // Member count: 11
            pub const ParticleAttrBoxFlags_t = enum(u32) {
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
            pub const ParticleTopology_t = enum(u32) {
                PARTICLE_TOPOLOGY_POINTS = 0x0,
                PARTICLE_TOPOLOGY_LINES = 0x1,
                PARTICLE_TOPOLOGY_TRIS = 0x2,
                PARTICLE_TOPOLOGY_QUADS = 0x3,
                PARTICLE_TOPOLOGY_CUBES = 0x4
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleLightBehaviorChoiceList_t = enum(u32) {
                PARTICLE_LIGHT_BEHAVIOR_FOLLOW_DIRECTION = 0x0,
                PARTICLE_LIGHT_BEHAVIOR_ROPE = 0x1,
                PARTICLE_LIGHT_BEHAVIOR_TRAILS = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const ModelHitboxType_t = enum(u32) {
                MODEL_HITBOX_TYPE_STANDARD = 0x0,
                MODEL_HITBOX_TYPE_RAW_BONES = 0x1,
                MODEL_HITBOX_TYPE_RENDERBOUNDS = 0x2,
                MODEL_HITBOX_TYPE_SNAPSHOT = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleMultiSegmentCountSelection_t = enum(u32) {
                PARTICLE_MULTISEGMENT_SEG_COUNT_7 = 0x7,
                PARTICLE_MULTISEGMENT_SEG_COUNT_14 = 0xE,
                PARTICLE_MULTISEGMENT_SEG_COUNT_16 = 0x10
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleOrientationType_t = enum(u32) {
                PARTICLE_ORIENTATION_NONE = 0x0,
                PARTICLE_ORIENTATION_VELOCITY = 0x1,
                PARTICLE_ORIENTATION_NORMAL = 0x2,
                PARTICLE_ORIENTATION_ROTATION = 0x4
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleTraceSet_t = enum(u32) {
                PARTICLE_TRACE_SET_ALL = 0x0,
                PARTICLE_TRACE_SET_STATIC = 0x1,
                PARTICLE_TRACE_SET_STATIC_AND_KEYFRAMED = 0x2,
                PARTICLE_TRACE_SET_DYNAMIC = 0x3
            };
            // Alignment: 4
            // Member count: 7
            pub const ParticleTextureLayerBlendType_t = enum(u32) {
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
            pub const ParticleSelection_t = enum(u32) {
                PARTICLE_SELECTION_FIRST = 0x0,
                PARTICLE_SELECTION_LAST = 0x1,
                PARTICLE_SELECTION_NUMBER = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleToolsState_t = enum(u32) {
                PARTICLE_TOOLS_STATE_ALWAYS_ON = 0xFFFFFFFF,
                PARTICLE_TOOLS_STATE_TOOLS_ONLY = 0x0,
                PARTICLE_TOOLS_STATE_GAME_ONLY = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const SnapshotIndexType_t = enum(u32) {
                SNAPSHOT_INDEX_INCREMENT = 0x0,
                SNAPSHOT_INDEX_DIRECT = 0x1
            };
            // Alignment: 4
            // Member count: 7
            pub const ParticleOutputBlendMode_t = enum(u32) {
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
            pub const ParticleLightnintBranchBehavior_t = enum(u32) {
                PARTICLE_LIGHTNING_BRANCH_CURRENT_DIR = 0x0,
                PARTICLE_LIGHTNING_BRANCH_ENDPOINT_DIR = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const MaterialProxyType_t = enum(u32) {
                MATERIAL_PROXY_STATUS_EFFECT = 0x0,
                MATERIAL_PROXY_TINT = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleDepthFeatheringMode_t = enum(u32) {
                PARTICLE_DEPTH_FEATHERING_OFF = 0x0,
                PARTICLE_DEPTH_FEATHERING_ON_OPTIONAL = 0x1,
                PARTICLE_DEPTH_FEATHERING_ON_REQUIRED = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const ParticleLightUnitChoiceList_t = enum(u32) {
                PARTICLE_LIGHT_UNIT_CANDELAS = 0x0,
                PARTICLE_LIGHT_UNIT_LUMENS = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleMultiSegmentSpecialCharacter_t = enum(u32) {
                PARTICLE_MULTISEGMENT_SPECIAL_NONE = 0xFFFFFFFF,
                PARTICLE_MULTISEGMENT_SPECIAL_DECIMAL = 0x0,
                PARTICLE_MULTISEGMENT_SPECIAL_COLON = 0x1,
                PARTICLE_MULTISEGMENT_SPECIAL_DEGREES = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleFalloffFunction_t = enum(u32) {
                PARTICLE_FALLOFF_CONSTANT = 0x0,
                PARTICLE_FALLOFF_LINEAR = 0x1,
                PARTICLE_FALLOFF_EXPONENTIAL = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleSequenceCropOverride_t = enum(u32) {
                PARTICLE_SEQUENCE_CROP_OVERRIDE_DEFAULT = 0xFFFFFFFF,
                PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_OFF = 0x0,
                PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_ON = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleDetailLevel_t = enum(u32) {
                PARTICLEDETAIL_LOW = 0x0,
                PARTICLEDETAIL_MEDIUM = 0x1,
                PARTICLEDETAIL_HIGH = 0x2,
                PARTICLEDETAIL_ULTRA = 0x3
            };
            // Alignment: 4
            // Member count: 4
            pub const BBoxVolumeType_t = enum(u32) {
                BBOX_VOLUME = 0x0,
                BBOX_DIMENSIONS = 0x1,
                BBOX_MINS_MAXS = 0x2,
                BBOX_RADIUS = 0x3
            };
            // Alignment: 4
            // Member count: 12
            pub const SpriteCardTextureType_t = enum(u32) {
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
            pub const ParticleAlphaReferenceType_t = enum(u32) {
                PARTICLE_ALPHA_REFERENCE_ALPHA_ALPHA = 0x0,
                PARTICLE_ALPHA_REFERENCE_OPAQUE_ALPHA = 0x1,
                PARTICLE_ALPHA_REFERENCE_ALPHA_OPAQUE = 0x2,
                PARTICLE_ALPHA_REFERENCE_OPAQUE_OPAQUE = 0x3
            };
            // Alignment: 4
            // Member count: 15
            pub const SpriteCardTextureChannel_t = enum(u32) {
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
            pub const ParticleVolumetricSmokeType_t = enum(u32) {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_EMISSION = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_SINK = 0x1,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_REPEL = 0x2,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_TRACE = 0x3
            };
            // Alignment: 4
            // Member count: 4
            pub const RenderModelSubModelFieldType_t = enum(u32) {
                SUBMODEL_AS_BODYGROUP_SUBMODEL = 0x0,
                SUBMODEL_AS_MESHGROUP_INDEX = 0x1,
                SUBMODEL_AS_MESHGROUP_MASK = 0x2,
                SUBMODEL_IGNORED_USE_MODEL_DEFAULT_MESHGROUP_MASK = 0x3
            };
            // Alignment: 4
            // Member count: 2
            pub const ParticleHitboxDataSelection_t = enum(u32) {
                PARTICLE_HITBOX_AVERAGE_SPEED = 0x0,
                PARTICLE_HITBOX_COUNT = 0x1
            };
            // Alignment: 4
            // Member count: 6
            pub const ParticleOrientationChoiceList_t = enum(u32) {
                PARTICLE_ORIENTATION_SCREEN_ALIGNED = 0x0,
                PARTICLE_ORIENTATION_SCREEN_Z_ALIGNED = 0x1,
                PARTICLE_ORIENTATION_WORLD_Z_ALIGNED = 0x2,
                PARTICLE_ORIENTATION_ALIGN_TO_PARTICLE_NORMAL = 0x3,
                PARTICLE_ORIENTATION_SCREENALIGN_TO_PARTICLE_NORMAL = 0x4,
                PARTICLE_ORIENTATION_FULL_3AXIS_ROTATION = 0x5
            };
            // Alignment: 4
            // Member count: 5
            pub const ParticleCollisionMode_t = enum(u32) {
                COLLISION_MODE_PER_PARTICLE_TRACE = 0x3,
                COLLISION_MODE_USE_NEAREST_TRACE = 0x2,
                COLLISION_MODE_PER_FRAME_PLANESET = 0x1,
                COLLISION_MODE_INITIAL_TRACE_DOWN = 0x0,
                COLLISION_MODE_DISABLED = 0xFFFFFFFF
            };
            // Alignment: 4
            // Member count: 2
            pub const ParticleSortingChoiceList_t = enum(u32) {
                PARTICLE_SORTING_NEAREST = 0x0,
                PARTICLE_SORTING_CREATION_TIME = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleEndcapMode_t = enum(u32) {
                PARTICLE_ENDCAP_ALWAYS_ON = 0xFFFFFFFF,
                PARTICLE_ENDCAP_ENDCAP_OFF = 0x0,
                PARTICLE_ENDCAP_ENDCAP_ON = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const ClosestPointTestType_t = enum(u32) {
                PARTICLE_CLOSEST_TYPE_BOX = 0x0,
                PARTICLE_CLOSEST_TYPE_CAPSULE = 0x1,
                PARTICLE_CLOSEST_TYPE_HYBRID = 0x2
            };
            // Alignment: 4
            // Member count: 6
            pub const ParticleImpulseType_t = enum(u32) {
                IMPULSE_TYPE_NONE = 0x0,
                IMPULSE_TYPE_GENERIC = 0x1,
                IMPULSE_TYPE_ROPE = 0x2,
                IMPULSE_TYPE_EXPLOSION = 0x4,
                IMPULSE_TYPE_EXPLOSION_UNDERWATER = 0x8,
                IMPULSE_TYPE_PARTICLE_SYSTEM = 0x10
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleLiquidContents_t = enum(u32) {
                PARTICLE_LIQUID_NONE = 0x0,
                PARTICLE_LIQUID_OIL = 0x1,
                PARTICLE_LIQUID_WATER = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const SpriteCardShaderType_t = enum(u32) {
                SPRITECARD_SHADER_BASE = 0x0,
                SPRITECARD_SHADER_CUSTOM = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const ParticleOmni2LightTypeChoiceList_t = enum(u32) {
                PARTICLE_OMNI2_LIGHT_TYPE_POINT = 0x0,
                PARTICLE_OMNI2_LIGHT_TYPE_SPHERE = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleLightFogLightingMode_t = enum(u32) {
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_NONE = 0x0,
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC = 0x2,
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC_NOSHADOWS = 0x4
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleLightTypeChoiceList_t = enum(u32) {
                PARTICLE_LIGHT_TYPE_POINT = 0x0,
                PARTICLE_LIGHT_TYPE_SPOT = 0x1,
                PARTICLE_LIGHT_TYPE_FX = 0x2,
                PARTICLE_LIGHT_TYPE_CAPSULE = 0x3
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleOrientationSetMode_t = enum(u32) {
                PARTICLE_ORIENTATION_SET_NONE = 0xFFFFFFFF,
                PARTICLE_ORIENTATION_SET_FROM_VELOCITY = 0x0,
                PARTICLE_ORIENTATION_SET_FROM_NORMAL = 0x1,
                PARTICLE_ORIENTATION_SET_FROM_ROTATIONS = 0x2
            };
            // Alignment: 8
            // Member count: 10
            pub const ParticleCollisionMask_t = enum(u64) {
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
            pub const TextureRepetitionMode_t = enum(u32) {
                TEXTURE_REPETITION_PARTICLE = 0x0,
                TEXTURE_REPETITION_PATH = 0x1
            };
            // Parent: None
            // Field count: 0
            pub const CPulse_ResumePoint = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CParticleBindingRealPulse = struct {
            };
            // Parent: None
            // Field count: 4
            pub const CPulse_OutflowConnection = struct {
                pub const m_SourceOutflowName: usize = 0x0; // PulseSymbol_t
                pub const m_nDestChunk: usize = 0x10; // PulseRuntimeChunkIndex_t
                pub const m_nInstruction: usize = 0x14; // int32
                pub const m_OutflowRegisterMap: usize = 0x18; // PulseRegisterMap_t
            };
            // Parent: None
            // Field count: 0
            pub const CBasePulseGraphInstance = struct {
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Resume = struct {
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Continue = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CParticleCollectionBindingInstance = struct {
            };
            // Parent: None
            // Field count: 1
            pub const CPulseCell_IsRequirementValid__Criteria_t = struct {
                pub const m_bIsValid: usize = 0x0; // bool
            };
            // Parent: None
            // Field count: 1
            pub const CPulseCell_Unknown = struct {
                pub const m_UnknownKeys: usize = 0x48; // KeyValues3
            };
            // Parent: None
            // Field count: 1
            pub const CPulseCell_LimitCount__Criteria_t = struct {
                pub const m_bLimitCountPasses: usize = 0x0; // bool
            };
            // Parent: None
            // Field count: 0
            pub const CPulseExecCursor = struct {
            };
            // Parent: None
            // Field count: 0
            pub const IParticleCollection = struct {
            };
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
            pub const C_OP_Decay = struct {
                pub const m_bRopeDecay: usize = 0x1D8; // bool
                pub const m_bForcePreserveParticleOrder: usize = 0x1D9; // bool
            };
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
            pub const C_OP_RenderDeferredLight = struct {
                pub const m_bUseAlphaTestWindow: usize = 0x228; // bool
                pub const m_bUseTexture: usize = 0x229; // bool
                pub const m_flRadiusScale: usize = 0x22C; // float32
                pub const m_flAlphaScale: usize = 0x230; // float32
                pub const m_nAlpha2Field: usize = 0x234; // ParticleAttributeIndex_t
                pub const m_vecColorScale: usize = 0x238; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0x8F0; // ParticleColorBlendType_t
                pub const m_flLightDistance: usize = 0x8F4; // float32
                pub const m_flStartFalloff: usize = 0x8F8; // float32
                pub const m_flDistanceFalloff: usize = 0x8FC; // float32
                pub const m_flSpotFoV: usize = 0x900; // float32
                pub const m_nAlphaTestPointField: usize = 0x904; // ParticleAttributeIndex_t
                pub const m_nAlphaTestRangeField: usize = 0x908; // ParticleAttributeIndex_t
                pub const m_nAlphaTestSharpnessField: usize = 0x90C; // ParticleAttributeIndex_t
                pub const m_hTexture: usize = 0x910; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_nHSVShiftControlPoint: usize = 0x918; // int32
            };
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
            pub const C_OP_RemapTransformToVelocity = struct {
                pub const m_TransformInput: usize = 0x1D8; // CParticleTransformInput
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            pub const CollisionGroupContext_t = struct {
                pub const m_nCollisionGroupNumber: usize = 0x0; // int32
            };
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
            pub const C_INIT_StatusEffectCitadel = struct {
                pub const m_flSFXColorWarpAmount: usize = 0x1E0; // float32
                pub const m_flSFXNormalAmount: usize = 0x1E4; // float32
                pub const m_flSFXMetalnessAmount: usize = 0x1E8; // float32
                pub const m_flSFXRoughnessAmount: usize = 0x1EC; // float32
                pub const m_flSFXSelfIllumAmount: usize = 0x1F0; // float32
                pub const m_flSFXSScale: usize = 0x1F4; // float32
                pub const m_flSFXSScrollX: usize = 0x1F8; // float32
                pub const m_flSFXSScrollY: usize = 0x1FC; // float32
                pub const m_flSFXSScrollZ: usize = 0x200; // float32
                pub const m_flSFXSOffsetX: usize = 0x204; // float32
                pub const m_flSFXSOffsetY: usize = 0x208; // float32
                pub const m_flSFXSOffsetZ: usize = 0x20C; // float32
                pub const m_nDetailCombo: usize = 0x210; // DetailCombo_t
                pub const m_flSFXSDetailAmount: usize = 0x214; // float32
                pub const m_flSFXSDetailScale: usize = 0x218; // float32
                pub const m_flSFXSDetailScrollX: usize = 0x21C; // float32
                pub const m_flSFXSDetailScrollY: usize = 0x220; // float32
                pub const m_flSFXSDetailScrollZ: usize = 0x224; // float32
                pub const m_flSFXSUseModelUVs: usize = 0x228; // float32
            };
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
            pub const C_OP_RenderSound = struct {
                pub const m_flDurationScale: usize = 0x228; // float32
                pub const m_flSndLvlScale: usize = 0x22C; // float32
                pub const m_flPitchScale: usize = 0x230; // float32
                pub const m_flVolumeScale: usize = 0x234; // float32
                pub const m_nSndLvlField: usize = 0x238; // ParticleAttributeIndex_t
                pub const m_nDurationField: usize = 0x23C; // ParticleAttributeIndex_t
                pub const m_nPitchField: usize = 0x240; // ParticleAttributeIndex_t
                pub const m_nVolumeField: usize = 0x244; // ParticleAttributeIndex_t
                pub const m_nChannel: usize = 0x248; // int32
                pub const m_nCPReference: usize = 0x24C; // int32
                pub const m_pszSoundName: usize = 0x250; // char[256]
                pub const m_bSuppressStopSoundEvent: usize = 0x350; // bool
            };
            // Parent: None
            // Field count: 19
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // COLLISION_MODE_PER_PARTICLE_TRACE
            // COLLISION_MODE_USE_NEAREST_TRACE
            pub const CParticleVisibilityInputs = struct {
                pub const m_flCameraBias: usize = 0x0; // float32
                pub const m_nCPin: usize = 0x4; // int32
                pub const m_flProxyRadius: usize = 0x8; // float32
                pub const m_flInputMin: usize = 0xC; // float32
                pub const m_flInputMax: usize = 0x10; // float32
                pub const m_flInputPixelVisFade: usize = 0x14; // float32
                pub const m_flNoPixelVisibilityFallback: usize = 0x18; // float32
                pub const m_flDistanceInputMin: usize = 0x1C; // float32
                pub const m_flDistanceInputMax: usize = 0x20; // float32
                pub const m_flDotInputMin: usize = 0x24; // float32
                pub const m_flDotInputMax: usize = 0x28; // float32
                pub const m_bDotCPAngles: usize = 0x2C; // bool
                pub const m_bDotCameraAngles: usize = 0x2D; // bool
                pub const m_flAlphaScaleMin: usize = 0x30; // float32
                pub const m_flAlphaScaleMax: usize = 0x34; // float32
                pub const m_flRadiusScaleMin: usize = 0x38; // float32
                pub const m_flRadiusScaleMax: usize = 0x3C; // float32
                pub const m_flRadiusScaleFOVBase: usize = 0x40; // float32
                pub const m_bRightEye: usize = 0x44; // bool
            };
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
            pub const C_OP_SetControlPointsToParticle = struct {
                pub const m_nChildGroupID: usize = 0x1D8; // int32
                pub const m_nFirstControlPoint: usize = 0x1DC; // int32
                pub const m_nNumControlPoints: usize = 0x1E0; // int32
                pub const m_nFirstSourcePoint: usize = 0x1E4; // int32
                pub const m_bReverse: usize = 0x1E8; // bool
                pub const m_bSetOrientation: usize = 0x1E9; // bool
                pub const m_nOrientationMode: usize = 0x1EC; // ParticleOrientationSetMode_t
                pub const m_nSetParent: usize = 0x1F0; // ParticleParentSetMode_t
            };
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
            pub const C_OP_RemapCPVelocityToVector = struct {
                pub const m_nControlPoint: usize = 0x1D8; // int32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1E0; // float32
                pub const m_bNormalize: usize = 0x1E4; // bool
            };
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
            pub const C_OP_PointVectorAtNextParticle = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInterpolation: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_bPrevious: usize = 0x350; // bool
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // PARTICLE_WORLDSPACE_CENTER
            // PARTICLE_EYES
            // PARTICLE_FLASHLIGHT
            pub const ParticlePreviewBodyGroup_t = struct {
                pub const m_bodyGroupName: usize = 0x0; // CUtlString
                pub const m_nValue: usize = 0x8; // int32
            };
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
            pub const C_OP_OscillateScalarSimple = struct {
                pub const m_Rate: usize = 0x1D8; // float32
                pub const m_Frequency: usize = 0x1DC; // float32
                pub const m_nField: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flOscMult: usize = 0x1E4; // float32
                pub const m_flOscAdd: usize = 0x1E8; // float32
            };
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
            pub const C_INIT_StatusEffect = struct {
                pub const m_nDetail2Combo: usize = 0x1E0; // Detail2Combo_t
                pub const m_flDetail2Rotation: usize = 0x1E4; // float32
                pub const m_flDetail2Scale: usize = 0x1E8; // float32
                pub const m_flDetail2BlendFactor: usize = 0x1EC; // float32
                pub const m_flColorWarpIntensity: usize = 0x1F0; // float32
                pub const m_flDiffuseWarpBlendToFull: usize = 0x1F4; // float32
                pub const m_flEnvMapIntensity: usize = 0x1F8; // float32
                pub const m_flAmbientScale: usize = 0x1FC; // float32
                pub const m_specularColor: usize = 0x200; // Color
                pub const m_flSpecularScale: usize = 0x204; // float32
                pub const m_flSpecularExponent: usize = 0x208; // float32
                pub const m_flSpecularExponentBlendToFull: usize = 0x20C; // float32
                pub const m_flSpecularBlendToFull: usize = 0x210; // float32
                pub const m_rimLightColor: usize = 0x214; // Color
                pub const m_flRimLightScale: usize = 0x218; // float32
                pub const m_flReflectionsTintByBaseBlendToNone: usize = 0x21C; // float32
                pub const m_flMetalnessBlendToFull: usize = 0x220; // float32
                pub const m_flSelfIllumBlendToFull: usize = 0x224; // float32
            };
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
            pub const C_OP_ConstrainDistance = struct {
                pub const m_fMinDistance: usize = 0x1D8; // CParticleCollectionFloatInput
                pub const m_fMaxDistance: usize = 0x348; // CParticleCollectionFloatInput
                pub const m_nControlPointNumber: usize = 0x4B8; // int32
                pub const m_CenterOffset: usize = 0x4BC; // Vector
                pub const m_bGlobalCenter: usize = 0x4C8; // bool
            };
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
            pub const C_INIT_RandomVector = struct {
                pub const m_vecMin: usize = 0x1E0; // Vector
                pub const m_vecMax: usize = 0x1EC; // Vector
                pub const m_nFieldOutput: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const m_randomnessParameters: usize = 0x1FC; // CRandomNumberGeneratorParameters
            };
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
            pub const C_INIT_InitialVelocityNoise = struct {
                pub const m_vecAbsVal: usize = 0x1E0; // Vector
                pub const m_vecAbsValInv: usize = 0x1EC; // Vector
                pub const m_vecOffsetLoc: usize = 0x1F8; // CPerParticleVecInput
                pub const m_flOffset: usize = 0x8B0; // CPerParticleFloatInput
                pub const m_vecOutputMin: usize = 0xA20; // CPerParticleVecInput
                pub const m_vecOutputMax: usize = 0x10D8; // CPerParticleVecInput
                pub const m_flNoiseScale: usize = 0x1790; // CPerParticleFloatInput
                pub const m_flNoiseScaleLoc: usize = 0x1900; // CPerParticleFloatInput
                pub const m_TransformInput: usize = 0x1A70; // CParticleTransformInput
                pub const m_bIgnoreDt: usize = 0x1AD8; // bool
            };
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
            pub const C_OP_RemapScalarOnceTimed = struct {
                pub const m_bProportional: usize = 0x1D8; // bool
                pub const m_nFieldInput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_flRemapTime: usize = 0x1F4; // float32
            };
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
            pub const C_INIT_RandomNamedModelSequence = struct {
            };
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
            pub const C_OP_PlaneCull = struct {
                pub const m_nPlaneControlPoint: usize = 0x1D8; // int32
                pub const m_vecPlaneDirection: usize = 0x1E0; // CParticleCollectionVecInput
                pub const m_bLocalSpace: usize = 0x898; // bool
                pub const m_flPlaneOffset: usize = 0x89C; // float32
            };
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
            pub const C_INIT_VelocityRandom = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_fSpeedMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_fSpeedMax: usize = 0x358; // CPerParticleFloatInput
                pub const m_LocalCoordinateSystemSpeedMin: usize = 0x4C8; // CPerParticleVecInput
                pub const m_LocalCoordinateSystemSpeedMax: usize = 0xB80; // CPerParticleVecInput
                pub const m_bIgnoreDT: usize = 0x1238; // bool
                pub const m_randomnessParameters: usize = 0x123C; // CRandomNumberGeneratorParameters
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
            };
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
            pub const C_OP_ModelDampenMovement = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_bBoundBox: usize = 0x1DC; // bool
                pub const m_bOutside: usize = 0x1DD; // bool
                pub const m_bUseBones: usize = 0x1DE; // bool
                pub const m_HitboxSetName: usize = 0x1DF; // char[128]
                pub const m_vecPosOffset: usize = 0x260; // CPerParticleVecInput
                pub const m_fDrag: usize = 0x918; // float32
            };
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
            pub const CSpinUpdateBase = struct {
            };
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
            pub const C_OP_OrientTo2dDirection = struct {
                pub const m_vecInput: usize = 0x1D8; // CPerParticleVecInput
                pub const m_flRotOffset: usize = 0x890; // float32
                pub const m_flSpinStrength: usize = 0x894; // float32
                pub const m_nFieldOutput: usize = 0x898; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_RemapDotProductToCP = struct {
                pub const m_nInputCP1: usize = 0x1E0; // int32
                pub const m_nInputCP2: usize = 0x1E4; // int32
                pub const m_nOutputCP: usize = 0x1E8; // int32
                pub const m_nOutVectorField: usize = 0x1EC; // int32
                pub const m_flInputMin: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_flInputMax: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_flOutputMin: usize = 0x4D0; // CParticleCollectionFloatInput
                pub const m_flOutputMax: usize = 0x640; // CParticleCollectionFloatInput
            };
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
            pub const C_INIT_RemapParticleCountToNamedModelElementScalar = struct {
                pub const m_hModel: usize = 0x210; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_outputMinName: usize = 0x218; // CUtlString
                pub const m_outputMaxName: usize = 0x220; // CUtlString
                pub const m_bModelFromRenderer: usize = 0x228; // bool
            };
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
            pub const C_OP_SetControlPointPositionToTimeOfDayValue = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_pszTimeOfDayParameter: usize = 0x1E4; // char[128]
                pub const m_vecDefaultValue: usize = 0x264; // Vector
            };
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
            pub const C_OP_DecayMaintainCount = struct {
                pub const m_nParticlesToMaintain: usize = 0x1D8; // int32
                pub const m_flDecayDelay: usize = 0x1DC; // float32
                pub const m_nSnapshotControlPoint: usize = 0x1E0; // int32
                pub const m_strSnapshotSubset: usize = 0x1E8; // CUtlString
                pub const m_bLifespanDecay: usize = 0x1F0; // bool
                pub const m_flScale: usize = 0x1F8; // CParticleCollectionFloatInput
                pub const m_bKillNewest: usize = 0x368; // bool
            };
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
            pub const C_INIT_RandomModelSequence = struct {
                pub const m_ActivityName: usize = 0x1E0; // char[256]
                pub const m_SequenceName: usize = 0x2E0; // char[256]
                pub const m_hModel: usize = 0x3E0; // CStrongHandle<InfoForResourceTypeCModel>
            };
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
            pub const C_OP_ExternalGameImpulseForce = struct {
                pub const m_flForceScale: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_bRopes: usize = 0x358; // bool
                pub const m_bRopesZOnly: usize = 0x359; // bool
                pub const m_bExplosions: usize = 0x35A; // bool
                pub const m_bParticles: usize = 0x35B; // bool
            };
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
            pub const C_OP_RemapAverageHitboxSpeedtoCP = struct {
                pub const m_nInControlPointNumber: usize = 0x1E0; // int32
                pub const m_nOutControlPointNumber: usize = 0x1E4; // int32
                pub const m_nField: usize = 0x1E8; // int32
                pub const m_nHitboxDataType: usize = 0x1EC; // ParticleHitboxDataSelection_t
                pub const m_flInputMin: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_flInputMax: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_flOutputMin: usize = 0x4D0; // CParticleCollectionFloatInput
                pub const m_flOutputMax: usize = 0x640; // CParticleCollectionFloatInput
                pub const m_nHeightControlPointNumber: usize = 0x7B0; // int32
                pub const m_vecComparisonVelocity: usize = 0x7B8; // CParticleCollectionVecInput
                pub const m_HitboxSetName: usize = 0xE70; // char[128]
            };
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
            pub const C_INIT_RandomAlpha = struct {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nAlphaMin: usize = 0x1E4; // int32
                pub const m_nAlphaMax: usize = 0x1E8; // int32
                pub const m_flAlphaRandExponent: usize = 0x1F4; // float32
            };
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
            pub const C_OP_NormalizeVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1DC; // float32
            };
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
            pub const C_OP_RepeatedTriggerChildGroup = struct {
                pub const m_nChildGroupID: usize = 0x1E0; // int32
                pub const m_flClusterRefireTime: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_flClusterSize: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_flClusterCooldown: usize = 0x4C8; // CParticleCollectionFloatInput
                pub const m_bLimitChildCount: usize = 0x638; // bool
            };
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
            pub const C_OP_RemapVelocityToVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1DC; // float32
                pub const m_bNormalize: usize = 0x1E0; // bool
            };
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
            pub const C_INIT_SetHitboxToClosest = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nDesiredHitbox: usize = 0x1E4; // int32
                pub const m_vecHitBoxScale: usize = 0x1E8; // CParticleCollectionVecInput
                pub const m_HitboxSetName: usize = 0x8A0; // char[128]
                pub const m_bUseBones: usize = 0x920; // bool
                pub const m_bUseClosestPointOnHitbox: usize = 0x921; // bool
                pub const m_nTestType: usize = 0x924; // ClosestPointTestType_t
                pub const m_flHybridRatio: usize = 0x928; // CParticleCollectionFloatInput
                pub const m_bUpdatePosition: usize = 0xA98; // bool
            };
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
            pub const C_INIT_RingWave = struct {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_flParticlesPerOrbit: usize = 0x248; // CParticleCollectionFloatInput
                pub const m_flInitialRadius: usize = 0x3B8; // CPerParticleFloatInput
                pub const m_flThickness: usize = 0x528; // CPerParticleFloatInput
                pub const m_flInitialSpeedMin: usize = 0x698; // CPerParticleFloatInput
                pub const m_flInitialSpeedMax: usize = 0x808; // CPerParticleFloatInput
                pub const m_flRoll: usize = 0x978; // CPerParticleFloatInput
                pub const m_flPitch: usize = 0xAE8; // CPerParticleFloatInput
                pub const m_flYaw: usize = 0xC58; // CPerParticleFloatInput
                pub const m_bEvenDistribution: usize = 0xDC8; // bool
                pub const m_bXYVelocityOnly: usize = 0xDC9; // bool
            };
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
            pub const C_INIT_RandomTrailLength = struct {
                pub const m_flMinLength: usize = 0x1E0; // float32
                pub const m_flMaxLength: usize = 0x1E4; // float32
                pub const m_flLengthRandExponent: usize = 0x1E8; // float32
            };
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
            pub const C_OP_RemapScalar = struct {
                pub const m_nFieldInput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E0; // float32
                pub const m_flInputMax: usize = 0x1E4; // float32
                pub const m_flOutputMin: usize = 0x1E8; // float32
                pub const m_flOutputMax: usize = 0x1EC; // float32
                pub const m_bOldCode: usize = 0x1F0; // bool
            };
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
            pub const C_OP_DistanceBetweenTransforms = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_TransformStart: usize = 0x1E0; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x248; // CParticleTransformInput
                pub const m_flInputMin: usize = 0x2B0; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x420; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x590; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x700; // CPerParticleFloatInput
                pub const m_flMaxTraceLength: usize = 0x870; // float32
                pub const m_flLOSScale: usize = 0x874; // float32
                pub const m_CollisionGroupName: usize = 0x878; // char[128]
                pub const m_nTraceSet: usize = 0x8F8; // ParticleTraceSet_t
                pub const m_bLOS: usize = 0x8FC; // bool
                pub const m_nSetMethod: usize = 0x900; // ParticleSetMethod_t
            };
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
            pub const C_OP_DecayOffscreen = struct {
                pub const m_flOffscreenTime: usize = 0x1D8; // CParticleCollectionFloatInput
            };
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
            pub const C_INIT_CreateSequentialPath = struct {
                pub const m_fMaxDistance: usize = 0x1E0; // float32
                pub const m_flNumToAssign: usize = 0x1E4; // float32
                pub const m_bLoop: usize = 0x1E8; // bool
                pub const m_bCPPairs: usize = 0x1E9; // bool
                pub const m_bSaveOffset: usize = 0x1EA; // bool
                pub const m_PathParams: usize = 0x1F0; // CPathParameters
            };
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
            pub const C_OP_EndCapTimedDecay = struct {
                pub const m_flDecayTime: usize = 0x1D8; // float32
            };
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
            pub const C_OP_ContinuousEmitter = struct {
                pub const m_flEmissionDuration: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_flStartTime: usize = 0x350; // CParticleCollectionFloatInput
                pub const m_flEmitRate: usize = 0x4C0; // CParticleCollectionFloatInput
                pub const m_flEmissionScale: usize = 0x630; // float32
                pub const m_flScalePerParentParticle: usize = 0x634; // float32
                pub const m_bInitFromKilledParentParticles: usize = 0x638; // bool
                pub const m_nEventType: usize = 0x63C; // EventTypeSelection_t
                pub const m_nSnapshotControlPoint: usize = 0x640; // int32
                pub const m_strSnapshotSubset: usize = 0x648; // CUtlString
                pub const m_nLimitPerUpdate: usize = 0x650; // int32
                pub const m_bForceEmitOnFirstUpdate: usize = 0x654; // bool
                pub const m_bForceEmitOnLastUpdate: usize = 0x655; // bool
            };
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
            pub const C_OP_OscillateVectorSimple = struct {
                pub const m_Rate: usize = 0x1D8; // Vector
                pub const m_Frequency: usize = 0x1E4; // Vector
                pub const m_nField: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_flOscMult: usize = 0x1F4; // float32
                pub const m_flOscAdd: usize = 0x1F8; // float32
                pub const m_bOffset: usize = 0x1FC; // bool
            };
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
            pub const C_INIT_SequenceLifeTime = struct {
                pub const m_flFramerate: usize = 0x1E0; // float32
            };
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
            pub const C_INIT_MoveBetweenPoints = struct {
                pub const m_flSpeedMin: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flSpeedMax: usize = 0x350; // CPerParticleFloatInput
                pub const m_flEndSpread: usize = 0x4C0; // CPerParticleFloatInput
                pub const m_flStartOffset: usize = 0x630; // CPerParticleFloatInput
                pub const m_flEndOffset: usize = 0x7A0; // CPerParticleFloatInput
                pub const m_nEndControlPointNumber: usize = 0x910; // int32
                pub const m_bTrailBias: usize = 0x914; // bool
            };
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
            pub const C_OP_SetUserEvent = struct {
                pub const m_flInput: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_flRisingEdge: usize = 0x348; // CPerParticleFloatInput
                pub const m_nRisingEventType: usize = 0x4B8; // EventTypeSelection_t
                pub const m_flFallingEdge: usize = 0x4C0; // CPerParticleFloatInput
                pub const m_nFallingEventType: usize = 0x630; // EventTypeSelection_t
            };
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
            pub const C_OP_QuantizeFloat = struct {
                pub const m_InputValue: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x348; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_BasicMovement = struct {
                pub const m_Gravity: usize = 0x1D8; // CParticleCollectionVecInput
                pub const m_fDrag: usize = 0x890; // CParticleCollectionFloatInput
                pub const m_massControls: usize = 0xA00; // CParticleMassCalculationParameters
                pub const m_nMaxConstraintPasses: usize = 0xE58; // int32
                pub const m_bUseNewCode: usize = 0xE5C; // bool
            };
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
            pub const C_INIT_RandomNamedModelElement = struct {
                pub const m_hModel: usize = 0x1E0; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_names: usize = 0x1E8; // CUtlVector<CUtlString>
                pub const m_bShuffle: usize = 0x200; // bool
                pub const m_bLinear: usize = 0x201; // bool
                pub const m_bModelFromRenderer: usize = 0x202; // bool
                pub const m_nFieldOutput: usize = 0x204; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_InitFromParentKilled = struct {
                pub const m_nAttributeToCopy: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nEventType: usize = 0x1E4; // EventTypeSelection_t
            };
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
            pub const C_OP_Callback = struct {
            };
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
            pub const C_OP_GlobalLight = struct {
                pub const m_flScale: usize = 0x1D8; // float32
                pub const m_bClampLowerRange: usize = 0x1DC; // bool
                pub const m_bClampUpperRange: usize = 0x1DD; // bool
            };
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
            pub const C_INIT_OffsetVectorToVector = struct {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_vecOutputMin: usize = 0x1E8; // Vector
                pub const m_vecOutputMax: usize = 0x1F4; // Vector
                pub const m_randomnessParameters: usize = 0x200; // CRandomNumberGeneratorParameters
            };
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
            pub const C_OP_SetPerChildControlPointFromAttribute = struct {
                pub const m_nChildGroupID: usize = 0x1D8; // int32
                pub const m_nFirstControlPoint: usize = 0x1DC; // int32
                pub const m_nNumControlPoints: usize = 0x1E0; // int32
                pub const m_nParticleIncrement: usize = 0x1E4; // int32
                pub const m_nFirstSourcePoint: usize = 0x1E8; // int32
                pub const m_bNumBasedOnParticleCount: usize = 0x1EC; // bool
                pub const m_nAttributeToRead: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_nCPField: usize = 0x1F4; // int32
            };
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
            pub const C_OP_SetParentControlPointsToChildCP = struct {
                pub const m_nChildGroupID: usize = 0x1E0; // int32
                pub const m_nChildControlPoint: usize = 0x1E4; // int32
                pub const m_nNumControlPoints: usize = 0x1E8; // int32
                pub const m_nFirstSourcePoint: usize = 0x1EC; // int32
                pub const m_bSetOrientation: usize = 0x1F0; // bool
            };
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
            pub const C_OP_BoxConstraint = struct {
                pub const m_vecMin: usize = 0x1D8; // CParticleCollectionVecInput
                pub const m_vecMax: usize = 0x890; // CParticleCollectionVecInput
                pub const m_nCP: usize = 0xF48; // int32
                pub const m_bLocalSpace: usize = 0xF4C; // bool
                pub const m_bAccountForRadius: usize = 0xF4D; // bool
            };
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
            pub const C_INIT_CreatePhyllotaxis = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nScaleCP: usize = 0x1E4; // int32
                pub const m_nComponent: usize = 0x1E8; // int32
                pub const m_fRadCentCore: usize = 0x1EC; // float32
                pub const m_fRadPerPoint: usize = 0x1F0; // float32
                pub const m_fRadPerPointTo: usize = 0x1F4; // float32
                pub const m_fpointAngle: usize = 0x1F8; // float32
                pub const m_fsizeOverall: usize = 0x1FC; // float32
                pub const m_fRadBias: usize = 0x200; // float32
                pub const m_fMinRad: usize = 0x204; // float32
                pub const m_fDistBias: usize = 0x208; // float32
                pub const m_bUseLocalCoords: usize = 0x20C; // bool
                pub const m_bUseWithContEmit: usize = 0x20D; // bool
                pub const m_bUseOrigRadius: usize = 0x20E; // bool
            };
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
            pub const C_OP_AttractToControlPoint = struct {
                pub const m_vecComponentScale: usize = 0x1E8; // Vector
                pub const m_fForceAmount: usize = 0x1F8; // CPerParticleFloatInput
                pub const m_fMinimumDistance: usize = 0x368; // CPerParticleFloatInput
                pub const m_fFalloffPower: usize = 0x4D8; // float32
                pub const m_TransformInput: usize = 0x4E0; // CParticleTransformInput
                pub const m_fForceAmountMin: usize = 0x548; // CPerParticleFloatInput
                pub const m_bApplyMinForce: usize = 0x6B8; // bool
            };
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
            pub const C_INIT_RandomLifeTime = struct {
                pub const m_fLifetimeMin: usize = 0x1E0; // float32
                pub const m_fLifetimeMax: usize = 0x1E4; // float32
                pub const m_fLifetimeRandExponent: usize = 0x1E8; // float32
            };
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
            pub const C_INIT_RemapParticleCountToNamedModelSequenceScalar = struct {
            };
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
            pub const C_INIT_VelocityRadialRandom = struct {
                pub const m_bPerParticleCenter: usize = 0x1E0; // bool
                pub const m_nControlPointNumber: usize = 0x1E4; // int32
                pub const m_vecPosition: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vecFwd: usize = 0x8A0; // CPerParticleVecInput
                pub const m_fSpeedMin: usize = 0xF58; // CPerParticleFloatInput
                pub const m_fSpeedMax: usize = 0x10C8; // CPerParticleFloatInput
                pub const m_vecLocalCoordinateSystemSpeedScale: usize = 0x1238; // Vector
                pub const m_bIgnoreDelta: usize = 0x1245; // bool
            };
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
            pub const C_INIT_RandomRadius = struct {
                pub const m_flRadiusMin: usize = 0x1E0; // float32
                pub const m_flRadiusMax: usize = 0x1E4; // float32
                pub const m_flRadiusRandExponent: usize = 0x1E8; // float32
            };
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
            pub const C_OP_Orient2DRelToCP = struct {
                pub const m_flRotOffset: usize = 0x1D8; // float32
                pub const m_flSpinStrength: usize = 0x1DC; // float32
                pub const m_nCP: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // PARTICLE_TOPOLOGY_LINES
            pub const ControlPointReference_t = struct {
                pub const m_controlPointNameString: usize = 0x0; // int32
                pub const m_vOffsetFromControlPoint: usize = 0x4; // Vector
                pub const m_bOffsetInLocalSpace: usize = 0x10; // bool
            };
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
            pub const C_OP_LightningSnapshotGenerator = struct {
                pub const m_nCPSnapshot: usize = 0x1E0; // int32
                pub const m_nCPStartPnt: usize = 0x1E4; // int32
                pub const m_nCPEndPnt: usize = 0x1E8; // int32
                pub const m_flSegments: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_flOffset: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_flOffsetDecay: usize = 0x4D0; // CParticleCollectionFloatInput
                pub const m_flRecalcRate: usize = 0x640; // CParticleCollectionFloatInput
                pub const m_flUVScale: usize = 0x7B0; // CParticleCollectionFloatInput
                pub const m_flUVOffset: usize = 0x920; // CParticleCollectionFloatInput
                pub const m_flSplitRate: usize = 0xA90; // CParticleCollectionFloatInput
                pub const m_flRecursionSplitScale: usize = 0xC00; // CParticleCollectionFloatInput
                pub const m_bScaleBranchDistance: usize = 0xD70; // bool
                pub const m_flBranchDistanceScale: usize = 0xD78; // CParticleCollectionFloatInput
                pub const m_bScaleBranchOffset: usize = 0xEE8; // bool
                pub const m_flBranchOffsetScale: usize = 0xEF0; // CParticleCollectionFloatInput
                pub const m_flBranchTwist: usize = 0x1060; // CParticleCollectionFloatInput
                pub const m_nBranchBehavior: usize = 0x11D0; // ParticleLightnintBranchBehavior_t
                pub const m_flRadiusStart: usize = 0x11D8; // CParticleCollectionFloatInput
                pub const m_flRadiusEnd: usize = 0x1348; // CParticleCollectionFloatInput
                pub const m_flDedicatedPool: usize = 0x14B8; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_RemapNamedModelMeshGroupOnceTimed = struct {
            };
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
            pub const C_INIT_RemapQAnglesToRotation = struct {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
            };
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
            pub const C_INIT_PositionWarp = struct {
                pub const m_vecWarpMin: usize = 0x1E0; // CParticleCollectionVecInput
                pub const m_vecWarpMax: usize = 0x898; // CParticleCollectionVecInput
                pub const m_nScaleControlPointNumber: usize = 0xF50; // int32
                pub const m_nControlPointNumber: usize = 0xF54; // int32
                pub const m_nRadiusComponent: usize = 0xF58; // int32
                pub const m_flWarpTime: usize = 0xF5C; // float32
                pub const m_flWarpStartTime: usize = 0xF60; // float32
                pub const m_flPrevPosScale: usize = 0xF64; // float32
                pub const m_bInvertWarp: usize = 0xF68; // bool
                pub const m_bUseCount: usize = 0xF69; // bool
            };
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
            pub const C_OP_SetControlPointFieldToScalarExpression = struct {
                pub const m_nExpression: usize = 0x1E0; // ScalarExpressionType_t
                pub const m_flInput1: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_flInput2: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_flOutputRemap: usize = 0x4C8; // CParticleRemapFloatInput
                pub const m_nOutputCP: usize = 0x638; // int32
                pub const m_nOutVectorField: usize = 0x63C; // int32
                pub const m_flInterpolation: usize = 0x640; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_CreateParticleSystemRenderer = struct {
                pub const m_hEffect: usize = 0x228; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_nEventType: usize = 0x230; // EventTypeSelection_t
                pub const m_vecCPs: usize = 0x238; // CUtlLeanVector<CPAssignment_t>
                pub const m_szParticleConfig: usize = 0x248; // CUtlString
                pub const m_AggregationPos: usize = 0x250; // CPerParticleVecInput
            };
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
            pub const C_INIT_RandomVectorComponent = struct {
                pub const m_flMin: usize = 0x1E0; // float32
                pub const m_flMax: usize = 0x1E4; // float32
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_nComponent: usize = 0x1EC; // int32
            };
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
            pub const C_INIT_SetVectorAttributeToVectorExpression = struct {
                pub const m_nExpression: usize = 0x1E0; // VectorExpressionType_t
                pub const m_vInput1: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x8A0; // CPerParticleVecInput
                pub const m_flLerp: usize = 0xF58; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x10C8; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x10CC; // ParticleSetMethod_t
                pub const m_bNormalizedOutput: usize = 0x10D0; // bool
            };
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
            pub const C_OP_RemapTransformVisibilityToVector = struct {
                pub const m_nSetMethod: usize = 0x1D8; // ParticleSetMethod_t
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x248; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x24C; // float32
                pub const m_flInputMax: usize = 0x250; // float32
                pub const m_vecOutputMin: usize = 0x254; // Vector
                pub const m_vecOutputMax: usize = 0x260; // Vector
                pub const m_flRadius: usize = 0x26C; // float32
            };
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
            pub const C_OP_MovementLoopInsideSphere = struct {
                pub const m_nCP: usize = 0x1D8; // int32
                pub const m_flDistance: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_vecScale: usize = 0x350; // CParticleCollectionVecInput
                pub const m_nDistSqrAttr: usize = 0xA08; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_RenderSimpleModelCollection = struct {
                pub const m_bCenterOffset: usize = 0x228; // bool
                pub const m_hModel: usize = 0x230; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_modelInput: usize = 0x238; // CParticleModelInput
                pub const m_fSizeCullScale: usize = 0x298; // CParticleCollectionFloatInput
                pub const m_bDisableShadows: usize = 0x408; // bool
                pub const m_bDisableMotionBlur: usize = 0x409; // bool
                pub const m_bAcceptsDecals: usize = 0x40A; // bool
                pub const m_fDrawFilter: usize = 0x410; // CPerParticleFloatInput
                pub const m_nAngularVelocityField: usize = 0x580; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_InitFloatCollection = struct {
                pub const m_InputValue: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_nOutputField: usize = 0x350; // ParticleAttributeIndex_t
            };
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
            pub const CPathParameters = struct {
                pub const m_nStartControlPointNumber: usize = 0x0; // int32
                pub const m_nEndControlPointNumber: usize = 0x4; // int32
                pub const m_nBulgeControl: usize = 0x8; // int32
                pub const m_flBulge: usize = 0xC; // float32
                pub const m_flMidPoint: usize = 0x10; // float32
                pub const m_vStartPointOffset: usize = 0x14; // Vector
                pub const m_vMidPointOffset: usize = 0x20; // Vector
                pub const m_vEndOffset: usize = 0x2C; // Vector
            };
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
            pub const C_OP_RemapScalarEndCap = struct {
                pub const m_nFieldInput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E0; // float32
                pub const m_flInputMax: usize = 0x1E4; // float32
                pub const m_flOutputMin: usize = 0x1E8; // float32
                pub const m_flOutputMax: usize = 0x1EC; // float32
            };
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
            pub const C_INIT_CreateFromPlaneCache = struct {
                pub const m_vecOffsetMin: usize = 0x1E0; // Vector
                pub const m_vecOffsetMax: usize = 0x1EC; // Vector
                pub const m_bUseNormal: usize = 0x1F9; // bool
            };
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
            pub const C_OP_LazyCullCompareFloat = struct {
                pub const m_flComparsion1: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_flComparsion2: usize = 0x348; // CPerParticleFloatInput
                pub const m_flCullTime: usize = 0x4B8; // CPerParticleFloatInput
            };
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
            pub const C_OP_ControlPointToRadialScreenSpace = struct {
                pub const m_nCPIn: usize = 0x1E0; // int32
                pub const m_vecCP1Pos: usize = 0x1E4; // Vector
                pub const m_nCPOut: usize = 0x1F0; // int32
                pub const m_nCPOutField: usize = 0x1F4; // int32
                pub const m_nCPSSPosOut: usize = 0x1F8; // int32
            };
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
            pub const C_INIT_NormalOffset = struct {
                pub const m_OffsetMin: usize = 0x1E0; // Vector
                pub const m_OffsetMax: usize = 0x1EC; // Vector
                pub const m_nControlPointNumber: usize = 0x1F8; // int32
                pub const m_bLocalCoords: usize = 0x1FC; // bool
                pub const m_bNormalize: usize = 0x1FD; // bool
            };
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
            pub const C_INIT_CreationNoise = struct {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_bAbsVal: usize = 0x1E4; // bool
                pub const m_bAbsValInv: usize = 0x1E5; // bool
                pub const m_flOffset: usize = 0x1E8; // float32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_flNoiseScale: usize = 0x1F4; // float32
                pub const m_flNoiseScaleLoc: usize = 0x1F8; // float32
                pub const m_vecOffsetLoc: usize = 0x1FC; // Vector
                pub const m_flWorldTimeScale: usize = 0x208; // float32
            };
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
            pub const C_OP_Spin = struct {
            };
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
            pub const C_OP_GameLiquidSpill = struct {
                pub const m_flLiquidContentsField: usize = 0x228; // CParticleCollectionFloatInput
                pub const m_flExpirationTime: usize = 0x398; // CParticleCollectionFloatInput
                pub const m_flRadius: usize = 0x508; // CParticleCollectionFloatInput
                pub const m_bCheckExposedToSky: usize = 0x678; // bool
                pub const m_nAmountAttribute: usize = 0x67C; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_InstantaneousEmitter = struct {
                pub const m_nParticlesToEmit: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_flStartTime: usize = 0x350; // CParticleCollectionFloatInput
                pub const m_flInitFromKilledParentParticles: usize = 0x4C0; // float32
                pub const m_nEventType: usize = 0x4C4; // EventTypeSelection_t
                pub const m_flParentParticleScale: usize = 0x4C8; // CParticleCollectionFloatInput
                pub const m_nMaxEmittedPerFrame: usize = 0x638; // int32
                pub const m_nSnapshotControlPoint: usize = 0x63C; // int32
                pub const m_strSnapshotSubset: usize = 0x640; // CUtlString
            };
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
            pub const C_INIT_LifespanFromVelocity = struct {
                pub const m_vecComponentScale: usize = 0x1E0; // Vector
                pub const m_flTraceOffset: usize = 0x1EC; // float32
                pub const m_flMaxTraceLength: usize = 0x1F0; // float32
                pub const m_flTraceTolerance: usize = 0x1F4; // float32
                pub const m_nMaxPlanes: usize = 0x1F8; // int32
                pub const m_CollisionGroupName: usize = 0x200; // char[128]
                pub const m_nTraceSet: usize = 0x280; // ParticleTraceSet_t
                pub const m_bIncludeWater: usize = 0x290; // bool
            };
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
            pub const C_INIT_VelocityFromCP = struct {
                pub const m_velocityInput: usize = 0x1E0; // CParticleCollectionVecInput
                pub const m_transformInput: usize = 0x898; // CParticleTransformInput
                pub const m_flVelocityScale: usize = 0x900; // float32
                pub const m_bDirectionOnly: usize = 0x904; // bool
            };
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
            pub const C_OP_MovementSkinnedPositionFromCPSnapshot = struct {
                pub const m_nSnapshotControlPointNumber: usize = 0x1D8; // int32
                pub const m_nControlPointNumber: usize = 0x1DC; // int32
                pub const m_bRandom: usize = 0x1E0; // bool
                pub const m_nRandomSeed: usize = 0x1E4; // int32
                pub const m_bSetNormal: usize = 0x1E8; // bool
                pub const m_bSetRadius: usize = 0x1E9; // bool
                pub const m_nIndexType: usize = 0x1EC; // SnapshotIndexType_t
                pub const m_flReadIndex: usize = 0x1F0; // CPerParticleFloatInput
                pub const m_flIncrement: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_nFullLoopIncrement: usize = 0x4D0; // CParticleCollectionFloatInput
                pub const m_nSnapShotStartPoint: usize = 0x640; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0x7B0; // CPerParticleFloatInput
            };
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
            pub const C_OP_OscillateVector = struct {
                pub const m_RateMin: usize = 0x1D8; // Vector
                pub const m_RateMax: usize = 0x1E4; // Vector
                pub const m_FrequencyMin: usize = 0x1F0; // Vector
                pub const m_FrequencyMax: usize = 0x1FC; // Vector
                pub const m_nField: usize = 0x208; // ParticleAttributeIndex_t
                pub const m_bProportional: usize = 0x20C; // bool
                pub const m_bProportionalOp: usize = 0x20D; // bool
                pub const m_bOffset: usize = 0x20E; // bool
                pub const m_flStartTime_min: usize = 0x210; // float32
                pub const m_flStartTime_max: usize = 0x214; // float32
                pub const m_flEndTime_min: usize = 0x218; // float32
                pub const m_flEndTime_max: usize = 0x21C; // float32
                pub const m_flOscMult: usize = 0x220; // CPerParticleFloatInput
                pub const m_flOscAdd: usize = 0x390; // CPerParticleFloatInput
                pub const m_flRateScale: usize = 0x500; // CPerParticleFloatInput
            };
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
            pub const C_OP_PositionLock = struct {
                pub const m_TransformInput: usize = 0x1D8; // CParticleTransformInput
                pub const m_flStartTime_min: usize = 0x240; // float32
                pub const m_flStartTime_max: usize = 0x244; // float32
                pub const m_flStartTime_exp: usize = 0x248; // float32
                pub const m_flEndTime_min: usize = 0x24C; // float32
                pub const m_flEndTime_max: usize = 0x250; // float32
                pub const m_flEndTime_exp: usize = 0x254; // float32
                pub const m_flRange: usize = 0x258; // float32
                pub const m_flRangeBias: usize = 0x260; // CParticleCollectionFloatInput
                pub const m_flJumpThreshold: usize = 0x3D0; // float32
                pub const m_flPrevPosScale: usize = 0x3D4; // float32
                pub const m_bLockRot: usize = 0x3D8; // bool
                pub const m_vecScale: usize = 0x3E0; // CParticleCollectionVecInput
                pub const m_nFieldOutput: usize = 0xA98; // ParticleAttributeIndex_t
                pub const m_nFieldOutputPrev: usize = 0xA9C; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_RenderVRHapticEvent = struct {
                pub const m_nHand: usize = 0x228; // ParticleVRHandChoiceList_t
                pub const m_nOutputHandCP: usize = 0x22C; // int32
                pub const m_nOutputField: usize = 0x230; // int32
                pub const m_flAmplitude: usize = 0x238; // CPerParticleFloatInput
            };
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
            pub const C_OP_SetControlPointToImpactPoint = struct {
                pub const m_nCPOut: usize = 0x1E0; // int32
                pub const m_nCPIn: usize = 0x1E4; // int32
                pub const m_flUpdateRate: usize = 0x1E8; // float32
                pub const m_flTraceLength: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_flStartOffset: usize = 0x360; // float32
                pub const m_flOffset: usize = 0x364; // float32
                pub const m_vecTraceDir: usize = 0x368; // Vector
                pub const m_CollisionGroupName: usize = 0x374; // char[128]
                pub const m_nTraceSet: usize = 0x3F4; // ParticleTraceSet_t
                pub const m_bSetToEndpoint: usize = 0x3F8; // bool
                pub const m_bTraceToClosestSurface: usize = 0x3F9; // bool
                pub const m_bIncludeWater: usize = 0x3FA; // bool
            };
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
            pub const C_OP_ReinitializeScalarEndCap = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flOutputMin: usize = 0x1DC; // float32
                pub const m_flOutputMax: usize = 0x1E0; // float32
            };
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
            pub const C_OP_TurbulenceForce = struct {
                pub const m_flNoiseCoordScale0: usize = 0x1E8; // float32
                pub const m_flNoiseCoordScale1: usize = 0x1EC; // float32
                pub const m_flNoiseCoordScale2: usize = 0x1F0; // float32
                pub const m_flNoiseCoordScale3: usize = 0x1F4; // float32
                pub const m_vecNoiseAmount0: usize = 0x1F8; // Vector
                pub const m_vecNoiseAmount1: usize = 0x204; // Vector
                pub const m_vecNoiseAmount2: usize = 0x210; // Vector
                pub const m_vecNoiseAmount3: usize = 0x21C; // Vector
            };
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
            pub const C_OP_RemapNamedModelElementOnceTimed = struct {
                pub const m_hModel: usize = 0x1D8; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_inNames: usize = 0x1E0; // CUtlVector<CUtlString>
                pub const m_outNames: usize = 0x1F8; // CUtlVector<CUtlString>
                pub const m_fallbackNames: usize = 0x210; // CUtlVector<CUtlString>
                pub const m_bModelFromRenderer: usize = 0x228; // bool
                pub const m_bProportional: usize = 0x229; // bool
                pub const m_nFieldInput: usize = 0x22C; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x230; // ParticleAttributeIndex_t
                pub const m_flRemapTime: usize = 0x234; // float32
            };
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
            pub const C_OP_SetControlPointToPlayer = struct {
                pub const m_nCP1: usize = 0x1E0; // int32
                pub const m_vecCP1Pos: usize = 0x1E4; // Vector
                pub const m_bOrientToEyes: usize = 0x1F0; // bool
                pub const m_nPosition: usize = 0x1F4; // ParticleEntityPos_t
                pub const m_nRadiusCP: usize = 0x1F8; // int32
                pub const m_nRadiusCPField: usize = 0x1FC; // int32
            };
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
            pub const C_OP_EndCapTimedFreeze = struct {
                pub const m_flFreezeTime: usize = 0x1D8; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_SetRandomControlPointPosition = struct {
                pub const m_bUseWorldLocation: usize = 0x1E0; // bool
                pub const m_bOrient: usize = 0x1E1; // bool
                pub const m_nCP1: usize = 0x1E4; // int32
                pub const m_nHeadLocation: usize = 0x1E8; // int32
                pub const m_flReRandomRate: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_vecCPMinPos: usize = 0x360; // Vector
                pub const m_vecCPMaxPos: usize = 0x36C; // Vector
                pub const m_flInterpolation: usize = 0x378; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_RenderVolumetricEmitter = struct {
                pub const m_strChannelType: usize = 0x228; // CUtlString
                pub const m_nType: usize = 0x230; // ParticleVolumetricSmokeType_t
                pub const m_nCreationType: usize = 0x234; // ParticleVolumetricSmokeCreationType_t
                pub const m_nEventType: usize = 0x238; // EventTypeSelection_t
                pub const m_vecPos: usize = 0x240; // CPerParticleVecInput
                pub const m_vecVelocity: usize = 0x8F8; // CPerParticleVecInput
                pub const m_vPrevPosition: usize = 0xFB0; // CPerParticleVecInput
                pub const m_flSpeed: usize = 0x1668; // CPerParticleFloatInput
                pub const m_flRadius: usize = 0x17D8; // CPerParticleFloatInput
                pub const m_flDensity: usize = 0x1948; // CPerParticleFloatInput
                pub const m_flTemperature: usize = 0x1AB8; // CPerParticleFloatInput
                pub const m_flMagnitude: usize = 0x1C28; // CPerParticleFloatInput
                pub const m_flKillRadius: usize = 0x1D98; // CPerParticleFloatInput
                pub const m_flKillDensityScale: usize = 0x1F08; // CPerParticleFloatInput
                pub const m_flFalloff: usize = 0x2078; // CPerParticleFloatInput
            };
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
            pub const C_OP_RemapTransformVisibilityToScalar = struct {
                pub const m_nSetMethod: usize = 0x1D8; // ParticleSetMethod_t
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x248; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x24C; // float32
                pub const m_flInputMax: usize = 0x250; // float32
                pub const m_flOutputMin: usize = 0x254; // float32
                pub const m_flOutputMax: usize = 0x258; // float32
                pub const m_flRadius: usize = 0x25C; // float32
            };
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
            pub const C_OP_RemapControlPointDirectionToVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1DC; // float32
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
            };
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
            pub const C_OP_ScreenSpacePositionOfTarget = struct {
                pub const m_vecTargetPosition: usize = 0x1D8; // CPerParticleVecInput
                pub const m_bOututBehindness: usize = 0x890; // bool
                pub const m_nBehindFieldOutput: usize = 0x894; // ParticleAttributeIndex_t
                pub const m_flBehindOutputRemap: usize = 0x898; // CParticleRemapFloatInput
                pub const m_nBehindSetMethod: usize = 0xA08; // ParticleSetMethod_t
            };
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
            pub const C_OP_DragRelativeToPlane = struct {
                pub const m_flDragAtPlane: usize = 0x1D8; // CParticleCollectionFloatInput
                pub const m_flFalloff: usize = 0x348; // CParticleCollectionFloatInput
                pub const m_bDirectional: usize = 0x4B8; // bool
                pub const m_vecPlaneNormal: usize = 0x4C0; // CParticleCollectionVecInput
                pub const m_nControlPointNumber: usize = 0xB78; // int32
            };
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
            pub const C_OP_SetCPtoVector = struct {
                pub const m_nCPInput: usize = 0x1D8; // int32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_RandomYaw = struct {
            };
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
            pub const C_OP_SnapshotRigidSkinToBones = struct {
                pub const m_bTransformNormals: usize = 0x1D8; // bool
                pub const m_bTransformRadii: usize = 0x1D9; // bool
                pub const m_nControlPointNumber: usize = 0x1DC; // int32
            };
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
            pub const C_OP_SetSingleControlPointPosition = struct {
                pub const m_bSetOnce: usize = 0x1E0; // bool
                pub const m_nCP1: usize = 0x1E4; // int32
                pub const m_vecCP1Pos: usize = 0x1E8; // CParticleCollectionVecInput
                pub const m_transformInput: usize = 0x8A0; // CParticleTransformInput
            };
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
            pub const C_INIT_DistanceToNeighborCull = struct {
                pub const m_flDistance: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_bIncludeRadii: usize = 0x350; // bool
                pub const m_flLifespanOverlap: usize = 0x358; // CPerParticleFloatInput
                pub const m_nFieldModify: usize = 0x4C8; // ParticleAttributeIndex_t
                pub const m_flModify: usize = 0x4D0; // CPerParticleFloatInput
                pub const m_nSetMethod: usize = 0x640; // ParticleSetMethod_t
                pub const m_bUseNeighbor: usize = 0x644; // bool
            };
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
            pub const C_OP_RemapCPtoScalar = struct {
                pub const m_nCPInput: usize = 0x1D8; // int32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nField: usize = 0x1E0; // int32
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_flStartTime: usize = 0x1F4; // float32
                pub const m_flEndTime: usize = 0x1F8; // float32
                pub const m_flInterpRate: usize = 0x1FC; // float32
                pub const m_nSetMethod: usize = 0x200; // ParticleSetMethod_t
            };
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
            pub const CParticleSystemDefinition = struct {
                pub const m_nBehaviorVersion: usize = 0x8; // int32
                pub const m_PreEmissionOperators: usize = 0x10; // CUtlVector<CParticleFunctionPreEmission*>
                pub const m_Emitters: usize = 0x28; // CUtlVector<CParticleFunctionEmitter*>
                pub const m_Initializers: usize = 0x40; // CUtlVector<CParticleFunctionInitializer*>
                pub const m_Operators: usize = 0x58; // CUtlVector<CParticleFunctionOperator*>
                pub const m_ForceGenerators: usize = 0x70; // CUtlVector<CParticleFunctionForce*>
                pub const m_Constraints: usize = 0x88; // CUtlVector<CParticleFunctionConstraint*>
                pub const m_Renderers: usize = 0xA0; // CUtlVector<CParticleFunctionRenderer*>
                pub const m_Children: usize = 0xB8; // CUtlVector<ParticleChildrenInfo_t>
                pub const m_nFirstMultipleOverride_BackwardCompat: usize = 0x178; // int32
                pub const m_nInitialParticles: usize = 0x258; // int32
                pub const m_nMaxParticles: usize = 0x25C; // int32
                pub const m_nGroupID: usize = 0x260; // int32
                pub const m_BoundingBoxMin: usize = 0x264; // Vector
                pub const m_BoundingBoxMax: usize = 0x270; // Vector
                pub const m_flDepthSortBias: usize = 0x27C; // float32
                pub const m_nSortOverridePositionCP: usize = 0x280; // int32
                pub const m_bInfiniteBounds: usize = 0x284; // bool
                pub const m_bEnableNamedValues: usize = 0x285; // bool
                pub const m_NamedValueDomain: usize = 0x288; // CUtlString
                pub const m_NamedValueLocals: usize = 0x290; // CUtlVector<ParticleNamedValueSource_t*>
                pub const m_ConstantColor: usize = 0x2A8; // Color
                pub const m_ConstantNormal: usize = 0x2AC; // Vector
                pub const m_flConstantRadius: usize = 0x2B8; // float32
                pub const m_flConstantRotation: usize = 0x2BC; // float32
                pub const m_flConstantRotationSpeed: usize = 0x2C0; // float32
                pub const m_flConstantLifespan: usize = 0x2C4; // float32
                pub const m_nConstantSequenceNumber: usize = 0x2C8; // int32
                pub const m_nConstantSequenceNumber1: usize = 0x2CC; // int32
                pub const m_nSnapshotControlPoint: usize = 0x2D0; // int32
                pub const m_hSnapshot: usize = 0x2D8; // CStrongHandle<InfoForResourceTypeIParticleSnapshot>
                pub const m_pszCullReplacementName: usize = 0x2E0; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_flCullRadius: usize = 0x2E8; // float32
                pub const m_flCullFillCost: usize = 0x2EC; // float32
                pub const m_nCullControlPoint: usize = 0x2F0; // int32
                pub const m_hFallback: usize = 0x2F8; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_nFallbackMaxCount: usize = 0x300; // int32
                pub const m_hLowViolenceDef: usize = 0x308; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_hReferenceReplacement: usize = 0x310; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_flPreSimulationTime: usize = 0x318; // float32
                pub const m_flStopSimulationAfterTime: usize = 0x31C; // float32
                pub const m_flMaximumTimeStep: usize = 0x320; // float32
                pub const m_flMaximumSimTime: usize = 0x324; // float32
                pub const m_flMinimumSimTime: usize = 0x328; // float32
                pub const m_flMinimumTimeStep: usize = 0x32C; // float32
                pub const m_nMinimumFrames: usize = 0x330; // int32
                pub const m_bIsGPUParticleSystem: usize = 0x334; // bool
                pub const m_nMinCPULevel: usize = 0x338; // int32
                pub const m_nMinGPULevel: usize = 0x33C; // int32
                pub const m_flNoDrawTimeToGoToSleep: usize = 0x340; // float32
                pub const m_flMaxDrawDistance: usize = 0x344; // float32
                pub const m_flStartFadeDistance: usize = 0x348; // float32
                pub const m_flMaxCreationDistance: usize = 0x34C; // float32
                pub const m_nAggregationMinAvailableParticles: usize = 0x350; // int32
                pub const m_flAggregateRadius: usize = 0x354; // float32
                pub const m_bShouldBatch: usize = 0x358; // bool
                pub const m_bShouldHitboxesFallbackToRenderBounds: usize = 0x359; // bool
                pub const m_bShouldHitboxesFallbackToSnapshot: usize = 0x35A; // bool
                pub const m_bShouldHitboxesFallbackToCollisionHulls: usize = 0x35B; // bool
                pub const m_nViewModelEffect: usize = 0x35C; // InheritableBoolType_t
                pub const m_bScreenSpaceEffect: usize = 0x360; // bool
                pub const m_pszTargetLayerID: usize = 0x368; // CUtlSymbolLarge
                pub const m_nSkipRenderControlPoint: usize = 0x370; // int32
                pub const m_nAllowRenderControlPoint: usize = 0x374; // int32
                pub const m_bShouldSort: usize = 0x378; // bool
                pub const m_controlPointConfigurations: usize = 0x3C0; // CUtlVector<ParticleControlPointConfiguration_t>
            };
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
            pub const C_OP_RemapNamedModelMeshGroupEndCap = struct {
            };
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
            pub const C_OP_PercentageBetweenTransformsVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1DC; // float32
                pub const m_flInputMax: usize = 0x1E0; // float32
                pub const m_vecOutputMin: usize = 0x1E4; // Vector
                pub const m_vecOutputMax: usize = 0x1F0; // Vector
                pub const m_TransformStart: usize = 0x200; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x268; // CParticleTransformInput
                pub const m_nSetMethod: usize = 0x2D0; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x2D4; // bool
                pub const m_bRadialCheck: usize = 0x2D5; // bool
            };
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
            pub const C_INIT_CreateWithinBox = struct {
                pub const m_vecMin: usize = 0x1E0; // CPerParticleVecInput
                pub const m_vecMax: usize = 0x898; // CPerParticleVecInput
                pub const m_nControlPointNumber: usize = 0xF50; // int32
                pub const m_bLocalSpace: usize = 0xF54; // bool
                pub const m_randomnessParameters: usize = 0xF58; // CRandomNumberGeneratorParameters
                pub const m_bUseNewCode: usize = 0xF60; // bool
            };
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
            pub const C_OP_ChooseRandomChildrenInGroup = struct {
                pub const m_nChildGroupID: usize = 0x1E0; // int32
                pub const m_flNumberOfChildren: usize = 0x1E8; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_ControlpointLight = struct {
                pub const m_flScale: usize = 0x1D8; // float32
                pub const m_nControlPoint1: usize = 0x660; // int32
                pub const m_nControlPoint2: usize = 0x664; // int32
                pub const m_nControlPoint3: usize = 0x668; // int32
                pub const m_nControlPoint4: usize = 0x66C; // int32
                pub const m_vecCPOffset1: usize = 0x670; // Vector
                pub const m_vecCPOffset2: usize = 0x67C; // Vector
                pub const m_vecCPOffset3: usize = 0x688; // Vector
                pub const m_vecCPOffset4: usize = 0x694; // Vector
                pub const m_LightFiftyDist1: usize = 0x6A0; // float32
                pub const m_LightZeroDist1: usize = 0x6A4; // float32
                pub const m_LightFiftyDist2: usize = 0x6A8; // float32
                pub const m_LightZeroDist2: usize = 0x6AC; // float32
                pub const m_LightFiftyDist3: usize = 0x6B0; // float32
                pub const m_LightZeroDist3: usize = 0x6B4; // float32
                pub const m_LightFiftyDist4: usize = 0x6B8; // float32
                pub const m_LightZeroDist4: usize = 0x6BC; // float32
                pub const m_LightColor1: usize = 0x6C0; // Color
                pub const m_LightColor2: usize = 0x6C4; // Color
                pub const m_LightColor3: usize = 0x6C8; // Color
                pub const m_LightColor4: usize = 0x6CC; // Color
                pub const m_bLightType1: usize = 0x6D0; // bool
                pub const m_bLightType2: usize = 0x6D1; // bool
                pub const m_bLightType3: usize = 0x6D2; // bool
                pub const m_bLightType4: usize = 0x6D3; // bool
                pub const m_bLightDynamic1: usize = 0x6D4; // bool
                pub const m_bLightDynamic2: usize = 0x6D5; // bool
                pub const m_bLightDynamic3: usize = 0x6D6; // bool
                pub const m_bLightDynamic4: usize = 0x6D7; // bool
                pub const m_bUseNormal: usize = 0x6D8; // bool
                pub const m_bUseHLambert: usize = 0x6D9; // bool
                pub const m_bClampLowerRange: usize = 0x6DE; // bool
                pub const m_bClampUpperRange: usize = 0x6DF; // bool
            };
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
            pub const C_OP_VectorFieldSnapshot = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_nAttributeToWrite: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x1E0; // int32
                pub const m_flInterpolation: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_vecScale: usize = 0x358; // CPerParticleVecInput
                pub const m_flBoundaryDampening: usize = 0xA10; // float32
                pub const m_bSetVelocity: usize = 0xA14; // bool
                pub const m_bLockToSurface: usize = 0xA15; // bool
                pub const m_flGridSpacing: usize = 0xA18; // float32
            };
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
            pub const C_OP_CylindricalDistanceToTransform = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x350; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4C0; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x630; // CPerParticleFloatInput
                pub const m_TransformStart: usize = 0x7A0; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x808; // CParticleTransformInput
                pub const m_nSetMethod: usize = 0x870; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x874; // bool
                pub const m_bAdditive: usize = 0x875; // bool
                pub const m_bCapsule: usize = 0x876; // bool
            };
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
            pub const C_INIT_RandomScalar = struct {
                pub const m_flMin: usize = 0x1E0; // float32
                pub const m_flMax: usize = 0x1E4; // float32
                pub const m_flExponent: usize = 0x1E8; // float32
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_RenderPostProcessing = struct {
                pub const m_flPostProcessStrength: usize = 0x228; // CPerParticleFloatInput
                pub const m_hPostTexture: usize = 0x398; // CStrongHandle<InfoForResourceTypeCPostProcessingResource>
                pub const m_nPriority: usize = 0x3A0; // ParticlePostProcessPriorityGroup_t
            };
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
            pub const C_OP_OscillateScalar = struct {
                pub const m_RateMin: usize = 0x1D8; // float32
                pub const m_RateMax: usize = 0x1DC; // float32
                pub const m_FrequencyMin: usize = 0x1E0; // float32
                pub const m_FrequencyMax: usize = 0x1E4; // float32
                pub const m_nField: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_bProportional: usize = 0x1EC; // bool
                pub const m_bProportionalOp: usize = 0x1ED; // bool
                pub const m_flStartTime_min: usize = 0x1F0; // float32
                pub const m_flStartTime_max: usize = 0x1F4; // float32
                pub const m_flEndTime_min: usize = 0x1F8; // float32
                pub const m_flEndTime_max: usize = 0x1FC; // float32
                pub const m_flOscMult: usize = 0x200; // float32
                pub const m_flOscAdd: usize = 0x204; // float32
            };
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
            pub const C_OP_FadeOut = struct {
                pub const m_flFadeOutTimeMin: usize = 0x1D8; // float32
                pub const m_flFadeOutTimeMax: usize = 0x1DC; // float32
                pub const m_flFadeOutTimeExp: usize = 0x1E0; // float32
                pub const m_flFadeBias: usize = 0x1E4; // float32
                pub const m_bProportional: usize = 0x220; // bool
                pub const m_bEaseInAndOut: usize = 0x221; // bool
            };
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
            pub const C_OP_WaterImpulseRenderer = struct {
                pub const m_vecPos: usize = 0x228; // CPerParticleVecInput
                pub const m_flRadius: usize = 0x8E0; // CPerParticleFloatInput
                pub const m_flMagnitude: usize = 0xA50; // CPerParticleFloatInput
                pub const m_flShape: usize = 0xBC0; // CPerParticleFloatInput
                pub const m_flWindSpeed: usize = 0xD30; // CPerParticleFloatInput
                pub const m_flWobble: usize = 0xEA0; // CPerParticleFloatInput
                pub const m_bIsRadialWind: usize = 0x1010; // bool
                pub const m_nEventType: usize = 0x1014; // EventTypeSelection_t
            };
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
            pub const C_INIT_RandomSequence = struct {
                pub const m_nSequenceMin: usize = 0x1E0; // int32
                pub const m_nSequenceMax: usize = 0x1E4; // int32
                pub const m_bShuffle: usize = 0x1E8; // bool
                pub const m_bLinear: usize = 0x1E9; // bool
                pub const m_WeightedList: usize = 0x1F0; // CUtlVector<SequenceWeightedList_t>
            };
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
            pub const C_OP_RampScalarSplineSimple = struct {
                pub const m_Rate: usize = 0x1D8; // float32
                pub const m_flStartTime: usize = 0x1DC; // float32
                pub const m_flEndTime: usize = 0x1E0; // float32
                pub const m_nField: usize = 0x210; // ParticleAttributeIndex_t
                pub const m_bEaseOut: usize = 0x214; // bool
            };
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
            pub const C_INIT_DistanceCull = struct {
                pub const m_nControlPoint: usize = 0x1E0; // int32
                pub const m_flDistance: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_bCullInside: usize = 0x358; // bool
            };
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
            pub const C_INIT_InitFromVectorFieldSnapshot = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nLocalSpaceCP: usize = 0x1E4; // int32
                pub const m_nWeightUpdateCP: usize = 0x1E8; // int32
                pub const m_bUseVerticalVelocity: usize = 0x1EC; // bool
                pub const m_vecScale: usize = 0x1F0; // CPerParticleVecInput
            };
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
            pub const C_OP_SetVectorAttributeToVectorExpression = struct {
                pub const m_nExpression: usize = 0x1D8; // VectorExpressionType_t
                pub const m_vInput1: usize = 0x1E0; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x898; // CPerParticleVecInput
                pub const m_flLerp: usize = 0xF50; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x10C0; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x10C4; // ParticleSetMethod_t
                pub const m_bNormalizedOutput: usize = 0x10C8; // bool
            };
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
            pub const C_INIT_AddVectorToVector = struct {
                pub const m_vecScale: usize = 0x1E0; // Vector
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_nFieldInput: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_vOffsetMin: usize = 0x1F4; // Vector
                pub const m_vOffsetMax: usize = 0x200; // Vector
                pub const m_randomnessParameters: usize = 0x20C; // CRandomNumberGeneratorParameters
            };
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
            pub const C_INIT_RemapInitialVisibilityScalar = struct {
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // float32
                pub const m_flInputMax: usize = 0x1EC; // float32
                pub const m_flOutputMin: usize = 0x1F0; // float32
                pub const m_flOutputMax: usize = 0x1F4; // float32
            };
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
            pub const C_OP_RemapTransformOrientationToYaw = struct {
                pub const m_TransformInput: usize = 0x1D8; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x240; // ParticleAttributeIndex_t
                pub const m_flRotOffset: usize = 0x244; // float32
                pub const m_flSpinStrength: usize = 0x248; // float32
            };
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
            pub const C_OP_RenderStatusEffect = struct {
                pub const m_pTextureColorWarp: usize = 0x228; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureDetail2: usize = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureDiffuseWarp: usize = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureFresnelColorWarp: usize = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureFresnelWarp: usize = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureSpecularWarp: usize = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureEnvMap: usize = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
            };
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
            pub const C_OP_RandomForce = struct {
                pub const m_MinForce: usize = 0x1E8; // Vector
                pub const m_MaxForce: usize = 0x1F4; // Vector
            };
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
            pub const C_OP_RemapParticleCountOnScalarEndCap = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nInputMin: usize = 0x1DC; // int32
                pub const m_nInputMax: usize = 0x1E0; // int32
                pub const m_flOutputMin: usize = 0x1E4; // float32
                pub const m_flOutputMax: usize = 0x1E8; // float32
                pub const m_bBackwards: usize = 0x1EC; // bool
                pub const m_nSetMethod: usize = 0x1F0; // ParticleSetMethod_t
            };
            // Parent: None
            // Field count: 18
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // INHERITABLE_BOOL_FALSE
            // INHERITABLE_BOOL_TRUE
            pub const ParticlePreviewState_t = struct {
                pub const m_previewModel: usize = 0x0; // CUtlString
                pub const m_nModSpecificData: usize = 0x8; // uint32
                pub const m_groundType: usize = 0xC; // PetGroundType_t
                pub const m_sequenceName: usize = 0x10; // CUtlString
                pub const m_nFireParticleOnSequenceFrame: usize = 0x18; // int32
                pub const m_hitboxSetName: usize = 0x20; // CUtlString
                pub const m_materialGroupName: usize = 0x28; // CUtlString
                pub const m_vecBodyGroups: usize = 0x30; // CUtlVector<ParticlePreviewBodyGroup_t>
                pub const m_flPlaybackSpeed: usize = 0x48; // float32
                pub const m_flParticleSimulationRate: usize = 0x4C; // float32
                pub const m_bShouldDrawHitboxes: usize = 0x50; // bool
                pub const m_bShouldDrawAttachments: usize = 0x51; // bool
                pub const m_bShouldDrawAttachmentNames: usize = 0x52; // bool
                pub const m_bShouldDrawControlPointAxes: usize = 0x53; // bool
                pub const m_bAnimationNonLooping: usize = 0x54; // bool
                pub const m_bSequenceNameIsAnimClipPath: usize = 0x55; // bool
                pub const m_vecPreviewGravity: usize = 0x58; // Vector
                pub const m_vecPreviewWind: usize = 0x64; // Vector
            };
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
            pub const C_OP_ModelCull = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_bBoundBox: usize = 0x1DC; // bool
                pub const m_bCullOutside: usize = 0x1DD; // bool
                pub const m_bUseBones: usize = 0x1DE; // bool
                pub const m_HitboxSetName: usize = 0x1DF; // char[128]
            };
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
            pub const C_OP_SetFloat = struct {
                pub const m_InputValue: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x348; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x34C; // ParticleSetMethod_t
                pub const m_Lerp: usize = 0x350; // CPerParticleFloatInput
            };
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
            pub const C_INIT_RemapTransformToVector = struct {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vInputMin: usize = 0x1E4; // Vector
                pub const m_vInputMax: usize = 0x1F0; // Vector
                pub const m_vOutputMin: usize = 0x1FC; // Vector
                pub const m_vOutputMax: usize = 0x208; // Vector
                pub const m_TransformInput: usize = 0x218; // CParticleTransformInput
                pub const m_LocalSpaceTransform: usize = 0x280; // CParticleTransformInput
                pub const m_flStartTime: usize = 0x2E8; // float32
                pub const m_flEndTime: usize = 0x2EC; // float32
                pub const m_nSetMethod: usize = 0x2F0; // ParticleSetMethod_t
                pub const m_bOffset: usize = 0x2F4; // bool
                pub const m_bAccelerate: usize = 0x2F5; // bool
                pub const m_flRemapBias: usize = 0x2F8; // float32
            };
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
            pub const C_OP_ScreenSpaceDistanceToEdge = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flMaxDistFromEdge: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flOutputRemap: usize = 0x350; // CParticleRemapFloatInput
                pub const m_nSetMethod: usize = 0x4C0; // ParticleSetMethod_t
            };
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
            pub const C_OP_RemapVectortoCP = struct {
                pub const m_nOutControlPointNumber: usize = 0x1D8; // int32
                pub const m_nFieldInput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nParticleNumber: usize = 0x1E0; // int32
            };
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
            pub const C_OP_SetFromCPSnapshot = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_strSnapshotSubset: usize = 0x1E0; // CUtlString
                pub const m_nAttributeToRead: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_nAttributeToWrite: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x1F0; // int32
                pub const m_bRandom: usize = 0x1F4; // bool
                pub const m_bReverse: usize = 0x1F5; // bool
                pub const m_nRandomSeed: usize = 0x1F8; // int32
                pub const m_nSnapShotStartPoint: usize = 0x200; // CParticleCollectionFloatInput
                pub const m_nSnapShotIncrement: usize = 0x370; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0x4E0; // CPerParticleFloatInput
                pub const m_bSubSample: usize = 0x650; // bool
                pub const m_bPrev: usize = 0x651; // bool
            };
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
            pub const C_OP_DistanceBetweenCPsToCP = struct {
                pub const m_nStartCP: usize = 0x1E0; // int32
                pub const m_nEndCP: usize = 0x1E4; // int32
                pub const m_nOutputCP: usize = 0x1E8; // int32
                pub const m_nOutputCPField: usize = 0x1EC; // int32
                pub const m_bSetOnce: usize = 0x1F0; // bool
                pub const m_flInputMin: usize = 0x1F4; // float32
                pub const m_flInputMax: usize = 0x1F8; // float32
                pub const m_flOutputMin: usize = 0x1FC; // float32
                pub const m_flOutputMax: usize = 0x200; // float32
                pub const m_flMaxTraceLength: usize = 0x204; // float32
                pub const m_flLOSScale: usize = 0x208; // float32
                pub const m_bLOS: usize = 0x20C; // bool
                pub const m_CollisionGroupName: usize = 0x20D; // char[128]
                pub const m_nTraceSet: usize = 0x290; // ParticleTraceSet_t
                pub const m_nSetParent: usize = 0x294; // ParticleParentSetMode_t
            };
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
            pub const C_OP_SetControlPointToHand = struct {
                pub const m_nCP1: usize = 0x1E0; // int32
                pub const m_nHand: usize = 0x1E4; // int32
                pub const m_vecCP1Pos: usize = 0x1E8; // Vector
                pub const m_bOrientToHand: usize = 0x1F4; // bool
            };
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
            pub const C_OP_DistanceCull = struct {
                pub const m_nControlPoint: usize = 0x1D8; // int32
                pub const m_vecPointOffset: usize = 0x1DC; // Vector
                pub const m_flDistance: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_bCullInside: usize = 0x358; // bool
                pub const m_nAttribute: usize = 0x35C; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_CreateAlongPath = struct {
                pub const m_fMaxDistance: usize = 0x1E0; // float32
                pub const m_PathParams: usize = 0x1F0; // CPathParameters
                pub const m_bUseRandomCPs: usize = 0x230; // bool
                pub const m_vEndOffset: usize = 0x234; // Vector
                pub const m_bSaveOffset: usize = 0x240; // bool
            };
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
            pub const C_OP_SetControlPointsToModelParticles = struct {
                pub const m_HitboxSetName: usize = 0x1D8; // char[128]
                pub const m_AttachmentName: usize = 0x258; // char[128]
                pub const m_nFirstControlPoint: usize = 0x2D8; // int32
                pub const m_nNumControlPoints: usize = 0x2DC; // int32
                pub const m_nFirstSourcePoint: usize = 0x2E0; // int32
                pub const m_bSkin: usize = 0x2E4; // bool
                pub const m_bAttachment: usize = 0x2E5; // bool
            };
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
            pub const C_OP_ColorInterpolateRandom = struct {
                pub const m_ColorFadeMin: usize = 0x1D8; // Color
                pub const m_ColorFadeMax: usize = 0x1F4; // Color
                pub const m_flFadeStartTime: usize = 0x204; // float32
                pub const m_flFadeEndTime: usize = 0x208; // float32
                pub const m_nFieldOutput: usize = 0x20C; // ParticleAttributeIndex_t
                pub const m_bEaseInOut: usize = 0x210; // bool
            };
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
            pub const C_INIT_RemapNamedModelSequenceToScalar = struct {
            };
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
            pub const C_OP_RenderLights = struct {
                pub const m_flAnimationRate: usize = 0x230; // float32
                pub const m_nAnimationType: usize = 0x234; // AnimationType_t
                pub const m_bAnimateInFPS: usize = 0x238; // bool
                pub const m_flMinSize: usize = 0x23C; // float32
                pub const m_flMaxSize: usize = 0x240; // float32
                pub const m_flStartFadeSize: usize = 0x244; // float32
                pub const m_flEndFadeSize: usize = 0x248; // float32
            };
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
            pub const C_OP_DecayClampCount = struct {
                pub const m_nCount: usize = 0x1D8; // CParticleCollectionFloatInput
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyAttributeChoiceName
            // MPropertyFriendlyName
            pub const CRandomNumberGeneratorParameters = struct {
                pub const m_bDistributeEvenly: usize = 0x0; // bool
                pub const m_nSeed: usize = 0x4; // int32
            };
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
            pub const C_INIT_ColorLitPerParticle = struct {
                pub const m_ColorMin: usize = 0x1F8; // Color
                pub const m_ColorMax: usize = 0x1FC; // Color
                pub const m_TintMin: usize = 0x200; // Color
                pub const m_TintMax: usize = 0x204; // Color
                pub const m_flTintPerc: usize = 0x208; // float32
                pub const m_nTintBlendMode: usize = 0x20C; // ParticleColorBlendMode_t
                pub const m_flLightAmplification: usize = 0x210; // float32
            };
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
            pub const C_INIT_SetAttributeToScalarExpression = struct {
                pub const m_nExpression: usize = 0x1E0; // ScalarExpressionType_t
                pub const m_flInput1: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flInput2: usize = 0x358; // CPerParticleFloatInput
                pub const m_flOutputRemap: usize = 0x4C8; // CParticleRemapFloatInput
                pub const m_nOutputField: usize = 0x638; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x63C; // ParticleSetMethod_t
            };
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
            pub const C_INIT_CreateOnGrid = struct {
                pub const m_nXCount: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_nYCount: usize = 0x350; // CParticleCollectionFloatInput
                pub const m_nZCount: usize = 0x4C0; // CParticleCollectionFloatInput
                pub const m_nXSpacing: usize = 0x630; // CParticleCollectionFloatInput
                pub const m_nYSpacing: usize = 0x7A0; // CParticleCollectionFloatInput
                pub const m_nZSpacing: usize = 0x910; // CParticleCollectionFloatInput
                pub const m_nControlPointNumber: usize = 0xA80; // int32
                pub const m_bLocalSpace: usize = 0xA84; // bool
                pub const m_bCenter: usize = 0xA85; // bool
                pub const m_bHollow: usize = 0xA86; // bool
            };
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
            pub const C_OP_RampCPLinearRandom = struct {
                pub const m_nOutControlPointNumber: usize = 0x1E0; // int32
                pub const m_vecRateMin: usize = 0x1E4; // Vector
                pub const m_vecRateMax: usize = 0x1F0; // Vector
            };
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
            pub const C_INIT_RandomAlphaWindowThreshold = struct {
                pub const m_flMin: usize = 0x1E0; // float32
                pub const m_flMax: usize = 0x1E4; // float32
                pub const m_flExponent: usize = 0x1E8; // float32
            };
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
            pub const C_INIT_CreateOnModelAtHeight = struct {
                pub const m_bUseBones: usize = 0x1E0; // bool
                pub const m_bForceZ: usize = 0x1E1; // bool
                pub const m_nControlPointNumber: usize = 0x1E4; // int32
                pub const m_nHeightCP: usize = 0x1E8; // int32
                pub const m_bUseWaterHeight: usize = 0x1EC; // bool
                pub const m_flDesiredHeight: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_vecHitBoxScale: usize = 0x360; // CParticleCollectionVecInput
                pub const m_vecDirectionBias: usize = 0xA18; // CParticleCollectionVecInput
                pub const m_nBiasType: usize = 0x10D0; // ParticleHitboxBiasType_t
                pub const m_bLocalCoords: usize = 0x10D4; // bool
                pub const m_bPreferMovingBoxes: usize = 0x10D5; // bool
                pub const m_HitboxSetName: usize = 0x10D6; // char[128]
                pub const m_flHitboxVelocityScale: usize = 0x1158; // CParticleCollectionFloatInput
                pub const m_flMaxBoneVelocity: usize = 0x12C8; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_ModelSurfaceSnapshotGenerator = struct {
                pub const m_nCPSnapshot: usize = 0x1E0; // int32
                pub const m_modelInput: usize = 0x1E8; // CParticleModelInput
                pub const m_flRecalcRate: usize = 0x248; // CParticleCollectionFloatInput
                pub const m_flUSpacing: usize = 0x3B8; // CParticleCollectionFloatInput
                pub const m_flVSpacing: usize = 0x528; // CParticleCollectionFloatInput
                pub const m_flSurfaceOffset: usize = 0x698; // CParticleCollectionFloatInput
                pub const m_bSetNormal: usize = 0x808; // bool
                pub const m_bSetUp: usize = 0x809; // bool
                pub const m_bSetGravity: usize = 0x80A; // bool
                pub const m_bSetUV: usize = 0x80B; // bool
            };
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
            pub const C_OP_RestartAfterDuration = struct {
                pub const m_flDurationMin: usize = 0x1D8; // float32
                pub const m_flDurationMax: usize = 0x1DC; // float32
                pub const m_nCP: usize = 0x1E0; // int32
                pub const m_nCPField: usize = 0x1E4; // int32
                pub const m_nChildGroupID: usize = 0x1E8; // int32
                pub const m_bOnlyChildren: usize = 0x1EC; // bool
            };
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
            pub const C_OP_RemapVisibilityScalar = struct {
                pub const m_nFieldInput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E0; // float32
                pub const m_flInputMax: usize = 0x1E4; // float32
                pub const m_flOutputMin: usize = 0x1E8; // float32
                pub const m_flOutputMax: usize = 0x1EC; // float32
                pub const m_flRadiusScale: usize = 0x1F0; // float32
            };
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
            pub const C_INIT_CreateSequentialPathV2 = struct {
                pub const m_fMaxDistance: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flNumToAssign: usize = 0x350; // CParticleCollectionFloatInput
                pub const m_bLoop: usize = 0x4C0; // bool
                pub const m_bCPPairs: usize = 0x4C1; // bool
                pub const m_bSaveOffset: usize = 0x4C2; // bool
                pub const m_PathParams: usize = 0x4D0; // CPathParameters
            };
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
            pub const VecInputMaterialVariable_t = struct {
                pub const m_strVariable: usize = 0x0; // CUtlString
                pub const m_vecInput: usize = 0x8; // CParticleCollectionVecInput
            };
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
            pub const C_INIT_RemapInitialDirectionToTransformToVector = struct {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x248; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x24C; // float32
                pub const m_flOffsetRot: usize = 0x250; // float32
                pub const m_vecOffsetAxis: usize = 0x254; // Vector
                pub const m_bNormalize: usize = 0x260; // bool
                pub const m_vecTargetPosition: usize = 0x1E0; // CPerParticleVecInput
            };
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
            pub const C_OP_LockToSavedSequentialPathV2 = struct {
                pub const m_flFadeStart: usize = 0x1D8; // float32
                pub const m_flFadeEnd: usize = 0x1DC; // float32
                pub const m_bCPPairs: usize = 0x1E0; // bool
                pub const m_PathParams: usize = 0x1F0; // CPathParameters
            };
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
            pub const C_OP_NormalLock = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
            };
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
            pub const C_INIT_RemapTransformOrientationToRotations = struct {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_vecRotation: usize = 0x248; // Vector
                pub const m_bUseQuat: usize = 0x254; // bool
                pub const m_bWriteNormal: usize = 0x255; // bool
            };
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
            pub const C_OP_Cull = struct {
                pub const m_flCullPerc: usize = 0x1D8; // float32
                pub const m_flCullStart: usize = 0x1DC; // float32
                pub const m_flCullEnd: usize = 0x1E0; // float32
                pub const m_flCullExp: usize = 0x1E4; // float32
            };
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
            pub const C_INIT_RandomYawFlip = struct {
                pub const m_flPercent: usize = 0x1E0; // float32
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            pub const SequenceWeightedList_t = struct {
                pub const m_nSequence: usize = 0x0; // int32
                pub const m_flRelativeWeight: usize = 0x4; // float32
            };
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
            pub const C_OP_ReadFromNeighboringParticle = struct {
                pub const m_nFieldInput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x1E0; // int32
                pub const m_DistanceCheck: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flInterpolation: usize = 0x358; // CPerParticleFloatInput
            };
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
            pub const C_OP_RenderText = struct {
                pub const m_OutlineColor: usize = 0x228; // Color
                pub const m_DefaultText: usize = 0x230; // CUtlString
            };
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
            pub const C_OP_LerpToInitialPosition = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_flInterpolation: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nCacheField: usize = 0x350; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_vecScale: usize = 0x4C8; // CParticleCollectionVecInput
            };
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
            pub const C_INIT_RandomRotation = struct {
            };
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
            pub const C_OP_LerpEndCapVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_vecOutput: usize = 0x1DC; // Vector
                pub const m_flLerpTime: usize = 0x1E8; // float32
            };
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
            pub const C_OP_VelocityDecay = struct {
                pub const m_flMinVelocity: usize = 0x1D8; // float32
            };
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
            pub const C_OP_SetCPOrientationToPointAtCP = struct {
                pub const m_nInputCP: usize = 0x1E0; // int32
                pub const m_nOutputCP: usize = 0x1E4; // int32
                pub const m_flInterpolation: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_b2DOrientation: usize = 0x358; // bool
                pub const m_bAvoidSingularity: usize = 0x359; // bool
                pub const m_bPointAway: usize = 0x35A; // bool
            };
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
            pub const C_OP_LockToPointList = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_pointList: usize = 0x1E0; // CUtlVector<PointDefinition_t>
                pub const m_bPlaceAlongPath: usize = 0x1F8; // bool
                pub const m_bClosedLoop: usize = 0x1F9; // bool
                pub const m_nNumPointsAlongPath: usize = 0x1FC; // int32
            };
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
            pub const C_OP_MovementPlaceOnGround = struct {
                pub const m_flOffset: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_flMaxTraceLength: usize = 0x348; // float32
                pub const m_flTolerance: usize = 0x34C; // float32
                pub const m_vecTraceDir: usize = 0x350; // CPerParticleVecInput
                pub const m_flTraceOffset: usize = 0xA08; // float32
                pub const m_flLerpRate: usize = 0xA0C; // float32
                pub const m_CollisionGroupName: usize = 0xA10; // char[128]
                pub const m_nTraceSet: usize = 0xA90; // ParticleTraceSet_t
                pub const m_nRefCP1: usize = 0xA94; // int32
                pub const m_nRefCP2: usize = 0xA98; // int32
                pub const m_nLerpCP: usize = 0xA9C; // int32
                pub const m_nTraceMissBehavior: usize = 0xAA8; // ParticleTraceMissBehavior_t
                pub const m_bIncludeShotHull: usize = 0xAAC; // bool
                pub const m_bIncludeWater: usize = 0xAAD; // bool
                pub const m_bSetNormal: usize = 0xAB0; // bool
                pub const m_bScaleOffset: usize = 0xAB1; // bool
                pub const m_nPreserveOffsetCP: usize = 0xAB4; // int32
                pub const m_nIgnoreCP: usize = 0xAB8; // int32
            };
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
            pub const C_OP_SetCPOrientationToDirection = struct {
                pub const m_nInputControlPoint: usize = 0x1D8; // int32
                pub const m_nOutputControlPoint: usize = 0x1DC; // int32
            };
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
            pub const C_OP_RemapCrossProductOfTwoVectorsToVector = struct {
                pub const m_InputVec1: usize = 0x1D8; // CPerParticleVecInput
                pub const m_InputVec2: usize = 0x890; // CPerParticleVecInput
                pub const m_nFieldOutput: usize = 0xF48; // ParticleAttributeIndex_t
                pub const m_bNormalize: usize = 0xF4C; // bool
            };
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
            pub const C_OP_RemapTransformOrientationToRotations = struct {
                pub const m_TransformInput: usize = 0x1D8; // CParticleTransformInput
                pub const m_vecRotation: usize = 0x240; // Vector
                pub const m_bUseQuat: usize = 0x24C; // bool
                pub const m_bWriteNormal: usize = 0x24D; // bool
            };
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
            pub const C_INIT_RandomRotationSpeed = struct {
            };
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
            pub const C_OP_InheritFromParentParticlesV2 = struct {
                pub const m_flScale: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_nFieldOutput: usize = 0x348; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x350; // CPerParticleFloatInput
                pub const m_bSubSample: usize = 0x4C0; // bool
                pub const m_bRandomDistribution: usize = 0x4C1; // bool
                pub const m_bReverse: usize = 0x4C2; // bool
                pub const m_nMissingParentBehavior: usize = 0x4C4; // MissingParentInheritBehavior_t
                pub const m_flInterpolation: usize = 0x4C8; // CPerParticleFloatInput
            };
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
            pub const C_INIT_RandomSecondSequence = struct {
                pub const m_nSequenceMin: usize = 0x1E0; // int32
                pub const m_nSequenceMax: usize = 0x1E4; // int32
            };
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
            pub const C_OP_SetFloatCollection = struct {
                pub const m_InputValue: usize = 0x1D8; // CParticleCollectionFloatInput
                pub const m_nOutputField: usize = 0x348; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x34C; // ParticleSetMethod_t
                pub const m_Lerp: usize = 0x350; // CParticleCollectionFloatInput
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            pub const PointDefinition_t = struct {
                pub const m_nControlPoint: usize = 0x0; // int32
                pub const m_bLocalCoords: usize = 0x4; // bool
                pub const m_vOffset: usize = 0x8; // Vector
            };
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
            pub const C_OP_Diffusion = struct {
                pub const m_flRadiusScale: usize = 0x1D8; // float32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nVoxelGridResolution: usize = 0x1E0; // int32
            };
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
            pub const C_INIT_AgeNoise = struct {
                pub const m_bAbsVal: usize = 0x1E0; // bool
                pub const m_bAbsValInv: usize = 0x1E1; // bool
                pub const m_flOffset: usize = 0x1E4; // float32
                pub const m_flAgeMin: usize = 0x1E8; // float32
                pub const m_flAgeMax: usize = 0x1EC; // float32
                pub const m_flNoiseScale: usize = 0x1F0; // float32
                pub const m_flNoiseScaleLoc: usize = 0x1F4; // float32
                pub const m_vecOffsetLoc: usize = 0x1F8; // Vector
            };
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
            pub const C_OP_RemapVectorComponentToScalar = struct {
                pub const m_nFieldInput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nComponent: usize = 0x1E0; // int32
            };
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
            pub const CGeneralRandomRotation = struct {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flDegrees: usize = 0x1E4; // float32
                pub const m_flDegreesMin: usize = 0x1E8; // float32
                pub const m_flDegreesMax: usize = 0x1EC; // float32
                pub const m_flRotationRandExponent: usize = 0x1F0; // float32
                pub const m_bRandomlyFlipDirection: usize = 0x1F4; // bool
            };
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
            pub const C_OP_DistanceBetweenVecs = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_vecPoint1: usize = 0x1E0; // CPerParticleVecInput
                pub const m_vecPoint2: usize = 0x898; // CPerParticleVecInput
                pub const m_flInputMin: usize = 0xF50; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x10C0; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x1230; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x13A0; // CPerParticleFloatInput
                pub const m_nSetMethod: usize = 0x1510; // ParticleSetMethod_t
                pub const m_bDeltaTime: usize = 0x1514; // bool
            };
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
            pub const C_OP_DampenToCP = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_flRange: usize = 0x1DC; // float32
                pub const m_flScale: usize = 0x1E0; // float32
            };
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
            pub const C_OP_CalculateVectorAttribute = struct {
                pub const m_vStartValue: usize = 0x1D8; // Vector
                pub const m_nFieldInput1: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flInputScale1: usize = 0x1E8; // float32
                pub const m_nFieldInput2: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_flInputScale2: usize = 0x1F0; // float32
                pub const m_nControlPointInput1: usize = 0x1F4; // ControlPointReference_t
                pub const m_flControlPointScale1: usize = 0x208; // float32
                pub const m_nControlPointInput2: usize = 0x20C; // ControlPointReference_t
                pub const m_flControlPointScale2: usize = 0x220; // float32
                pub const m_nFieldOutput: usize = 0x224; // ParticleAttributeIndex_t
                pub const m_vFinalOutputScale: usize = 0x228; // Vector
            };
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
            pub const C_OP_LockToBone = struct {
                pub const m_modelInput: usize = 0x1D8; // CParticleModelInput
                pub const m_transformInput: usize = 0x238; // CParticleTransformInput
                pub const m_flLifeTimeFadeStart: usize = 0x2A0; // float32
                pub const m_flLifeTimeFadeEnd: usize = 0x2A4; // float32
                pub const m_flJumpThreshold: usize = 0x2A8; // float32
                pub const m_flPrevPosScale: usize = 0x2AC; // float32
                pub const m_HitboxSetName: usize = 0x2B0; // char[128]
                pub const m_bRigid: usize = 0x330; // bool
                pub const m_bUseBones: usize = 0x331; // bool
                pub const m_nFieldOutput: usize = 0x334; // ParticleAttributeIndex_t
                pub const m_nFieldOutputPrev: usize = 0x338; // ParticleAttributeIndex_t
                pub const m_nRotationSetType: usize = 0x33C; // ParticleRotationLockType_t
                pub const m_bRigidRotationLock: usize = 0x340; // bool
                pub const m_vecRotation: usize = 0x348; // CPerParticleVecInput
                pub const m_flRotLerp: usize = 0xA00; // CPerParticleFloatInput
            };
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
            pub const C_OP_RemapNamedModelBodyPartOnceTimed = struct {
            };
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
            pub const C_OP_ScreenSpaceRotateTowardTarget = struct {
                pub const m_vecTargetPosition: usize = 0x1D8; // CPerParticleVecInput
                pub const m_flOutputRemap: usize = 0x890; // CParticleRemapFloatInput
                pub const m_nSetMethod: usize = 0xA00; // ParticleSetMethod_t
                pub const m_flScreenEdgeAlignmentDistance: usize = 0xA08; // CPerParticleFloatInput
            };
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
            pub const C_OP_MovementMaintainOffset = struct {
                pub const m_vecOffset: usize = 0x1D8; // Vector
                pub const m_nCP: usize = 0x1E4; // int32
                pub const m_bRadiusScale: usize = 0x1E8; // bool
            };
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
            pub const C_INIT_CreateWithinCapsuleTransform = struct {
                pub const m_fRadiusMin: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_fRadiusMax: usize = 0x350; // CPerParticleFloatInput
                pub const m_fHeight: usize = 0x4C0; // CPerParticleFloatInput
                pub const m_TransformInput: usize = 0x630; // CParticleTransformInput
                pub const m_fSpeedMin: usize = 0x698; // CPerParticleFloatInput
                pub const m_fSpeedMax: usize = 0x808; // CPerParticleFloatInput
                pub const m_fSpeedRandExp: usize = 0x978; // float32
                pub const m_LocalCoordinateSystemSpeedMin: usize = 0x980; // CPerParticleVecInput
                pub const m_LocalCoordinateSystemSpeedMax: usize = 0x1038; // CPerParticleVecInput
                pub const m_nFieldOutput: usize = 0x16F0; // ParticleAttributeIndex_t
                pub const m_nFieldVelocity: usize = 0x16F4; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_CreateFromParentParticles = struct {
                pub const m_flVelocityScale: usize = 0x1E0; // float32
                pub const m_flIncrement: usize = 0x1E4; // float32
                pub const m_bRandomDistribution: usize = 0x1E8; // bool
                pub const m_nRandomSeed: usize = 0x1EC; // int32
                pub const m_bSubFrame: usize = 0x1F0; // bool
                pub const m_bSetRopeSegmentID: usize = 0x1F1; // bool
            };
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
            pub const C_INIT_CheckParticleForWater = struct {
                pub const m_flRadius: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nFieldOutput: usize = 0x350; // ParticleAttributeIndex_t
                pub const m_flOutputRemap: usize = 0x358; // CParticleRemapFloatInput
                pub const m_nSetMethod: usize = 0x4C8; // ParticleSetMethod_t
            };
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
            pub const C_INIT_RandomNamedModelBodyPart = struct {
            };
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
            pub const CPAssignment_t = struct {
                pub const m_nCPNumber: usize = 0x0; // int32
                pub const m_Pos: usize = 0x8; // CPerParticleVecInput
                pub const m_nOrientationMode: usize = 0x6C0; // ParticleOrientationSetMode_t
            };
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
            pub const C_INIT_RemapParticleCountToNamedModelBodyPartScalar = struct {
            };
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
            pub const C_INIT_InitSkinnedPositionFromCPSnapshot = struct {
                pub const m_nSnapshotControlPointNumber: usize = 0x1E0; // int32
                pub const m_nControlPointNumber: usize = 0x1E4; // int32
                pub const m_bRandom: usize = 0x1E8; // bool
                pub const m_nRandomSeed: usize = 0x1EC; // int32
                pub const m_bRigid: usize = 0x1F0; // bool
                pub const m_bSetNormal: usize = 0x1F1; // bool
                pub const m_bIgnoreDt: usize = 0x1F2; // bool
                pub const m_flMinNormalVelocity: usize = 0x1F4; // float32
                pub const m_flMaxNormalVelocity: usize = 0x1F8; // float32
                pub const m_nIndexType: usize = 0x1FC; // SnapshotIndexType_t
                pub const m_flReadIndex: usize = 0x200; // CPerParticleFloatInput
                pub const m_flIncrement: usize = 0x370; // float32
                pub const m_nFullLoopIncrement: usize = 0x374; // int32
                pub const m_nSnapShotStartPoint: usize = 0x378; // int32
                pub const m_flBoneVelocity: usize = 0x37C; // float32
                pub const m_flBoneVelocityMax: usize = 0x380; // float32
                pub const m_bCopyColor: usize = 0x384; // bool
                pub const m_bCopyAlpha: usize = 0x385; // bool
                pub const m_bSetRadius: usize = 0x386; // bool
            };
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
            pub const C_OP_LagCompensation = struct {
                pub const m_nDesiredVelocityCP: usize = 0x1D8; // int32
                pub const m_nLatencyCP: usize = 0x1DC; // int32
                pub const m_nLatencyCPField: usize = 0x1E0; // int32
                pub const m_nDesiredVelocityCPField: usize = 0x1E4; // int32
            };
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
            pub const C_OP_Noise = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flOutputMin: usize = 0x1DC; // float32
                pub const m_flOutputMax: usize = 0x1E0; // float32
                pub const m_fl4NoiseScale: usize = 0x1E4; // float32
                pub const m_bAdditive: usize = 0x1E8; // bool
                pub const m_flNoiseAnimationTimeScale: usize = 0x1EC; // float32
            };
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
            pub const C_OP_FadeAndKillForTracers = struct {
                pub const m_flStartFadeInTime: usize = 0x1D8; // float32
                pub const m_flEndFadeInTime: usize = 0x1DC; // float32
                pub const m_flStartFadeOutTime: usize = 0x1E0; // float32
                pub const m_flEndFadeOutTime: usize = 0x1E4; // float32
                pub const m_flStartAlpha: usize = 0x1E8; // float32
                pub const m_flEndAlpha: usize = 0x1EC; // float32
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const CParticleMassCalculationParameters = struct {
                pub const m_nMassMode: usize = 0x0; // ParticleMassMode_t
                pub const m_flRadius: usize = 0x8; // CPerParticleFloatInput
                pub const m_flNominalRadius: usize = 0x178; // CPerParticleFloatInput
                pub const m_flScale: usize = 0x2E8; // CPerParticleFloatInput
            };
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
            pub const C_OP_SequenceFromModel = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nFieldOutputAnim: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_nSetMethod: usize = 0x1F4; // ParticleSetMethod_t
            };
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
            pub const C_OP_AlphaDecay = struct {
                pub const m_flMinAlpha: usize = 0x1D8; // float32
            };
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
            pub const C_INIT_InitVec = struct {
                pub const m_InputValue: usize = 0x1E0; // CPerParticleVecInput
                pub const m_nOutputField: usize = 0x898; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x89C; // ParticleSetMethod_t
                pub const m_bNormalizedOutput: usize = 0x8A0; // bool
                pub const m_bWritePreviousPosition: usize = 0x8A1; // bool
            };
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
            pub const C_INIT_SetHitboxToModel = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nForceInModel: usize = 0x1E4; // int32
                pub const m_bEvenDistribution: usize = 0x1E8; // bool
                pub const m_nDesiredHitbox: usize = 0x1EC; // int32
                pub const m_vecHitBoxScale: usize = 0x1F0; // CParticleCollectionVecInput
                pub const m_vecDirectionBias: usize = 0x8A8; // Vector
                pub const m_bMaintainHitbox: usize = 0x8B4; // bool
                pub const m_bUseBones: usize = 0x8B5; // bool
                pub const m_HitboxSetName: usize = 0x8B6; // char[128]
                pub const m_flShellSize: usize = 0x938; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_MovementMoveAlongSkinnedCPSnapshot = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_nSnapshotControlPointNumber: usize = 0x1DC; // int32
                pub const m_bSetNormal: usize = 0x1E0; // bool
                pub const m_bSetRadius: usize = 0x1E1; // bool
                pub const m_flInterpolation: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flTValue: usize = 0x358; // CPerParticleFloatInput
            };
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
            pub const C_OP_LerpScalar = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flOutput: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flStartTime: usize = 0x350; // float32
                pub const m_flEndTime: usize = 0x354; // float32
            };
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
            pub const C_INIT_InitialRepulsionVelocity = struct {
                pub const m_CollisionGroupName: usize = 0x1E0; // char[128]
                pub const m_nTraceSet: usize = 0x260; // ParticleTraceSet_t
                pub const m_vecOutputMin: usize = 0x264; // Vector
                pub const m_vecOutputMax: usize = 0x270; // Vector
                pub const m_nControlPointNumber: usize = 0x27C; // int32
                pub const m_bPerParticle: usize = 0x280; // bool
                pub const m_bTranslate: usize = 0x281; // bool
                pub const m_bProportional: usize = 0x282; // bool
                pub const m_flTraceLength: usize = 0x284; // float32
                pub const m_bPerParticleTR: usize = 0x288; // bool
                pub const m_bInherit: usize = 0x289; // bool
                pub const m_nChildCP: usize = 0x28C; // int32
                pub const m_nChildGroupID: usize = 0x290; // int32
            };
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
            pub const C_OP_ClampScalar = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flOutputMin: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x350; // CPerParticleFloatInput
            };
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
            pub const C_OP_SetControlPointToHMD = struct {
                pub const m_nCP1: usize = 0x1E0; // int32
                pub const m_vecCP1Pos: usize = 0x1E4; // Vector
                pub const m_bOrientToHMD: usize = 0x1F0; // bool
            };
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
            pub const C_OP_DifferencePreviousParticle = struct {
                pub const m_nFieldInput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E0; // float32
                pub const m_flInputMax: usize = 0x1E4; // float32
                pub const m_flOutputMin: usize = 0x1E8; // float32
                pub const m_flOutputMax: usize = 0x1EC; // float32
                pub const m_nSetMethod: usize = 0x1F0; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x1F4; // bool
                pub const m_bSetPreviousParticle: usize = 0x1F5; // bool
            };
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
            pub const C_OP_PercentageBetweenTransforms = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1DC; // float32
                pub const m_flInputMax: usize = 0x1E0; // float32
                pub const m_flOutputMin: usize = 0x1E4; // float32
                pub const m_flOutputMax: usize = 0x1E8; // float32
                pub const m_TransformStart: usize = 0x1F0; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x258; // CParticleTransformInput
                pub const m_nSetMethod: usize = 0x2C0; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x2C4; // bool
                pub const m_bRadialCheck: usize = 0x2C5; // bool
            };
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
            pub const C_OP_RemapNamedModelSequenceEndCap = struct {
            };
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
            pub const C_INIT_InitFromCPSnapshot = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_strSnapshotSubset: usize = 0x1E8; // CUtlString
                pub const m_nAttributeToRead: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_nAttributeToWrite: usize = 0x1F4; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x1F8; // int32
                pub const m_bRandom: usize = 0x1FC; // bool
                pub const m_bReverse: usize = 0x1FD; // bool
                pub const m_nSnapShotIncrement: usize = 0x200; // CParticleCollectionFloatInput
                pub const m_nManualSnapshotIndex: usize = 0x370; // CPerParticleFloatInput
                pub const m_nRandomSeed: usize = 0x4E0; // int32
                pub const m_bLocalSpaceAngles: usize = 0x4E4; // bool
            };
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
            pub const C_OP_RenderCables = struct {
                pub const m_flRadiusScale: usize = 0x228; // CParticleCollectionFloatInput
                pub const m_flAlphaScale: usize = 0x398; // CParticleCollectionFloatInput
                pub const m_vecColorScale: usize = 0x508; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0xBC0; // ParticleColorBlendType_t
                pub const m_hMaterial: usize = 0xBC8; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_nTextureRepetitionMode: usize = 0xBD0; // TextureRepetitionMode_t
                pub const m_flTextureRepeatsPerSegment: usize = 0xBD8; // CParticleCollectionFloatInput
                pub const m_flTextureRepeatsCircumference: usize = 0xD48; // CParticleCollectionFloatInput
                pub const m_flColorMapOffsetV: usize = 0xEB8; // CParticleCollectionFloatInput
                pub const m_flColorMapOffsetU: usize = 0x1028; // CParticleCollectionFloatInput
                pub const m_flNormalMapOffsetV: usize = 0x1198; // CParticleCollectionFloatInput
                pub const m_flNormalMapOffsetU: usize = 0x1308; // CParticleCollectionFloatInput
                pub const m_bDrawCableCaps: usize = 0x1478; // bool
                pub const m_flCapRoundness: usize = 0x147C; // float32
                pub const m_flCapOffsetAmount: usize = 0x1480; // float32
                pub const m_flTessScale: usize = 0x1484; // float32
                pub const m_nMinTesselation: usize = 0x1488; // int32
                pub const m_nMaxTesselation: usize = 0x148C; // int32
                pub const m_nRoundness: usize = 0x1490; // int32
                pub const m_nForceRoundnessFixed: usize = 0x1494; // bool
                pub const m_bOnlyRenderInEffectsBloomPass: usize = 0x1495; // bool
                pub const m_LightingTransform: usize = 0x1498; // CParticleTransformInput
                pub const m_MaterialFloatVars: usize = 0x1500; // CUtlLeanVector<FloatInputMaterialVariable_t>
                pub const m_MaterialVecVars: usize = 0x1520; // CUtlLeanVector<VecInputMaterialVariable_t>
            };
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
            pub const C_INIT_InheritVelocity = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_flVelocityScale: usize = 0x1E4; // float32
            };
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
            pub const C_INIT_NormalAlignToCP = struct {
                pub const m_transformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_nControlPointAxis: usize = 0x248; // ParticleControlPointAxis_t
            };
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
            pub const C_OP_SetChildControlPoints = struct {
                pub const m_nChildGroupID: usize = 0x1D8; // int32
                pub const m_nFirstControlPoint: usize = 0x1DC; // int32
                pub const m_nNumControlPoints: usize = 0x1E0; // int32
                pub const m_nFirstSourcePoint: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_bReverse: usize = 0x358; // bool
                pub const m_bSetOrientation: usize = 0x359; // bool
                pub const m_nOrientation: usize = 0x35C; // ParticleOrientationType_t
            };
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
            pub const C_OP_ChladniWave = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x350; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4C0; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x630; // CPerParticleFloatInput
                pub const m_vecWaveLength: usize = 0x7A0; // CPerParticleVecInput
                pub const m_vecHarmonics: usize = 0xE58; // CPerParticleVecInput
                pub const m_nSetMethod: usize = 0x1510; // ParticleSetMethod_t
                pub const m_nLocalSpaceControlPoint: usize = 0x1514; // int32
                pub const m_b3D: usize = 0x1518; // bool
            };
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
            pub const C_OP_RemapDirectionToCPToVector = struct {
                pub const m_nCP: usize = 0x1D8; // int32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1E0; // float32
                pub const m_flOffsetRot: usize = 0x1E4; // float32
                pub const m_vecOffsetAxis: usize = 0x1E8; // Vector
                pub const m_bNormalize: usize = 0x1F4; // bool
                pub const m_nFieldStrength: usize = 0x1F8; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_DriveCPFromGlobalSoundFloat = struct {
                pub const m_nOutputControlPoint: usize = 0x1E0; // int32
                pub const m_nOutputField: usize = 0x1E4; // int32
                pub const m_flInputMin: usize = 0x1E8; // float32
                pub const m_flInputMax: usize = 0x1EC; // float32
                pub const m_flOutputMin: usize = 0x1F0; // float32
                pub const m_flOutputMax: usize = 0x1F4; // float32
                pub const m_StackName: usize = 0x1F8; // CUtlString
                pub const m_OperatorName: usize = 0x200; // CUtlString
                pub const m_FieldName: usize = 0x208; // CUtlString
            };
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
            pub const C_INIT_ScreenSpacePositionOfTarget = struct {
                pub const m_vecTargetPosition: usize = 0x1E0; // CPerParticleVecInput
                pub const m_bOututBehindness: usize = 0x898; // bool
                pub const m_nBehindFieldOutput: usize = 0x89C; // ParticleAttributeIndex_t
                pub const m_flBehindOutputRemap: usize = 0x8A0; // CParticleRemapFloatInput
            };
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
            pub const C_OP_RtEnvCull = struct {
                pub const m_vecTestDir: usize = 0x1D8; // Vector
                pub const m_vecTestNormal: usize = 0x1E4; // Vector
                pub const m_bCullOnMiss: usize = 0x1F0; // bool
                pub const m_bStickInsteadOfCull: usize = 0x1F1; // bool
                pub const m_RtEnvName: usize = 0x1F2; // char[128]
                pub const m_nRTEnvCP: usize = 0x274; // int32
                pub const m_nComponent: usize = 0x278; // int32
            };
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
            pub const C_OP_PinParticleToCP = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_vecOffset: usize = 0x1E0; // CParticleCollectionVecInput
                pub const m_bOffsetLocal: usize = 0x898; // bool
                pub const m_nParticleSelection: usize = 0x89C; // ParticleSelection_t
                pub const m_nParticleNumber: usize = 0x8A0; // CParticleCollectionFloatInput
                pub const m_nPinBreakType: usize = 0xA10; // ParticlePinDistance_t
                pub const m_flBreakDistance: usize = 0xA18; // CParticleCollectionFloatInput
                pub const m_flBreakSpeed: usize = 0xB88; // CParticleCollectionFloatInput
                pub const m_flAge: usize = 0xCF8; // CParticleCollectionFloatInput
                pub const m_nBreakControlPointNumber: usize = 0xE68; // int32
                pub const m_nBreakControlPointNumber2: usize = 0xE6C; // int32
                pub const m_flBreakValue: usize = 0xE70; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0xFE0; // CPerParticleFloatInput
                pub const m_bRetainInitialVelocity: usize = 0x1150; // bool
            };
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
            pub const C_OP_RemapCPtoVector = struct {
                pub const m_nCPInput: usize = 0x1D8; // int32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x1E0; // int32
                pub const m_vInputMin: usize = 0x1E4; // Vector
                pub const m_vInputMax: usize = 0x1F0; // Vector
                pub const m_vOutputMin: usize = 0x1FC; // Vector
                pub const m_vOutputMax: usize = 0x208; // Vector
                pub const m_flStartTime: usize = 0x214; // float32
                pub const m_flEndTime: usize = 0x218; // float32
                pub const m_flInterpRate: usize = 0x21C; // float32
                pub const m_nSetMethod: usize = 0x220; // ParticleSetMethod_t
                pub const m_bOffset: usize = 0x224; // bool
                pub const m_bAccelerate: usize = 0x225; // bool
            };
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
            pub const C_INIT_CreateParticleImpulse = struct {
                pub const m_InputRadius: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_InputMagnitude: usize = 0x350; // CPerParticleFloatInput
                pub const m_nFalloffFunction: usize = 0x4C0; // ParticleFalloffFunction_t
                pub const m_InputFalloffExp: usize = 0x4C8; // CPerParticleFloatInput
                pub const m_nImpulseType: usize = 0x638; // ParticleImpulseType_t
            };
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
            pub const C_INIT_CreateInEpitrochoid = struct {
                pub const m_nComponent1: usize = 0x1E0; // int32
                pub const m_nComponent2: usize = 0x1E4; // int32
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_flParticleDensity: usize = 0x250; // CPerParticleFloatInput
                pub const m_flOffset: usize = 0x3C0; // CPerParticleFloatInput
                pub const m_flRadius1: usize = 0x530; // CPerParticleFloatInput
                pub const m_flRadius2: usize = 0x6A0; // CPerParticleFloatInput
                pub const m_bUseCount: usize = 0x810; // bool
                pub const m_bUseLocalCoords: usize = 0x811; // bool
                pub const m_bOffsetExistingPos: usize = 0x812; // bool
            };
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
            pub const C_OP_SetControlPointPositions = struct {
                pub const m_bUseWorldLocation: usize = 0x1E0; // bool
                pub const m_bOrient: usize = 0x1E1; // bool
                pub const m_bSetOnce: usize = 0x1E2; // bool
                pub const m_nCP1: usize = 0x1E4; // int32
                pub const m_nCP2: usize = 0x1E8; // int32
                pub const m_nCP3: usize = 0x1EC; // int32
                pub const m_nCP4: usize = 0x1F0; // int32
                pub const m_vecCP1Pos: usize = 0x1F4; // Vector
                pub const m_vecCP2Pos: usize = 0x200; // Vector
                pub const m_vecCP3Pos: usize = 0x20C; // Vector
                pub const m_vecCP4Pos: usize = 0x218; // Vector
                pub const m_nHeadLocation: usize = 0x224; // int32
            };
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
            pub const C_OP_SetFloatAttributeToVectorExpression = struct {
                pub const m_nExpression: usize = 0x1D8; // VectorFloatExpressionType_t
                pub const m_vInput1: usize = 0x1E0; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x898; // CPerParticleVecInput
                pub const m_flOutputRemap: usize = 0xF50; // CParticleRemapFloatInput
                pub const m_nOutputField: usize = 0x10C0; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x10C4; // ParticleSetMethod_t
            };
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
            pub const C_OP_MovementRotateParticleAroundAxis = struct {
                pub const m_vecRotAxis: usize = 0x1D8; // CParticleCollectionVecInput
                pub const m_flRotRate: usize = 0x890; // CParticleCollectionFloatInput
                pub const m_TransformInput: usize = 0xA00; // CParticleTransformInput
                pub const m_bLocalSpace: usize = 0xA68; // bool
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            pub const C_INIT_InitFloat = struct {
                pub const m_InputValue: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x350; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x354; // ParticleSetMethod_t
                pub const m_InputStrength: usize = 0x358; // CPerParticleFloatInput
            };
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
            pub const C_INIT_CreateOnModel = struct {
                pub const m_modelInput: usize = 0x1E0; // CParticleModelInput
                pub const m_transformInput: usize = 0x240; // CParticleTransformInput
                pub const m_nForceInModel: usize = 0x2A8; // int32
                pub const m_bScaleToVolume: usize = 0x2AC; // bool
                pub const m_bEvenDistribution: usize = 0x2AD; // bool
                pub const m_nDesiredHitbox: usize = 0x2B0; // CParticleCollectionFloatInput
                pub const m_nHitboxValueFromControlPointIndex: usize = 0x420; // int32
                pub const m_vecHitBoxScale: usize = 0x428; // CParticleCollectionVecInput
                pub const m_flBoneVelocity: usize = 0xAE0; // float32
                pub const m_flMaxBoneVelocity: usize = 0xAE4; // float32
                pub const m_vecDirectionBias: usize = 0xAE8; // CParticleCollectionVecInput
                pub const m_HitboxSetName: usize = 0x11A0; // char[128]
                pub const m_bLocalCoords: usize = 0x1220; // bool
                pub const m_bUseBones: usize = 0x1221; // bool
                pub const m_bUseMesh: usize = 0x1222; // bool
                pub const m_flShellSize: usize = 0x1228; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_InheritFromPeerSystem = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nFieldInput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x1E0; // int32
                pub const m_nGroupID: usize = 0x1E4; // int32
            };
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
            pub const C_INIT_RandomNamedModelMeshGroup = struct {
            };
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
            pub const C_OP_MaxVelocity = struct {
                pub const m_flMaxVelocity: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_flMinVelocity: usize = 0x348; // CPerParticleFloatInput
            };
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
            pub const C_OP_MaintainEmitter = struct {
                pub const m_nParticlesToMaintain: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_flStartTime: usize = 0x350; // float32
                pub const m_flEmissionDuration: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_flEmissionRate: usize = 0x4C8; // float32
                pub const m_nSnapshotControlPoint: usize = 0x4CC; // int32
                pub const m_strSnapshotSubset: usize = 0x4D0; // CUtlString
                pub const m_bEmitInstantaneously: usize = 0x4D8; // bool
                pub const m_bFinalEmitOnStop: usize = 0x4D9; // bool
                pub const m_flScale: usize = 0x4E0; // CParticleCollectionFloatInput
            };
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
            pub const C_INIT_PositionOffsetToCP = struct {
                pub const m_nControlPointNumberStart: usize = 0x1E0; // int32
                pub const m_nControlPointNumberEnd: usize = 0x1E4; // int32
                pub const m_bLocalCoords: usize = 0x1E8; // bool
            };
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
            pub const C_INIT_RemapInitialTransformDirectionToRotation = struct {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x248; // ParticleAttributeIndex_t
                pub const m_flOffsetRot: usize = 0x24C; // float32
                pub const m_nComponent: usize = 0x250; // int32
            };
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
            pub const C_OP_FadeAndKill = struct {
                pub const m_flStartFadeInTime: usize = 0x1D8; // float32
                pub const m_flEndFadeInTime: usize = 0x1DC; // float32
                pub const m_flStartFadeOutTime: usize = 0x1E0; // float32
                pub const m_flEndFadeOutTime: usize = 0x1E4; // float32
                pub const m_flStartAlpha: usize = 0x1E8; // float32
                pub const m_flEndAlpha: usize = 0x1EC; // float32
                pub const m_bForcePreserveParticleOrder: usize = 0x1F0; // bool
            };
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
            pub const C_OP_RampScalarSpline = struct {
                pub const m_RateMin: usize = 0x1D8; // float32
                pub const m_RateMax: usize = 0x1DC; // float32
                pub const m_flStartTime_min: usize = 0x1E0; // float32
                pub const m_flStartTime_max: usize = 0x1E4; // float32
                pub const m_flEndTime_min: usize = 0x1E8; // float32
                pub const m_flEndTime_max: usize = 0x1EC; // float32
                pub const m_flBias: usize = 0x1F0; // float32
                pub const m_nField: usize = 0x220; // ParticleAttributeIndex_t
                pub const m_bProportionalOp: usize = 0x224; // bool
                pub const m_bEaseOut: usize = 0x225; // bool
            };
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
            pub const C_OP_RemapNamedModelSequenceOnceTimed = struct {
            };
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
            pub const C_OP_MaintainSequentialPath = struct {
                pub const m_fMaxDistance: usize = 0x1D8; // float32
                pub const m_flNumToAssign: usize = 0x1DC; // float32
                pub const m_flCohesionStrength: usize = 0x1E0; // float32
                pub const m_flTolerance: usize = 0x1E4; // float32
                pub const m_bLoop: usize = 0x1E8; // bool
                pub const m_bUseParticleCount: usize = 0x1E9; // bool
                pub const m_PathParams: usize = 0x1F0; // CPathParameters
            };
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
            pub const C_OP_RemapNamedModelBodyPartEndCap = struct {
            };
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
            pub const C_OP_StopAfterCPDuration = struct {
                pub const m_flDuration: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_bDestroyImmediately: usize = 0x350; // bool
                pub const m_bPlayEndCap: usize = 0x351; // bool
            };
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
            pub const CGeneralSpin = struct {
                pub const m_nSpinRateDegrees: usize = 0x1D8; // int32
                pub const m_nSpinRateMinDegrees: usize = 0x1DC; // int32
                pub const m_fSpinRateStopTime: usize = 0x1E4; // float32
            };
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
            pub const C_INIT_RemapNamedModelElementToScalar = struct {
                pub const m_hModel: usize = 0x1E0; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_names: usize = 0x1E8; // CUtlVector<CUtlString>
                pub const m_values: usize = 0x200; // CUtlVector<float32>
                pub const m_nFieldInput: usize = 0x218; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x21C; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x220; // ParticleSetMethod_t
                pub const m_bModelFromRenderer: usize = 0x224; // bool
            };
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
            pub const C_OP_ClampVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_vecOutputMin: usize = 0x1E0; // CPerParticleVecInput
                pub const m_vecOutputMax: usize = 0x898; // CPerParticleVecInput
            };
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
            pub const C_OP_RenderStandardLight = struct {
                pub const m_nLightType: usize = 0x228; // ParticleLightTypeChoiceList_t
                pub const m_nMaxAllowed: usize = 0x22C; // uint16
                pub const m_vecColorScale: usize = 0x230; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0x8E8; // ParticleColorBlendType_t
                pub const m_strLightStyle: usize = 0x8F0; // CUtlString
                pub const m_flLightStyleTime: usize = 0x8F8; // CPerParticleFloatInput
                pub const m_flIntensity: usize = 0xA68; // CPerParticleFloatInput
                pub const m_bCastShadows: usize = 0xBD8; // bool
                pub const m_bDynamicBounce: usize = 0xBD9; // bool
                pub const m_flBounceScale: usize = 0xBE0; // CParticleCollectionFloatInput
                pub const m_flTheta: usize = 0xD50; // CParticleCollectionFloatInput
                pub const m_flPhi: usize = 0xEC0; // CParticleCollectionFloatInput
                pub const m_flRadiusMultiplier: usize = 0x1030; // CParticleCollectionFloatInput
                pub const m_nAttenuationStyle: usize = 0x11A0; // StandardLightingAttenuationStyle_t
                pub const m_flFalloffLinearity: usize = 0x11A8; // CParticleCollectionFloatInput
                pub const m_flFiftyPercentFalloff: usize = 0x1318; // CParticleCollectionFloatInput
                pub const m_flZeroPercentFalloff: usize = 0x1488; // CParticleCollectionFloatInput
                pub const m_bRenderDiffuse: usize = 0x15F8; // bool
                pub const m_bRenderSpecular: usize = 0x15F9; // bool
                pub const m_lightCookie: usize = 0x1600; // CUtlString
                pub const m_nPriority: usize = 0x1608; // int32
                pub const m_nFogLightingMode: usize = 0x160C; // ParticleLightFogLightingMode_t
                pub const m_flFogContribution: usize = 0x1610; // CParticleCollectionRendererFloatInput
                pub const m_nCapsuleLightBehavior: usize = 0x1780; // ParticleLightBehaviorChoiceList_t
                pub const m_flCapsuleLength: usize = 0x1784; // float32
                pub const m_bReverseOrder: usize = 0x1788; // bool
                pub const m_bClosedLoop: usize = 0x1789; // bool
                pub const m_nPrevPntSource: usize = 0x178C; // ParticleAttributeIndex_t
                pub const m_flMaxLength: usize = 0x1790; // float32
                pub const m_flMinLength: usize = 0x1794; // float32
                pub const m_bIgnoreDT: usize = 0x1798; // bool
                pub const m_flConstrainRadiusToLengthRatio: usize = 0x179C; // float32
                pub const m_flLengthScale: usize = 0x17A0; // float32
                pub const m_flLengthFadeInTime: usize = 0x17A4; // float32
            };
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
            pub const C_OP_DistanceToTransform = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x350; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4C0; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x630; // CPerParticleFloatInput
                pub const m_TransformStart: usize = 0x7A0; // CParticleTransformInput
                pub const m_bLOS: usize = 0x808; // bool
                pub const m_CollisionGroupName: usize = 0x809; // char[128]
                pub const m_nTraceSet: usize = 0x88C; // ParticleTraceSet_t
                pub const m_flMaxTraceLength: usize = 0x890; // float32
                pub const m_flLOSScale: usize = 0x894; // float32
                pub const m_nSetMethod: usize = 0x898; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x89C; // bool
                pub const m_bAdditive: usize = 0x89D; // bool
                pub const m_vecComponentScale: usize = 0x8A0; // CPerParticleVecInput
            };
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
            pub const C_OP_RemapControlPointOrientationToRotation = struct {
                pub const m_nCP: usize = 0x1D8; // int32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flOffsetRot: usize = 0x1E0; // float32
                pub const m_nComponent: usize = 0x1E4; // int32
            };
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
            pub const C_OP_SetControlPointToCenter = struct {
                pub const m_nCP1: usize = 0x1E0; // int32
                pub const m_vecCP1Pos: usize = 0x1E4; // Vector
                pub const m_bUseAvgParticlePos: usize = 0x1F0; // bool
                pub const m_nSetParent: usize = 0x1F4; // ParticleParentSetMode_t
            };
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
            pub const C_OP_RemapAverageScalarValuetoCP = struct {
                pub const m_nExpression: usize = 0x1E0; // SetStatisticExpressionType_t
                pub const m_flDecimalPlaces: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_nOutControlPointNumber: usize = 0x358; // int32
                pub const m_nOutVectorField: usize = 0x35C; // int32
                pub const m_nField: usize = 0x360; // ParticleAttributeIndex_t
                pub const m_flOutputRemap: usize = 0x368; // CParticleRemapFloatInput
            };
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
            pub const C_OP_RemapDotProductToScalar = struct {
                pub const m_nInputCP1: usize = 0x1D8; // int32
                pub const m_nInputCP2: usize = 0x1DC; // int32
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_bUseParticleVelocity: usize = 0x1F4; // bool
                pub const m_nSetMethod: usize = 0x1F8; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x1FC; // bool
                pub const m_bUseParticleNormal: usize = 0x1FD; // bool
            };
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
            pub const C_INIT_Orient2DRelToCP = struct {
                pub const m_nCP: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flRotOffset: usize = 0x1E8; // float32
            };
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
            pub const C_OP_FadeIn = struct {
                pub const m_flFadeInTimeMin: usize = 0x1D8; // float32
                pub const m_flFadeInTimeMax: usize = 0x1DC; // float32
                pub const m_flFadeInTimeExp: usize = 0x1E0; // float32
                pub const m_bProportional: usize = 0x1E4; // bool
            };
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
            pub const C_OP_RemapVectorToRotations = struct {
                pub const m_vecInput: usize = 0x1D8; // CPerParticleVecInput
                pub const m_vecRotation: usize = 0x890; // CPerParticleVecInput
            };
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
            pub const C_INIT_GlobalScale = struct {
                pub const m_flScale: usize = 0x1E0; // float32
                pub const m_nScaleControlPointNumber: usize = 0x1E4; // int32
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_bScaleRadius: usize = 0x1EC; // bool
                pub const m_bScalePosition: usize = 0x1ED; // bool
                pub const m_bScaleVelocity: usize = 0x1EE; // bool
            };
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
            pub const C_INIT_RadiusFromCPObject = struct {
                pub const m_nControlPoint: usize = 0x1E0; // int32
            };
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
            pub const C_INIT_InitialVelocityFromHitbox = struct {
                pub const m_flVelocityMin: usize = 0x1E0; // float32
                pub const m_flVelocityMax: usize = 0x1E4; // float32
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_HitboxSetName: usize = 0x1EC; // char[128]
                pub const m_bUseBones: usize = 0x26C; // bool
            };
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
            pub const C_OP_LerpVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_vecOutput: usize = 0x1DC; // Vector
                pub const m_flStartTime: usize = 0x1E8; // float32
                pub const m_flEndTime: usize = 0x1EC; // float32
                pub const m_nSetMethod: usize = 0x1F0; // ParticleSetMethod_t
            };
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
            pub const C_OP_SetControlPointFieldToWater = struct {
                pub const m_nSourceCP: usize = 0x1E0; // int32
                pub const m_nDestCP: usize = 0x1E4; // int32
                pub const m_nCPField: usize = 0x1E8; // int32
            };
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // PARTICLE_VRHAND_RIGHT
            // PARTICLE_VRHAND_CP
            pub const TextureGroup_t = struct {
                pub const m_bEnabled: usize = 0x0; // bool
                pub const m_bReplaceTextureWithGradient: usize = 0x1; // bool
                pub const m_hTexture: usize = 0x8; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_Gradient: usize = 0x10; // CColorGradient
                pub const m_nTextureType: usize = 0x28; // SpriteCardTextureType_t
                pub const m_nTextureChannels: usize = 0x2C; // SpriteCardTextureChannel_t
                pub const m_nTextureBlendMode: usize = 0x30; // ParticleTextureLayerBlendType_t
                pub const m_flTextureBlend: usize = 0x38; // CParticleCollectionRendererFloatInput
                pub const m_TextureControls: usize = 0x1A8; // TextureControls_t
            };
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
            pub const C_OP_SetCPOrientationToGroundNormal = struct {
                pub const m_flInterpRate: usize = 0x1D8; // float32
                pub const m_flMaxTraceLength: usize = 0x1DC; // float32
                pub const m_flTolerance: usize = 0x1E0; // float32
                pub const m_flTraceOffset: usize = 0x1E4; // float32
                pub const m_CollisionGroupName: usize = 0x1E8; // char[128]
                pub const m_nTraceSet: usize = 0x268; // ParticleTraceSet_t
                pub const m_nInputCP: usize = 0x26C; // int32
                pub const m_nOutputCP: usize = 0x270; // int32
                pub const m_bIncludeWater: usize = 0x280; // bool
            };
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
            pub const C_OP_SnapshotSkinToBones = struct {
                pub const m_bTransformNormals: usize = 0x1D8; // bool
                pub const m_bTransformRadii: usize = 0x1D9; // bool
                pub const m_nControlPointNumber: usize = 0x1DC; // int32
                pub const m_flLifeTimeFadeStart: usize = 0x1E0; // float32
                pub const m_flLifeTimeFadeEnd: usize = 0x1E4; // float32
                pub const m_flJumpThreshold: usize = 0x1E8; // float32
                pub const m_flPrevPosScale: usize = 0x1EC; // float32
            };
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
            pub const C_INIT_CreateWithinSphereTransform = struct {
                pub const m_fRadiusMin: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_fRadiusMax: usize = 0x350; // CPerParticleFloatInput
                pub const m_vecDistanceBias: usize = 0x4C0; // CPerParticleVecInput
                pub const m_vecDistanceBiasAbs: usize = 0xB78; // Vector
                pub const m_TransformInput: usize = 0xB88; // CParticleTransformInput
                pub const m_fSpeedMin: usize = 0xBF0; // CPerParticleFloatInput
                pub const m_fSpeedMax: usize = 0xD60; // CPerParticleFloatInput
                pub const m_fSpeedRandExp: usize = 0xED0; // float32
                pub const m_bLocalCoords: usize = 0xED4; // bool
                pub const m_LocalCoordinateSystemSpeedMin: usize = 0xED8; // CPerParticleVecInput
                pub const m_LocalCoordinateSystemSpeedMax: usize = 0x1590; // CPerParticleVecInput
                pub const m_nFieldOutput: usize = 0x1C48; // ParticleAttributeIndex_t
                pub const m_nFieldVelocity: usize = 0x1C4C; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_RadiusDecay = struct {
                pub const m_flMinRadius: usize = 0x1D8; // float32
            };
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
            pub const C_INIT_RemapNamedModelBodyPartToScalar = struct {
            };
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
            pub const C_INIT_RemapScalarToVector = struct {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // float32
                pub const m_flInputMax: usize = 0x1EC; // float32
                pub const m_vecOutputMin: usize = 0x1F0; // Vector
                pub const m_vecOutputMax: usize = 0x1FC; // Vector
                pub const m_flStartTime: usize = 0x208; // float32
                pub const m_flEndTime: usize = 0x20C; // float32
                pub const m_nSetMethod: usize = 0x210; // ParticleSetMethod_t
                pub const m_nControlPointNumber: usize = 0x214; // int32
                pub const m_bLocalCoords: usize = 0x218; // bool
                pub const m_flRemapBias: usize = 0x21C; // float32
            };
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
            pub const C_INIT_InitialSequenceFromModel = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nFieldOutputAnim: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1EC; // float32
                pub const m_flInputMax: usize = 0x1F0; // float32
                pub const m_flOutputMin: usize = 0x1F4; // float32
                pub const m_flOutputMax: usize = 0x1F8; // float32
                pub const m_nSetMethod: usize = 0x1FC; // ParticleSetMethod_t
            };
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
            pub const C_OP_NoiseEmitter = struct {
                pub const m_flEmissionDuration: usize = 0x1E0; // float32
                pub const m_flStartTime: usize = 0x1E4; // float32
                pub const m_flEmissionScale: usize = 0x1E8; // float32
                pub const m_nScaleControlPoint: usize = 0x1EC; // int32
                pub const m_nScaleControlPointField: usize = 0x1F0; // int32
                pub const m_nWorldNoisePoint: usize = 0x1F4; // int32
                pub const m_bAbsVal: usize = 0x1F8; // bool
                pub const m_bAbsValInv: usize = 0x1F9; // bool
                pub const m_flOffset: usize = 0x1FC; // float32
                pub const m_flOutputMin: usize = 0x200; // float32
                pub const m_flOutputMax: usize = 0x204; // float32
                pub const m_flNoiseScale: usize = 0x208; // float32
                pub const m_flWorldNoiseScale: usize = 0x20C; // float32
                pub const m_vecOffsetLoc: usize = 0x210; // Vector
                pub const m_flWorldTimeScale: usize = 0x21C; // float32
            };
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
            pub const C_OP_SelectivelyEnableChildren = struct {
                pub const m_nChildGroupID: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_nFirstChild: usize = 0x350; // CParticleCollectionFloatInput
                pub const m_nNumChildrenToEnable: usize = 0x4C0; // CParticleCollectionFloatInput
                pub const m_bPlayEndcapOnStop: usize = 0x630; // bool
                pub const m_bDestroyImmediately: usize = 0x631; // bool
            };
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
            pub const ModelReference_t = struct {
                pub const m_model: usize = 0x0; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_flRelativeProbabilityOfSpawn: usize = 0x8; // float32
            };
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
            pub const C_INIT_CreateFromCPs = struct {
                pub const m_nIncrement: usize = 0x1E0; // int32
                pub const m_nMinCP: usize = 0x1E4; // int32
                pub const m_nMaxCP: usize = 0x1E8; // int32
                pub const m_nDynamicCPCount: usize = 0x1F0; // CParticleCollectionFloatInput
            };
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
            pub const C_INIT_CreateSpiralSphere = struct {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_flDensity: usize = 0x248; // CPerParticleFloatInput
                pub const m_flInitialRadius: usize = 0x3B8; // CPerParticleFloatInput
                pub const m_flInitialSpeedMin: usize = 0x528; // CPerParticleFloatInput
                pub const m_flInitialSpeedMax: usize = 0x698; // CPerParticleFloatInput
                pub const m_bUseParticleCount: usize = 0x808; // bool
            };
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
            pub const C_OP_CPVelocityForce = struct {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_flScale: usize = 0x1F0; // CPerParticleFloatInput
            };
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
            pub const C_OP_RemapNamedModelElementEndCap = struct {
                pub const m_hModel: usize = 0x1D8; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_inNames: usize = 0x1E0; // CUtlVector<CUtlString>
                pub const m_outNames: usize = 0x1F8; // CUtlVector<CUtlString>
                pub const m_fallbackNames: usize = 0x210; // CUtlVector<CUtlString>
                pub const m_bModelFromRenderer: usize = 0x228; // bool
                pub const m_nFieldInput: usize = 0x22C; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x230; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_ScaleVelocity = struct {
                pub const m_vecScale: usize = 0x1E0; // CParticleCollectionVecInput
            };
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
            pub const C_OP_MoveToHitbox = struct {
                pub const m_modelInput: usize = 0x1D8; // CParticleModelInput
                pub const m_transformInput: usize = 0x238; // CParticleTransformInput
                pub const m_flLifeTimeLerpStart: usize = 0x2A4; // float32
                pub const m_flLifeTimeLerpEnd: usize = 0x2A8; // float32
                pub const m_flPrevPosScale: usize = 0x2AC; // float32
                pub const m_HitboxSetName: usize = 0x2B0; // char[128]
                pub const m_bUseBones: usize = 0x330; // bool
                pub const m_nLerpType: usize = 0x334; // HitboxLerpType_t
                pub const m_flInterpolation: usize = 0x338; // CPerParticleFloatInput
            };
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
            pub const C_OP_PinRopeSegmentParticleToParent = struct {
                pub const m_nParticleSelection: usize = 0x1D8; // ParticleSelection_t
                pub const m_nParticleNumber: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0x350; // CPerParticleFloatInput
            };
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
            pub const C_INIT_PointList = struct {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_pointList: usize = 0x1E8; // CUtlVector<PointDefinition_t>
                pub const m_bPlaceAlongPath: usize = 0x200; // bool
                pub const m_bClosedLoop: usize = 0x201; // bool
                pub const m_nNumPointsAlongPath: usize = 0x204; // int32
            };
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
            pub const C_OP_LerpToOtherAttribute = struct {
                pub const m_flInterpolation: usize = 0x1D8; // CPerParticleFloatInput
                pub const m_nFieldInputFrom: usize = 0x348; // ParticleAttributeIndex_t
                pub const m_nFieldInput: usize = 0x34C; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x350; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_RemapParticleCountToScalar = struct {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nInputMin: usize = 0x1E4; // int32
                pub const m_nInputMax: usize = 0x1E8; // int32
                pub const m_nScaleControlPoint: usize = 0x1EC; // int32
                pub const m_nScaleControlPointField: usize = 0x1F0; // int32
                pub const m_flOutputMin: usize = 0x1F4; // float32
                pub const m_flOutputMax: usize = 0x1F8; // float32
                pub const m_nSetMethod: usize = 0x1FC; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x200; // bool
                pub const m_bInvert: usize = 0x201; // bool
                pub const m_bWrap: usize = 0x202; // bool
                pub const m_flRemapBias: usize = 0x204; // float32
            };
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
            pub const C_INIT_InheritFromParentParticles = struct {
                pub const m_flScale: usize = 0x1E0; // float32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x1E8; // int32
                pub const m_bRandomDistribution: usize = 0x1EC; // bool
                pub const m_nRandomSeed: usize = 0x1F0; // int32
            };
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
            pub const C_OP_RampScalarLinearSimple = struct {
                pub const m_Rate: usize = 0x1D8; // float32
                pub const m_flStartTime: usize = 0x1DC; // float32
                pub const m_flEndTime: usize = 0x1E0; // float32
                pub const m_nField: usize = 0x210; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_ChaoticAttractor = struct {
                pub const m_flAParm: usize = 0x1E0; // float32
                pub const m_flBParm: usize = 0x1E4; // float32
                pub const m_flCParm: usize = 0x1E8; // float32
                pub const m_flDParm: usize = 0x1EC; // float32
                pub const m_flScale: usize = 0x1F0; // float32
                pub const m_flSpeedMin: usize = 0x1F4; // float32
                pub const m_flSpeedMax: usize = 0x1F8; // float32
                pub const m_nBaseCP: usize = 0x1FC; // int32
                pub const m_bUniformSpeed: usize = 0x200; // bool
            };
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
            pub const C_OP_MovementRigidAttachToCP = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_nScaleControlPoint: usize = 0x1DC; // int32
                pub const m_nScaleCPField: usize = 0x1E0; // int32
                pub const m_nFieldInput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_bOffsetLocal: usize = 0x1EC; // bool
            };
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
            pub const C_INIT_DistanceToCPInit = struct {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x358; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4C8; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x638; // CPerParticleFloatInput
                pub const m_nStartCP: usize = 0x7A8; // int32
                pub const m_bLOS: usize = 0x7AC; // bool
                pub const m_CollisionGroupName: usize = 0x7AD; // char[128]
                pub const m_nTraceSet: usize = 0x830; // ParticleTraceSet_t
                pub const m_flMaxTraceLength: usize = 0x838; // CPerParticleFloatInput
                pub const m_flLOSScale: usize = 0x9A8; // float32
                pub const m_nSetMethod: usize = 0x9AC; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x9B0; // bool
                pub const m_vecDistanceScale: usize = 0x9B4; // Vector
                pub const m_flRemapBias: usize = 0x9C0; // float32
            };
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
            pub const C_OP_EndCapDecay = struct {
            };
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
            pub const C_OP_RemapDensityToVector = struct {
                pub const m_flRadiusScale: usize = 0x1D8; // float32
                pub const m_nFieldOutput: usize = 0x1DC; // ParticleAttributeIndex_t
                pub const m_flDensityMin: usize = 0x1E0; // float32
                pub const m_flDensityMax: usize = 0x1E4; // float32
                pub const m_vecOutputMin: usize = 0x1E8; // Vector
                pub const m_vecOutputMax: usize = 0x1F4; // Vector
                pub const m_bUseParentDensity: usize = 0x200; // bool
                pub const m_nVoxelGridResolution: usize = 0x204; // int32
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // PARTICLE_WORLDSPACE_CENTER
            // PARTICLE_EYES
            pub const ParticleControlPointConfiguration_t = struct {
                pub const m_name: usize = 0x0; // CUtlString
                pub const m_drivers: usize = 0x8; // CUtlVector<ParticleControlPointDriver_t>
                pub const m_previewState: usize = 0x20; // ParticlePreviewState_t
            };
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
            pub const C_INIT_SetRigidAttachment = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nFieldInput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_bLocalSpace: usize = 0x1EC; // bool
            };
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
            pub const MaterialVariable_t = struct {
                pub const m_strVariable: usize = 0x0; // CUtlString
                pub const m_nVariableField: usize = 0x8; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0xC; // float32
            };
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
            pub const C_OP_RemapSpeed = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1DC; // float32
                pub const m_flInputMax: usize = 0x1E0; // float32
                pub const m_flOutputMin: usize = 0x1E4; // float32
                pub const m_flOutputMax: usize = 0x1E8; // float32
                pub const m_nSetMethod: usize = 0x1EC; // ParticleSetMethod_t
                pub const m_bIgnoreDelta: usize = 0x1F0; // bool
            };
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
            pub const C_OP_RenderModels = struct {
                pub const m_bOnlyRenderInEffectsBloomPass: usize = 0x228; // bool
                pub const m_bOnlyRenderInEffectsWaterPass: usize = 0x229; // bool
                pub const m_bUseMixedResolutionRendering: usize = 0x22A; // bool
                pub const m_bOnlyRenderInEffecsGameOverlay: usize = 0x22B; // bool
                pub const m_ModelList: usize = 0x230; // CUtlVector<ModelReference_t>
                pub const m_nBodyGroupField: usize = 0x248; // ParticleAttributeIndex_t
                pub const m_nSubModelField: usize = 0x24C; // ParticleAttributeIndex_t
                pub const m_bIgnoreNormal: usize = 0x250; // bool
                pub const m_bOrientZ: usize = 0x251; // bool
                pub const m_bCenterOffset: usize = 0x252; // bool
                pub const m_vecLocalOffset: usize = 0x258; // CPerParticleVecInput
                pub const m_vecLocalRotation: usize = 0x910; // CPerParticleVecInput
                pub const m_bIgnoreRadius: usize = 0xFC8; // bool
                pub const m_nModelScaleCP: usize = 0xFCC; // int32
                pub const m_vecComponentScale: usize = 0xFD0; // CPerParticleVecInput
                pub const m_bLocalScale: usize = 0x1688; // bool
                pub const m_nSizeCullBloat: usize = 0x168C; // int32
                pub const m_bAnimated: usize = 0x1690; // bool
                pub const m_flAnimationRate: usize = 0x1698; // CPerParticleFloatInput
                pub const m_bScaleAnimationRate: usize = 0x1808; // bool
                pub const m_bForceLoopingAnimation: usize = 0x1809; // bool
                pub const m_bResetAnimOnStop: usize = 0x180A; // bool
                pub const m_bManualAnimFrame: usize = 0x180B; // bool
                pub const m_nAnimationScaleField: usize = 0x180C; // ParticleAttributeIndex_t
                pub const m_nAnimationField: usize = 0x1810; // ParticleAttributeIndex_t
                pub const m_nManualFrameField: usize = 0x1814; // ParticleAttributeIndex_t
                pub const m_ActivityName: usize = 0x1818; // char[256]
                pub const m_SequenceName: usize = 0x1918; // char[256]
                pub const m_bEnableClothSimulation: usize = 0x1A18; // bool
                pub const m_bDisableClothGroundCollision: usize = 0x1A19; // bool
                pub const m_ClothEffectName: usize = 0x1A1A; // char[64]
                pub const m_hOverrideMaterial: usize = 0x1A60; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_bOverrideTranslucentMaterials: usize = 0x1A68; // bool
                pub const m_nSkin: usize = 0x1A70; // CPerParticleFloatInput
                pub const m_MaterialVars: usize = 0x1BE0; // CUtlVector<MaterialVariable_t>
                pub const m_flRenderFilter: usize = 0x1BF8; // CPerParticleFloatInput
                pub const m_flManualModelSelection: usize = 0x1D68; // CPerParticleFloatInput
                pub const m_modelInput: usize = 0x1ED8; // CParticleModelInput
                pub const m_nLOD: usize = 0x1F38; // int32
                pub const m_EconSlotName: usize = 0x1F3C; // char[256]
                pub const m_bOriginalModel: usize = 0x203C; // bool
                pub const m_bSuppressTint: usize = 0x203D; // bool
                pub const m_nSubModelFieldType: usize = 0x2040; // RenderModelSubModelFieldType_t
                pub const m_bDisableShadows: usize = 0x2044; // bool
                pub const m_bDisableDepthPrepass: usize = 0x2045; // bool
                pub const m_bAcceptsDecals: usize = 0x2046; // bool
                pub const m_bForceDrawInterlevedWithSiblings: usize = 0x2047; // bool
                pub const m_bDoNotDrawInParticlePass: usize = 0x2048; // bool
                pub const m_bAllowApproximateTransforms: usize = 0x2049; // bool
                pub const m_szRenderAttribute: usize = 0x204A; // char[260]
                pub const m_flRadiusScale: usize = 0x2150; // CParticleCollectionFloatInput
                pub const m_flAlphaScale: usize = 0x22C0; // CParticleCollectionFloatInput
                pub const m_flRollScale: usize = 0x2430; // CParticleCollectionFloatInput
                pub const m_nAlpha2Field: usize = 0x25A0; // ParticleAttributeIndex_t
                pub const m_vecColorScale: usize = 0x25A8; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0x2C60; // ParticleColorBlendType_t
                pub const m_strLightStyle: usize = 0x2C68; // CUtlString
                pub const m_flLightStyleTime: usize = 0x2C70; // CPerParticleFloatInput
            };
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
            pub const C_INIT_RemapNamedModelMeshGroupToScalar = struct {
            };
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
            pub const C_INIT_PositionWarpScalar = struct {
                pub const m_vecWarpMin: usize = 0x1E0; // Vector
                pub const m_vecWarpMax: usize = 0x1EC; // Vector
                pub const m_InputValue: usize = 0x1F8; // CPerParticleFloatInput
                pub const m_flPrevPosScale: usize = 0x368; // float32
                pub const m_nScaleControlPointNumber: usize = 0x36C; // int32
                pub const m_nControlPointNumber: usize = 0x370; // int32
            };
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
            pub const C_OP_ForceControlPointStub = struct {
                pub const m_ControlPoint: usize = 0x1E0; // int32
            };
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
            pub const C_OP_VectorNoise = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_vecOutputMin: usize = 0x1DC; // Vector
                pub const m_vecOutputMax: usize = 0x1E8; // Vector
                pub const m_fl4NoiseScale: usize = 0x1F4; // float32
                pub const m_bAdditive: usize = 0x1F8; // bool
                pub const m_bOffset: usize = 0x1F9; // bool
                pub const m_flNoiseAnimationTimeScale: usize = 0x1FC; // float32
            };
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
            pub const C_OP_RemapParticleCountToScalar = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_nInputMin: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_nInputMax: usize = 0x350; // CParticleCollectionFloatInput
                pub const m_flOutputMin: usize = 0x4C0; // CParticleCollectionFloatInput
                pub const m_flOutputMax: usize = 0x630; // CParticleCollectionFloatInput
                pub const m_bActiveRange: usize = 0x7A0; // bool
                pub const m_nSetMethod: usize = 0x7A4; // ParticleSetMethod_t
            };
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
            pub const C_INIT_QuantizeFloat = struct {
                pub const m_InputValue: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x350; // ParticleAttributeIndex_t
            };
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
            pub const C_OP_SetToCP = struct {
                pub const m_nControlPointNumber: usize = 0x1D8; // int32
                pub const m_vecOffset: usize = 0x1DC; // Vector
                pub const m_bOffsetLocal: usize = 0x1E8; // bool
            };
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // PARTICLE_FAN_TYPE_ROTOR_WASH
            // PARTICLE_FAN_TYPE_RADIAL
            pub const ParticleControlPointDriver_t = struct {
                pub const m_iControlPoint: usize = 0x0; // ParticleParamID_t
                pub const m_iAttachType: usize = 0x10; // ParticleAttachment_t
                pub const m_attachmentName: usize = 0x18; // CUtlString
                pub const m_vecOffset: usize = 0x20; // Vector
                pub const m_angOffset: usize = 0x2C; // QAngle
                pub const m_entityName: usize = 0x38; // CUtlString
            };
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
            pub const C_OP_SetControlPointToCPVelocity = struct {
                pub const m_nCPInput: usize = 0x1E0; // int32
                pub const m_nCPOutputVel: usize = 0x1E4; // int32
                pub const m_bNormalize: usize = 0x1E8; // bool
                pub const m_nCPOutputMag: usize = 0x1EC; // int32
                pub const m_nCPField: usize = 0x1F0; // int32
                pub const m_vecComparisonVelocity: usize = 0x1F8; // CParticleCollectionVecInput
            };
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
            pub const RenderProjectedMaterial_t = struct {
                pub const m_hMaterial: usize = 0x0; // CStrongHandle<InfoForResourceTypeIMaterial2>
            };
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
            pub const C_INIT_SetFloatAttributeToVectorExpression = struct {
                pub const m_nExpression: usize = 0x1E0; // VectorFloatExpressionType_t
                pub const m_vInput1: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x8A0; // CPerParticleVecInput
                pub const m_flOutputRemap: usize = 0xF58; // CParticleRemapFloatInput
                pub const m_nOutputField: usize = 0x10C8; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x10CC; // ParticleSetMethod_t
            };
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
            pub const C_INIT_ModelCull = struct {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_bBoundBox: usize = 0x1E4; // bool
                pub const m_bCullOutside: usize = 0x1E5; // bool
                pub const m_bUseBones: usize = 0x1E6; // bool
                pub const m_HitboxSetName: usize = 0x1E7; // char[128]
            };
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
            pub const C_OP_PercentageBetweenTransformLerpCPs = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1DC; // float32
                pub const m_flInputMax: usize = 0x1E0; // float32
                pub const m_TransformStart: usize = 0x1E8; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x250; // CParticleTransformInput
                pub const m_nOutputStartCP: usize = 0x2B8; // int32
                pub const m_nOutputStartField: usize = 0x2BC; // int32
                pub const m_nOutputEndCP: usize = 0x2C0; // int32
                pub const m_nOutputEndField: usize = 0x2C4; // int32
                pub const m_nSetMethod: usize = 0x2C8; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x2CC; // bool
                pub const m_bRadialCheck: usize = 0x2CD; // bool
            };
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
            pub const C_OP_SetPerChildControlPoint = struct {
                pub const m_nChildGroupID: usize = 0x1D8; // int32
                pub const m_nFirstControlPoint: usize = 0x1DC; // int32
                pub const m_nNumControlPoints: usize = 0x1E0; // int32
                pub const m_nParticleIncrement: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_nFirstSourcePoint: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_bSetOrientation: usize = 0x4C8; // bool
                pub const m_nOrientationField: usize = 0x4CC; // ParticleAttributeIndex_t
                pub const m_bNumBasedOnParticleCount: usize = 0x4D0; // bool
            };
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
            pub const C_OP_SetAttributeToScalarExpression = struct {
                pub const m_nExpression: usize = 0x1D8; // ScalarExpressionType_t
                pub const m_flInput1: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flInput2: usize = 0x350; // CPerParticleFloatInput
                pub const m_flOutputRemap: usize = 0x4C0; // CParticleRemapFloatInput
                pub const m_nOutputField: usize = 0x630; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x634; // ParticleSetMethod_t
            };
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
            pub const C_OP_RenderMaterialProxy = struct {
                pub const m_nMaterialControlPoint: usize = 0x228; // int32
                pub const m_nProxyType: usize = 0x22C; // MaterialProxyType_t
                pub const m_MaterialVars: usize = 0x230; // CUtlVector<MaterialVariable_t>
                pub const m_hOverrideMaterial: usize = 0x248; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_flMaterialOverrideEnabled: usize = 0x250; // CParticleCollectionFloatInput
                pub const m_vecColorScale: usize = 0x3C0; // CParticleCollectionVecInput
                pub const m_flAlpha: usize = 0xA78; // CPerParticleFloatInput
                pub const m_nColorBlendType: usize = 0xBE8; // ParticleColorBlendType_t
            };
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
            pub const FloatInputMaterialVariable_t = struct {
                pub const m_strVariable: usize = 0x0; // CUtlString
                pub const m_flInput: usize = 0x8; // CParticleCollectionFloatInput
            };
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
            pub const C_OP_RampScalarLinear = struct {
                pub const m_RateMin: usize = 0x1D8; // float32
                pub const m_RateMax: usize = 0x1DC; // float32
                pub const m_flStartTime_min: usize = 0x1E0; // float32
                pub const m_flStartTime_max: usize = 0x1E4; // float32
                pub const m_flEndTime_min: usize = 0x1E8; // float32
                pub const m_flEndTime_max: usize = 0x1EC; // float32
                pub const m_nField: usize = 0x210; // ParticleAttributeIndex_t
                pub const m_bProportionalOp: usize = 0x214; // bool
            };
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
            pub const C_OP_RotateVector = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_vecRotAxisMin: usize = 0x1DC; // Vector
                pub const m_vecRotAxisMax: usize = 0x1E8; // Vector
                pub const m_flRotRateMin: usize = 0x1F4; // float32
                pub const m_flRotRateMax: usize = 0x1F8; // float32
                pub const m_bNormalize: usize = 0x1FC; // bool
                pub const m_flScale: usize = 0x200; // CPerParticleFloatInput
            };
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
            pub const C_INIT_InitVecCollection = struct {
                pub const m_InputValue: usize = 0x1E0; // CParticleCollectionVecInput
                pub const m_nOutputField: usize = 0x898; // ParticleAttributeIndex_t
            };
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
            pub const C_INIT_RemapParticleCountToNamedModelMeshGroupScalar = struct {
            };
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
            pub const C_INIT_SequenceFromCP = struct {
                pub const m_bKillUnused: usize = 0x1E0; // bool
                pub const m_bRadiusScale: usize = 0x1E1; // bool
                pub const m_nCP: usize = 0x1E4; // int32
                pub const m_vecOffset: usize = 0x1E8; // Vector
            };
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
            pub const C_OP_CPOffsetToPercentageBetweenCPs = struct {
                pub const m_flInputMin: usize = 0x1D8; // float32
                pub const m_flInputMax: usize = 0x1DC; // float32
                pub const m_flInputBias: usize = 0x1E0; // float32
                pub const m_nStartCP: usize = 0x1E4; // int32
                pub const m_nEndCP: usize = 0x1E8; // int32
                pub const m_nOffsetCP: usize = 0x1EC; // int32
                pub const m_nOuputCP: usize = 0x1F0; // int32
                pub const m_nInputCP: usize = 0x1F4; // int32
                pub const m_bRadialCheck: usize = 0x1F8; // bool
                pub const m_bScaleOffset: usize = 0x1F9; // bool
                pub const m_vecOffset: usize = 0x1FC; // Vector
            };
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
            pub const C_OP_LerpEndCapScalar = struct {
                pub const m_nFieldOutput: usize = 0x1D8; // ParticleAttributeIndex_t
                pub const m_flOutput: usize = 0x1DC; // float32
                pub const m_flLerpTime: usize = 0x1E0; // float32
            };
        };
    };
};
