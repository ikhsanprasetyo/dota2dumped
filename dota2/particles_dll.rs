// Generated using https://github.com/ikhsanprasetyo/Source2Dumped
// 2026-03-30 05:23:54.809776700 UTC

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod cs2_dumper {
    pub mod schemas {
        // Module: particles.dll
        // Class count: 496
        // Enum count: 76
        pub mod particles_dll {
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
            // Member count: 7
            #[repr(u32)]
            pub enum Detail2Combo_t {
                DETAIL_2_COMBO_UNINITIALIZED = u32::MAX,
                DETAIL_2_COMBO_OFF = 0x0,
                DETAIL_2_COMBO_ADD = 0x1,
                DETAIL_2_COMBO_ADD_SELF_ILLUM = 0x2,
                DETAIL_2_COMBO_MOD2X = 0x3,
                DETAIL_2_COMBO_MUL = 0x4,
                DETAIL_2_COMBO_CROSSFADE = 0x5
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum MissingParentInheritBehavior_t {
                MISSING_PARENT_DO_NOTHING = u32::MAX,
                MISSING_PARENT_KILL = 0x0,
                MISSING_PARENT_FIND_NEW = 0x1,
                MISSING_PARENT_SAME_INDEX = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleTraceMissBehavior_t {
                PARTICLE_TRACE_MISS_BEHAVIOR_NONE = 0x0,
                PARTICLE_TRACE_MISS_BEHAVIOR_KILL = 0x1,
                PARTICLE_TRACE_MISS_BEHAVIOR_TRACE_END = 0x2
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum PFuncVisualizationType_t {
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
            #[repr(u32)]
            pub enum ParticleVRHandChoiceList_t {
                PARTICLE_VRHAND_LEFT = 0x0,
                PARTICLE_VRHAND_RIGHT = 0x1,
                PARTICLE_VRHAND_CP = 0x2,
                PARTICLE_VRHAND_CP_OBJECT = 0x3
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ParticleReplicationMode_t {
                PARTICLE_REPLICATIONMODE_NONE = 0x0,
                PARTICLE_REPLICATIONMODE_REPLICATE_FOR_EACH_PARENT_PARTICLE = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleEntityPos_t {
                PARTICLE_ABS_ORIGIN = 0x0,
                PARTICLE_WORLDSPACE_CENTER = 0x1,
                PARTICLE_EYES = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleFanType_t {
                PARTICLE_FAN_TYPE_FAN = 0x0,
                PARTICLE_FAN_TYPE_ROTOR_WASH = 0x1,
                PARTICLE_FAN_TYPE_RADIAL = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum PetGroundType_t {
                PET_GROUND_NONE = 0x0,
                PET_GROUND_GRID = 0x1,
                PET_GROUND_PLANE = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum InheritableBoolType_t {
                INHERITABLE_BOOL_INHERIT = 0x0,
                INHERITABLE_BOOL_FALSE = 0x1,
                INHERITABLE_BOOL_TRUE = 0x2
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum ParticlePostProcessPriorityGroup_t {
                PARTICLE_POST_PROCESS_PRIORITY_LEVEL_VOLUME = 0x0,
                PARTICLE_POST_PROCESS_PRIORITY_LEVEL_OVERRIDE = 0x1,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_EFFECT = 0x2,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_LOW = 0x3,
                PARTICLE_POST_PROCESS_PRIORITY_GAMEPLAY_STATE_HIGH = 0x4,
                PARTICLE_POST_PROCESS_PRIORITY_GLOBAL_UI = 0x5
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum ParticleCollisionGroup_t {
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
            #[repr(u32)]
            pub enum DetailCombo_t {
                DETAIL_COMBO_OFF = 0x0,
                DETAIL_COMBO_ADD = 0x1,
                DETAIL_COMBO_ADD_SELF_ILLUM = 0x2,
                DETAIL_COMBO_MOD2X = 0x3
            }
            // Alignment: 4
            // Member count: 12
            #[repr(u32)]
            pub enum ScalarExpressionType_t {
                SCALAR_EXPRESSION_UNINITIALIZED = u32::MAX,
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
            #[repr(u32)]
            pub enum SpriteCardPerParticleScale_t {
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
            #[repr(u32)]
            pub enum BlurFilterType_t {
                BLURFILTER_GAUSSIAN = 0x0,
                BLURFILTER_BOX = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum StandardLightingAttenuationStyle_t {
                LIGHT_STYLE_OLD = 0x0,
                LIGHT_STYLE_NEW = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleParentSetMode_t {
                PARTICLE_SET_PARENT_NO = 0x0,
                PARTICLE_SET_PARENT_IMMEDIATE = 0x1,
                PARTICLE_SET_PARENT_ROOT = 0x2
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum ParticleLightingQuality_t {
                PARTICLE_LIGHTING_PER_PARTICLE = 0x0,
                PARTICLE_LIGHTING_PER_VERTEX = 0x1,
                PARTICLE_LIGHTING_PER_PIXEL = u32::MAX,
                PARTICLE_LIGHTING_OVERRIDE_POSITION = 0x2,
                PARTICLE_LIGHTING_OVERRIDE_COLOR = 0x3,
                PARTICLE_LIGHTING_ADD_EXTRA_LIGHT = 0x4
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ParticleVolumetricSmokeCreationType_t {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_CONTINUOUS = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_IMPULSE = 0x1
            }
            // Alignment: 4
            // Member count: 8
            #[repr(u32)]
            pub enum SetStatisticExpressionType_t {
                SET_EXPRESSION_UNINITIALIZED = u32::MAX,
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
            #[repr(u32)]
            pub enum EventTypeSelection_t {
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
            #[repr(u32)]
            pub enum ParticleMassMode_t {
                PARTICLE_MASSMODE_RADIUS_CUBED = 0x0,
                PARTICLE_MASSMODE_RADIUS_SQUARED = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ParticleHitboxBiasType_t {
                PARTICLE_HITBOX_BIAS_ENTITY = 0x0,
                PARTICLE_HITBOX_BIAS_HITBOX = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum ParticleControlPointAxis_t {
                PARTICLE_CP_AXIS_X = 0x0,
                PARTICLE_CP_AXIS_Y = 0x1,
                PARTICLE_CP_AXIS_Z = 0x2,
                PARTICLE_CP_AXIS_NEGATIVE_X = 0x3,
                PARTICLE_CP_AXIS_NEGATIVE_Y = 0x4,
                PARTICLE_CP_AXIS_NEGATIVE_Z = 0x5
            }
            // Alignment: 4
            // Member count: 12
            #[repr(u32)]
            pub enum ParticlePinDistance_t {
                PARTICLE_PIN_DISTANCE_NONE = u32::MAX,
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
            #[repr(u32)]
            pub enum VectorFloatExpressionType_t {
                VECTOR_FLOAT_EXPRESSION_UNINITIALIZED = u32::MAX,
                VECTOR_FLOAT_EXPRESSION_DOTPRODUCT = 0x0,
                VECTOR_FLOAT_EXPRESSION_DISTANCE = 0x1,
                VECTOR_FLOAT_EXPRESSION_DISTANCESQR = 0x2,
                VECTOR_FLOAT_EXPRESSION_INPUT1_LENGTH = 0x3,
                VECTOR_FLOAT_EXPRESSION_INPUT1_LENGTHSQR = 0x4,
                VECTOR_FLOAT_EXPRESSION_INPUT1_NOISE = 0x5
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleFogType_t {
                PARTICLE_FOG_GAME_DEFAULT = 0x0,
                PARTICLE_FOG_ENABLED = 0x1,
                PARTICLE_FOG_DISABLED = 0x2
            }
            // Alignment: 4
            // Member count: 10
            #[repr(u32)]
            pub enum VectorExpressionType_t {
                VECTOR_EXPRESSION_UNINITIALIZED = u32::MAX,
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
            #[repr(u32)]
            pub enum ParticleMultiSegmentInputSelection_t {
                PARTICLE_MULTISEGMENT_SELECTION_FLOAT = 0x0,
                PARTICLE_MULTISEGMENT_SELECTION_STRING = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleRotationLockType_t {
                PARTICLE_ROTATION_LOCK_NONE = 0x0,
                PARTICLE_ROTATION_LOCK_ROTATIONS = 0x1,
                PARTICLE_ROTATION_LOCK_NORMAL = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum HitboxLerpType_t {
                HITBOX_LERP_LIFETIME = 0x0,
                HITBOX_LERP_CONSTANT = 0x1
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum ParticleAttrBoxFlags_t {
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
            #[repr(u32)]
            pub enum ParticleTopology_t {
                PARTICLE_TOPOLOGY_POINTS = 0x0,
                PARTICLE_TOPOLOGY_LINES = 0x1,
                PARTICLE_TOPOLOGY_TRIS = 0x2,
                PARTICLE_TOPOLOGY_QUADS = 0x3,
                PARTICLE_TOPOLOGY_CUBES = 0x4
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleLightBehaviorChoiceList_t {
                PARTICLE_LIGHT_BEHAVIOR_FOLLOW_DIRECTION = 0x0,
                PARTICLE_LIGHT_BEHAVIOR_ROPE = 0x1,
                PARTICLE_LIGHT_BEHAVIOR_TRAILS = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ModelHitboxType_t {
                MODEL_HITBOX_TYPE_STANDARD = 0x0,
                MODEL_HITBOX_TYPE_RAW_BONES = 0x1,
                MODEL_HITBOX_TYPE_RENDERBOUNDS = 0x2,
                MODEL_HITBOX_TYPE_SNAPSHOT = 0x3
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleMultiSegmentCountSelection_t {
                PARTICLE_MULTISEGMENT_SEG_COUNT_7 = 0x7,
                PARTICLE_MULTISEGMENT_SEG_COUNT_14 = 0xE,
                PARTICLE_MULTISEGMENT_SEG_COUNT_16 = 0x10
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleOrientationType_t {
                PARTICLE_ORIENTATION_NONE = 0x0,
                PARTICLE_ORIENTATION_VELOCITY = 0x1,
                PARTICLE_ORIENTATION_NORMAL = 0x2,
                PARTICLE_ORIENTATION_ROTATION = 0x4
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleTraceSet_t {
                PARTICLE_TRACE_SET_ALL = 0x0,
                PARTICLE_TRACE_SET_STATIC = 0x1,
                PARTICLE_TRACE_SET_STATIC_AND_KEYFRAMED = 0x2,
                PARTICLE_TRACE_SET_DYNAMIC = 0x3
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum ParticleTextureLayerBlendType_t {
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
            #[repr(u32)]
            pub enum ParticleSelection_t {
                PARTICLE_SELECTION_FIRST = 0x0,
                PARTICLE_SELECTION_LAST = 0x1,
                PARTICLE_SELECTION_NUMBER = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleToolsState_t {
                PARTICLE_TOOLS_STATE_ALWAYS_ON = u32::MAX,
                PARTICLE_TOOLS_STATE_TOOLS_ONLY = 0x0,
                PARTICLE_TOOLS_STATE_GAME_ONLY = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum SnapshotIndexType_t {
                SNAPSHOT_INDEX_INCREMENT = 0x0,
                SNAPSHOT_INDEX_DIRECT = 0x1
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum ParticleOutputBlendMode_t {
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
            #[repr(u32)]
            pub enum ParticleLightnintBranchBehavior_t {
                PARTICLE_LIGHTNING_BRANCH_CURRENT_DIR = 0x0,
                PARTICLE_LIGHTNING_BRANCH_ENDPOINT_DIR = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum MaterialProxyType_t {
                MATERIAL_PROXY_STATUS_EFFECT = 0x0,
                MATERIAL_PROXY_TINT = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleDepthFeatheringMode_t {
                PARTICLE_DEPTH_FEATHERING_OFF = 0x0,
                PARTICLE_DEPTH_FEATHERING_ON_OPTIONAL = 0x1,
                PARTICLE_DEPTH_FEATHERING_ON_REQUIRED = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ParticleLightUnitChoiceList_t {
                PARTICLE_LIGHT_UNIT_CANDELAS = 0x0,
                PARTICLE_LIGHT_UNIT_LUMENS = 0x1
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleMultiSegmentSpecialCharacter_t {
                PARTICLE_MULTISEGMENT_SPECIAL_NONE = u32::MAX,
                PARTICLE_MULTISEGMENT_SPECIAL_DECIMAL = 0x0,
                PARTICLE_MULTISEGMENT_SPECIAL_COLON = 0x1,
                PARTICLE_MULTISEGMENT_SPECIAL_DEGREES = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleFalloffFunction_t {
                PARTICLE_FALLOFF_CONSTANT = 0x0,
                PARTICLE_FALLOFF_LINEAR = 0x1,
                PARTICLE_FALLOFF_EXPONENTIAL = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleSequenceCropOverride_t {
                PARTICLE_SEQUENCE_CROP_OVERRIDE_DEFAULT = u32::MAX,
                PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_OFF = 0x0,
                PARTICLE_SEQUENCE_CROP_OVERRIDE_FORCE_ON = 0x1
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleDetailLevel_t {
                PARTICLEDETAIL_LOW = 0x0,
                PARTICLEDETAIL_MEDIUM = 0x1,
                PARTICLEDETAIL_HIGH = 0x2,
                PARTICLEDETAIL_ULTRA = 0x3
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum BBoxVolumeType_t {
                BBOX_VOLUME = 0x0,
                BBOX_DIMENSIONS = 0x1,
                BBOX_MINS_MAXS = 0x2,
                BBOX_RADIUS = 0x3
            }
            // Alignment: 4
            // Member count: 12
            #[repr(u32)]
            pub enum SpriteCardTextureType_t {
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
            #[repr(u32)]
            pub enum ParticleAlphaReferenceType_t {
                PARTICLE_ALPHA_REFERENCE_ALPHA_ALPHA = 0x0,
                PARTICLE_ALPHA_REFERENCE_OPAQUE_ALPHA = 0x1,
                PARTICLE_ALPHA_REFERENCE_ALPHA_OPAQUE = 0x2,
                PARTICLE_ALPHA_REFERENCE_OPAQUE_OPAQUE = 0x3
            }
            // Alignment: 4
            // Member count: 15
            #[repr(u32)]
            pub enum SpriteCardTextureChannel_t {
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
            #[repr(u32)]
            pub enum ParticleVolumetricSmokeType_t {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_EMISSION = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_SINK = 0x1,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_REPEL = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum RenderModelSubModelFieldType_t {
                SUBMODEL_AS_BODYGROUP_SUBMODEL = 0x0,
                SUBMODEL_AS_MESHGROUP_INDEX = 0x1,
                SUBMODEL_AS_MESHGROUP_MASK = 0x2,
                SUBMODEL_IGNORED_USE_MODEL_DEFAULT_MESHGROUP_MASK = 0x3
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ParticleHitboxDataSelection_t {
                PARTICLE_HITBOX_AVERAGE_SPEED = 0x0,
                PARTICLE_HITBOX_COUNT = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum ParticleOrientationChoiceList_t {
                PARTICLE_ORIENTATION_SCREEN_ALIGNED = 0x0,
                PARTICLE_ORIENTATION_SCREEN_Z_ALIGNED = 0x1,
                PARTICLE_ORIENTATION_WORLD_Z_ALIGNED = 0x2,
                PARTICLE_ORIENTATION_ALIGN_TO_PARTICLE_NORMAL = 0x3,
                PARTICLE_ORIENTATION_SCREENALIGN_TO_PARTICLE_NORMAL = 0x4,
                PARTICLE_ORIENTATION_FULL_3AXIS_ROTATION = 0x5
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ParticleCollisionMode_t {
                COLLISION_MODE_PER_PARTICLE_TRACE = 0x3,
                COLLISION_MODE_USE_NEAREST_TRACE = 0x2,
                COLLISION_MODE_PER_FRAME_PLANESET = 0x1,
                COLLISION_MODE_INITIAL_TRACE_DOWN = 0x0,
                COLLISION_MODE_DISABLED = u32::MAX
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ParticleSortingChoiceList_t {
                PARTICLE_SORTING_NEAREST = 0x0,
                PARTICLE_SORTING_CREATION_TIME = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleEndcapMode_t {
                PARTICLE_ENDCAP_ALWAYS_ON = u32::MAX,
                PARTICLE_ENDCAP_ENDCAP_OFF = 0x0,
                PARTICLE_ENDCAP_ENDCAP_ON = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ClosestPointTestType_t {
                PARTICLE_CLOSEST_TYPE_BOX = 0x0,
                PARTICLE_CLOSEST_TYPE_CAPSULE = 0x1,
                PARTICLE_CLOSEST_TYPE_HYBRID = 0x2
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum ParticleImpulseType_t {
                IMPULSE_TYPE_NONE = 0x0,
                IMPULSE_TYPE_GENERIC = 0x1,
                IMPULSE_TYPE_ROPE = 0x2,
                IMPULSE_TYPE_EXPLOSION = 0x4,
                IMPULSE_TYPE_EXPLOSION_UNDERWATER = 0x8,
                IMPULSE_TYPE_PARTICLE_SYSTEM = 0x10
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleLiquidContents_t {
                PARTICLE_LIQUID_NONE = 0x0,
                PARTICLE_LIQUID_OIL = 0x1,
                PARTICLE_LIQUID_WATER = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum SpriteCardShaderType_t {
                SPRITECARD_SHADER_BASE = 0x0,
                SPRITECARD_SHADER_CUSTOM = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ParticleOmni2LightTypeChoiceList_t {
                PARTICLE_OMNI2_LIGHT_TYPE_POINT = 0x0,
                PARTICLE_OMNI2_LIGHT_TYPE_SPHERE = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleLightFogLightingMode_t {
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_NONE = 0x0,
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC = 0x2,
                PARTICLE_LIGHT_FOG_LIGHTING_MODE_DYNAMIC_NOSHADOWS = 0x4
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleLightTypeChoiceList_t {
                PARTICLE_LIGHT_TYPE_POINT = 0x0,
                PARTICLE_LIGHT_TYPE_SPOT = 0x1,
                PARTICLE_LIGHT_TYPE_FX = 0x2,
                PARTICLE_LIGHT_TYPE_CAPSULE = 0x3
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleOrientationSetMode_t {
                PARTICLE_ORIENTATION_SET_NONE = u32::MAX,
                PARTICLE_ORIENTATION_SET_FROM_VELOCITY = 0x0,
                PARTICLE_ORIENTATION_SET_FROM_NORMAL = 0x1,
                PARTICLE_ORIENTATION_SET_FROM_ROTATIONS = 0x2
            }
            // Alignment: 8
            // Member count: 10
            #[repr(u64)]
            pub enum ParticleCollisionMask_t {
                PARTICLE_MASK_ALL = u64::MAX,
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
            #[repr(u32)]
            pub enum TextureRepetitionMode_t {
                TEXTURE_REPETITION_PARTICLE = 0x0,
                TEXTURE_REPETITION_PATH = 0x1
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_WaitForCursorsWithTag {
            }
            // Parent: None
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 942
            pub mod CPulse_ResumePoint {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 49
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_BaseState {
            }
            // Parent: None
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            // Parent: None
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 49
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            // Field count: 942
            pub mod SignatureOutflow_Continue {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
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
            // Field count: 49
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 942
            pub mod CParticleCollectionBindingInstance {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
            }
            // Parent: None
            // Field count: 49
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            pub mod CPulseCell_Outflow_CycleRandom {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Step_PublicOutput {
            }
            // Parent: None
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 47
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            pub mod IParticleCollection {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            pub mod ParticleAttributeIndex_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            pub mod C_OP_RemapGravityToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_Decay {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderDeferredLight {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapSpeedtoCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapTransformToVelocity {
            }
            // Parent: None
            // Field count: 49
            pub mod CollisionGroupContext_t {
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            pub mod CParticleFunctionPreEmission {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_FadeOutSimple {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SpringToVectorConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderRopes {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_StatusEffectCitadel {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderSound {
            }
            // Parent: None
            // Field count: 49
            pub mod CParticleVisibilityInputs {
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointsToParticle {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapCPVelocityToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PointVectorAtNextParticle {
            }
            // Parent: None
            // Field count: 47
            pub mod ParticlePreviewBodyGroup_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_OscillateScalarSimple {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_INIT_StatusEffect {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RtEnvCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ConstrainDistance {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitialVelocityNoise {
            }
            // Parent: None
            // Field count: 54
            pub mod ParticleChildrenInfo_t {
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
                pub const : usize = 0xBC2B8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0x14C3D0; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x65006C; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x750064; // 
                pub const : usize = 0x43002E; // 
                pub const : usize = 0x69004C; // 
                pub const : usize = 0x5C0073; // 
                pub const : usize = 0x650062; // 
                pub const : usize = 0x36006E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x50; // 
                pub const : usize = 0x0; // 
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
            pub mod C_OP_RemapScalarOnceTimed {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomNamedModelSequence {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PlaneCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_VelocityRandom {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ModelDampenMovement {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_TwistAroundAxis {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_TeleportBeam {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RemapExternalWindToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod CBaseRendererSource2 {
            }
            // Parent: None
            // Field count: 0
            pub mod CSpinUpdateBase {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_OrientTo2dDirection {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RemapDotProductToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapParticleCountToNamedModelElementScalar {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RenderTrails {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointPositionToTimeOfDayValue {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DecayMaintainCount {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomModelSequence {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ExternalGameImpulseForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapAverageHitboxSpeedtoCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomAlpha {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_NormalizeVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_FadeInSimple {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RepeatedTriggerChildGroup {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapVelocityToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_SetHitboxToClosest {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RingWave {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomTrailLength {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DistanceBetweenTransforms {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DecayOffscreen {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateSequentialPath {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_EndCapTimedDecay {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RemapDistanceToLineSegmentBase {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ContinuousEmitter {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_OscillateVectorSimple {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_SequenceLifeTime {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_MoveBetweenPoints {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetUserEvent {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_QuantizeFloat {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_BasicMovement {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomNamedModelElement {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitFromParentKilled {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_Callback {
            }
            // Parent: None
            // Field count: 45
            pub mod CParticleFunction {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            // Parent: particles
            // Field count: 0
            pub mod C_OP_GlobalLight {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_OffsetVectorToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetPerChildControlPointFromAttribute {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetParentControlPointsToChildCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_BoxConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreatePhyllotaxis {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_AttractToControlPoint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomLifeTime {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapParticleCountToNamedModelSequenceScalar {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_VelocityRadialRandom {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomRadius {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_Orient2DRelToCP {
            }
            // Parent: None
            // Field count: 47
            pub mod TextureControls_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 49
            pub mod ControlPointReference_t {
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetControlPointToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LightningSnapshotGenerator {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RemapNamedModelMeshGroupOnceTimed {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapQAnglesToRotation {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_PositionWarp {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointFieldToScalarExpression {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_CreateParticleSystemRenderer {
            }
            // Parent: None
            // Field count: 942
            pub mod CParticleFunctionForce {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomVectorComponent {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_InheritFromParentParticles {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_SetVectorAttributeToVectorExpression {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RemapTransformVisibilityToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DirectionBetweenVecsToVec {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MovementLoopInsideSphere {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderSimpleModelCollection {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_QuantizeCPComponent {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_PlayEndCapWhenFinished {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitFloatCollection {
            }
            // Parent: None
            // Field count: 54
            pub mod CPathParameters {
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
                pub const : usize = 0xBC2B8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0x14C3D0; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x65006C; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x750064; // 
                pub const : usize = 0x43002E; // 
                pub const : usize = 0x69004C; // 
                pub const : usize = 0x5C0073; // 
                pub const : usize = 0x650062; // 
                pub const : usize = 0x36006E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x50; // 
                pub const : usize = 0x0; // 
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
            pub mod C_OP_RemapScalarEndCap {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_CreateFromPlaneCache {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LazyCullCompareFloat {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ControlPointToRadialScreenSpace {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SpinUpdate {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_NormalOffset {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RemapDistanceToLineSegmentToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderAsModels {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreationNoise {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_Spin {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_GameLiquidSpill {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_InstantaneousEmitter {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ConstrainLineLength {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_LifespanFromVelocity {
            }
            // Parent: None
            // Field count: 0
            pub mod CBaseTrailRenderer {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_INIT_VelocityFromCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointOrientation {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MovementSkinnedPositionFromCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MultiSegmentDisplaySnapshotGenerator {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_OscillateVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PositionLock {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderVRHapticEvent {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetControlPointToImpactPoint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_InterpolateRadius {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ReinitializeScalarEndCap {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_TurbulenceForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapNamedModelElementOnceTimed {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetControlPointToPlayer {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_EndCapTimedFreeze {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RenderGpuImplicit {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetRandomControlPointPosition {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderVolumetricEmitter {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapTransformVisibilityToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapControlPointDirectionToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ScreenSpacePositionOfTarget {
            }
            // Parent: particles
            // Field count: 942
            pub mod CParticleFunctionOperator {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DragRelativeToPlane {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetCPtoVector {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_INIT_RandomYaw {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SnapshotRigidSkinToBones {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetSingleControlPointPosition {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_DistanceToNeighborCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapCPtoScalar {
            }
            // Parent: None
            // Field count: 942
            pub mod CParticleFunctionRenderer {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapNamedModelMeshGroupEndCap {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PercentageBetweenTransformsVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderScreenVelocityRotate {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_UpdateLightSource {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateWithinBox {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ChooseRandomChildrenInGroup {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ControlpointLight {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_VectorFieldSnapshot {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_CylindricalDistanceToTransform {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_PositionPlaceOnGround {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderPostProcessing {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_WorldTraceConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderBlobs {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_OscillateScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_FadeOut {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_WaterImpulseRenderer {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomSequence {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RampScalarSplineSimple {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_DistanceCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_CollideWithParentParticles {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitFromVectorFieldSnapshot {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetVectorAttributeToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_AddVectorToVector {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_RemapInitialVisibilityScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapTransformOrientationToYaw {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderStatusEffect {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RandomForce {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RemapParticleCountOnScalarEndCap {
            }
            // Parent: None
            // Field count: 47
            pub mod ParticlePreviewState_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_LocalAccelerationForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ModelCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetFloat {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapTransformToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ScreenSpaceDistanceToEdge {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapDistanceToLineSegmentToScalar {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RemapVectortoCP {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetFromCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DistanceBetweenCPsToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointToHand {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ConstrainDistanceToPath {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DistanceCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateAlongPath {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_GameDecalRenderer {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_SetControlPointsToModelParticles {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ColorInterpolateRandom {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapNamedModelSequenceToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderLights {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DecayClampCount {
            }
            // Parent: None
            // Field count: 49
            pub mod CRandomNumberGeneratorParameters {
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_ColorLitPerParticle {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderPoints {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_INIT_SetAttributeToScalarExpression {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateOnGrid {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RampCPLinearRandom {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_VelocityMatchingForce {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_INIT_RandomAlphaWindowThreshold {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateOnModelAtHeight {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_ModelSurfaceSnapshotGenerator {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RestartAfterDuration {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderClothForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapVisibilityScalar {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_CreateSequentialPathV2 {
            }
            // Parent: None
            // Field count: 47
            pub mod VecInputMaterialVariable_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_INIT_RemapInitialDirectionToTransformToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LockToSavedSequentialPathV2 {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_NormalLock {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapTransformOrientationToRotations {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_Cull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomYawFlip {
            }
            // Parent: None
            // Field count: 49
            pub mod SequenceWeightedList_t {
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_ReadFromNeighboringParticle {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderText {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_LerpToInitialPosition {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomRotation {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LerpEndCapVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_VelocityDecay {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetCPOrientationToPointAtCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LockToPointList {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MovementPlaceOnGround {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetCPOrientationToDirection {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapCrossProductOfTwoVectorsToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapTransformOrientationToRotations {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomRotationSpeed {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_InheritFromParentParticlesV2 {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomSecondSequence {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetFloatCollection {
            }
            // Parent: None
            // Field count: 49
            pub mod PointDefinition_t {
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
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x720061; // 
                pub const : usize = 0x6D006D; // 
                pub const : usize = 0x67005C; // 
                pub const : usize = 0x69006C; // 
                pub const : usize = 0x64002E; // 
                pub const : usize = 0x53005C; // 
                pub const : usize = 0x610065; // 
                pub const : usize = 0x74006F; // 
                pub const : usize = 0x690062; // 
                pub const : usize = 0x53; // 
                pub const l: usize = 0xBC16BD86; // 
                pub const : usize = 0x0; // ����3�H��$�
                pub const : usize = 0x10000A0; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointPositionToRandomActiveCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_Diffusion {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_AgeNoise {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapVectorComponentToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod CGeneralRandomRotation {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DistanceBetweenVecs {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DampenToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_CalculateVectorAttribute {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LockToBone {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapNamedModelBodyPartOnceTimed {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ScreenSpaceRotateTowardTarget {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_MovementMaintainOffset {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateWithinCapsuleTransform {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetVec {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateFromParentParticles {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CheckParticleForWater {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomNamedModelBodyPart {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderOmni2Light {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ConnectParentParticleToNearest {
            }
            // Parent: None
            // Field count: 47
            pub mod CPAssignment_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_INIT_RemapParticleCountToNamedModelBodyPartScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitSkinnedPositionFromCPSnapshot {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_LagCompensation {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_CollideWithSelf {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_Noise {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_FadeAndKillForTracers {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ColorAdjustHSL {
            }
            // Parent: None
            // Field count: 47
            pub mod CParticleMassCalculationParameters {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_SequenceFromModel {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_AlphaDecay {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapDensityGradientToVectorAttribute {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitVec {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_SetHitboxToModel {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MovementMoveAlongSkinnedCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LerpScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitialRepulsionVelocity {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ClampScalar {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetControlPointToHMD {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DifferencePreviousParticle {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointFieldFromVectorExpression {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_PercentageBetweenTransforms {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_PlaneCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapNamedModelSequenceEndCap {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitFromCPSnapshot {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderCables {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InheritVelocity {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointToWaterSurface {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_PositionOffset {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_NormalAlignToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ShapeMatchingConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetChildControlPoints {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ChladniWave {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapDirectionToCPToVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DriveCPFromGlobalSoundFloat {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_ScreenSpacePositionOfTarget {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RtEnvCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PinParticleToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapCPtoVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateParticleImpulse {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_DensityForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateInEpitrochoid {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ConstrainDistanceToUserSpecifiedPath {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetControlPointPositions {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetFloatAttributeToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MovementRotateParticleAroundAxis {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_IntraParticleForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitFloat {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateOnModel {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_InheritFromPeerSystem {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PerParticleForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomNamedModelMeshGroup {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderProjected {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_MaxVelocity {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_VelocityFromNormal {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MaintainEmitter {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_PositionOffsetToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapInitialTransformDirectionToRotation {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_FadeAndKill {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ColorInterpolate {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RampScalarSpline {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapNamedModelSequenceOnceTimed {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_SetControlPointFromObjectScale {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MaintainSequentialPath {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapNamedModelBodyPartEndCap {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_StopAfterCPDuration {
            }
            // Parent: None
            // Field count: 0
            pub mod CGeneralSpin {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LockToSavedSequentialPath {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapNamedModelElementToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ClampVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderStatusEffectCitadel {
            }
            // Parent: None
            // Field count: 45
            pub mod IParticleSystemDefinition {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x72006100; // 
                pub const : usize = 0x6D006D00; // 
                pub const : usize = 0x67005C00; // 
                pub const : usize = 0x69006C00; // 
                pub const : usize = 0x64002E00; // 
                pub const : usize = 0x72006200; // 
                pub const : usize = 0x6F006300; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x5C003400; // 
                pub const : usize = 0x30002D00; // 
                pub const : usize = 0x3A004400; // 
                pub const : usize = 0x74007300; // 
                pub const : usize = 0x64005C00; // 
                pub const : usize = 0x5C006500; // 
                pub const : usize = 0x55002D00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
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
            pub mod C_OP_WindForce {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_SetVariable {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderStandardLight {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_DistanceToTransform {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RemapControlPointOrientationToRotation {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointToCenter {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapAverageScalarValuetoCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapDotProductToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapCPtoCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointRotation {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_CurlNoiseForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_Orient2DRelToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetSimulationRate {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_FadeIn {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderScreenShake {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapBoundingVolumetoCP {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_HSVShiftToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapVectorToRotations {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_INIT_GlobalScale {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_RadiusFromCPObject {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InitialVelocityFromHitbox {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LerpVector {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointFieldToWater {
            }
            // Parent: None
            // Field count: 47
            pub mod TextureGroup_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_TimeVaryingForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetCPOrientationToGroundNormal {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SnapshotSkinToBones {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateWithinSphereTransform {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RadiusDecay {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapNamedModelBodyPartToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapScalarToVector {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_INIT_InitialSequenceFromModel {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_NoiseEmitter {
            }
            // Parent: None
            // Field count: 942
            pub mod CParticleFunctionInitializer {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SelectivelyEnableChildren {
            }
            // Parent: None
            // Field count: 47
            pub mod ModelReference_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_PlanarConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_CreateFromCPs {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LockPoints {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_CreateSpiralSphere {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_CPVelocityForce {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RemapNamedModelElementEndCap {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_ScaleVelocity {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_MoveToHitbox {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PinRopeSegmentParticleToParent {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_PointList {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LerpToOtherAttribute {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RandomColor {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetGravityToCP {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_RemapParticleCountToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_InheritFromParentParticles {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RampScalarLinearSimple {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_ChaoticAttractor {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_MovementRigidAttachToCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderFlattenGrass {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderLightBeam {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_EnableChildrenFromParentParticleCount {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_DistanceToCPInit {
            }
            // Parent: None
            // Field count: 47
            pub mod CReplicationParameters {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_EndCapDecay {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ForceBasedOnDistanceToPlane {
            }
            // Parent: ______
            // Field count: 0
            pub mod C_OP_RemapDensityToVector {
            }
            // Parent: None
            // Field count: 47
            pub mod ParticleControlPointConfiguration_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_INIT_SetRigidAttachment {
            }
            // Parent: None
            // Field count: 47
            pub mod MaterialVariable_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            // Field count: 942
            pub mod CParticleFunctionConstraint {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapSpeed {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderModels {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderClientPhysicsImpulse {
            }
            // Parent: None
            // Field count: 942
            pub mod CParticleFunctionEmitter {
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1B00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0xF00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0xCCCCCC00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80000040; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x1F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xB00; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10007208; // 
                pub const : usize = 0x1000D20B; // 
                pub const : usize = 0x100E0600; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x870A3D3F; // 
                pub const : usize = 0x1000D20A; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x59999A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0xA00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x12FF; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4EB85200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0xD555553E; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1000C20A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10010600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001F3F; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2001; // 
                pub const : usize = 0x10000A03; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10002208; // 
                pub const : usize = 0x10002209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000C20F; // 
                pub const 
                : usize = 0x700000C0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10001A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x107B4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x2F41; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003205; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x6147AEBD; // 
                pub const : usize = 0x3B; // 
                pub const : usize = 0x10080600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x10001207; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x4001F00; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10001208; // 
                pub const : usize = 0x1000120A; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x1000720C; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x23F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x32BF; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x363F; // 
                pub const : usize = 0x100AA600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10075600; // 
                pub const : usize = 0x10084600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x10055600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x1000F20B; // 
                pub const : usize = 0x8000003F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x1000F20A; // 
                pub const 
                : usize = 0x3F; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000F205; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x800000BF; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0xE3F; // 
                pub const : usize = 0x1000520F; // 
                pub const : usize = 0xBF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000320C; // 
                pub const : usize = 0x3F; // 
                pub const : usize = 0x1800; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x107E4600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x10004208; // 
                pub const : usize = 0x10004207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x3601; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10090600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x900; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000820A; // 
                pub const : usize = 0x3D; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x10003A03; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x1500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0xD0955600; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x1900; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x6147AE00; // 
                pub const : usize = 0x4D2E1C00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1000520A; // 
                pub const 
                : usize = 0xBF; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x1000520A; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10008600; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x10008208; // 
                pub const : usize = 0x10008205; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x40; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5600; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10004205; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x800; // 
                pub const : usize = 0x4001F01; // 
                pub const : usize = 0x4500; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10101A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1DBD; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x10004600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10600000; // 
                pub const : usize = 0x1000F209; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x3700; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000E208; // 
                pub const : usize = 0x10007209; // 
                pub const : usize = 0x3600; // 
                pub const : usize = 0x500; // 
                pub const : usize = 0x1645A23E; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10007205; // 
                pub const : usize = 0x2000; // 
                pub const : usize = 0x10002207; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10040600; // 
                pub const : usize = 0x100E8600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x1000320A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400103; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x323F; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x601; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x93A92A3F; // 
                pub const : usize = 0x300; // 
                pub const : usize = 0x100E4600; // 
                pub const : usize = 0x10000600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x1000720A; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20824600; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10000A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10103A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1700; // 
                pub const : usize = 0x20801A00; // 
                pub const : usize = 0x10002A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x20802A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400200; // 
                pub const : usize = 0x1000E20A; // 
                pub const : usize = 0x1A00; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x4B00; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x10004209; // 
                pub const : usize = 0x3200; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x200; // 
                pub const : usize = 0x10003A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x100FF600; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x75564E10; // 
                pub const : usize = 0x2C00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF7FF0001; // 
                pub const : usize = 0x67FF15E0; // 
                pub const : usize = 0x7F01201; // 
                pub const : usize = 0x671404C0; // 
                pub const : usize = 0x800000D8; // 
                pub const : usize = 0x7091C49; // 
                pub const : usize = 0xC7191F49; // 
                pub const : usize = 0x671F1F5C; // 
                pub const : usize = 0x6717175C; // 
                pub const : usize = 0x671D1D5C; // 
                pub const : usize = 0x701165C; // 
                pub const : usize = 0xF7161E5C; // 
                pub const : usize = 0x771E1E5C; // 
                pub const : usize = 0x71B1B5C; // 
                pub const : usize = 0xD717074C; // 
                pub const : usize = 0x970D085C; // 
                pub const : usize = 0xF7F01F5C; // 
                pub const : usize = 0x8701085C; // 
                pub const : usize = 0x17F0025C; // 
                pub const : usize = 0x8705085C; // 
                pub const : usize = 0x17F00A5C; // 
                pub const : usize = 0x37161601; // 
                pub const : usize = 0xF7140C5C; // 
                pub const : usize = 0x271400C0; // 
                pub const : usize = 0x37181D50; // 
                pub const : usize = 0xA70E18F0; // 
                pub const : usize = 0x71B0A49; // 
                pub const : usize = 0xA090B39; // 
                pub const : usize = 0x70F0A49; // 
                pub const : usize = 0x8706185C; // 
                pub const : usize = 0x7190732; // 
                pub const : usize = 0x71D0732; // 
                pub const : usize = 0xE7031E38; // 
                pub const : usize = 0x770A0A49; // 
                pub const : usize = 0xE7000E33; // 
                pub const : usize = 0xE7131F32; // 
                pub const : usize = 0xE7101032; // 
                pub const : usize = 0xB7080732; // 
                pub const : usize = 0x37FF1A01; // 
                pub const : usize = 0x27170C58; // 
                pub const : usize = 0x971B1B5C; // 
                pub const : usize = 0xB7161C4B; // 
                pub const : usize = 0x57FF045C; // 
                pub const : usize = 0x70B0F5C; // 
                pub const : usize = 0x37010F5C; // 
                pub const : usize = 0x67090E5C; // 
                pub const : usize = 0x4717175C; // 
                pub const : usize = 0xEA0E02D8; // 
                pub const : usize = 0x8000005C; // 
                pub const : usize = 0xC7FF095C; // 
                pub const : usize = 0x71B0F5C; // 
                pub const : usize = 0x709095C; // 
                pub const : usize = 0xE7000C38; // 
                pub const : usize = 0x1713115C; // 
                pub const : usize = 0xA011849; // 
                pub const : usize = 0x9702FF5C; // 
                pub const : usize = 0x97021A5B; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0xE3041E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x90319D8; // 
                pub const : usize = 0xF71900E3; // 
                pub const : usize = 0x97008F5C; // 
                pub const : usize = 0x171CFF5B; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0x5B; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0xC80E0250; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0x9700875C; // 
                pub const : usize = 0xF71C0058; // 
                pub const : usize = 0xF71C1FE2; // 
                pub const : usize = 0xF71C075B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D00D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E02E3; // 
                pub const : usize = 0x9011849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80319D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0x8A000F5C; // 
                pub const : usize = 0xC70E19F0; // 
                pub const : usize = 0x770D18E3; // 
                pub const : usize = 0x790D02D8; // 
                pub const : usize = 0x7000F49; // 
                pub const : usize = 0x9702005C; // 
                pub const : usize = 0x3000F5C; // 
                pub const : usize = 0x9703A7E2; // 
                pub const : usize = 0xE3060E5B; // 
                pub const : usize = 0x7404175C; // 
                pub const : usize = 0xFE2; // 
                pub const : usize = 0x1000FF0; // 
                pub const : usize = 0xC80E00E3; // 
                pub const : usize = 0x9031849; // 
                pub const : usize = 0xF718035C; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0x8000005B; // 
                pub const : usize = 0x97038F5B; // 
                pub const : usize = 0xD4060D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x97030158; // 
                pub const : usize = 0x97029FE2; // 
                pub const : usize = 0x9702875B; // 
                pub const : usize = 0xC3040C5C; // 
                pub const : usize = 0xE7001B5C; // 
                pub const : usize = 0x77001D5C; // 
                pub const : usize = 0xC7000D5C; // 
                pub const : usize = 0xE2; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x7000FD8; // 
                pub const : usize = 0x80119D8; // 
                pub const : usize = 0xF71902E3; // 
                pub const : usize = 0x7F02750; // 
                pub const : usize = 0x1700FF58; // 
                pub const : usize = 0xA001F5B; // 
                pub const : usize = 0xA3041A5C; // 
                pub const : usize = 0xBA000E5C; // 
                pub const : usize = 0xDA00175C; // 
                pub const : usize = 0xE7140E5B; // 
                pub const : usize = 0x70150D5C; // 
                pub const : usize = 0x670F165C; // 
                pub const : usize = 0x870F185C; // 
                pub const : usize = 0x670D0D5B; // 
                pub const : usize = 0xF70B015C; // 
                pub const : usize = 0xA7150A5C; // 
                pub const : usize = 0x77110449; // 
                pub const : usize = 0x7130049; // 
                pub const : usize = 0x37130359; // 
                pub const : usize = 0x2704045C; // 
                pub const : usize = 0x703875C; // 
                pub const : usize = 0x87000FE3; // 
                pub const : usize = 0x70F0050; // 
                pub const : usize = 0xC122BB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8F800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCB000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x127000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x14A800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x160800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x189000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10001A00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x3800; // 
                pub const : usize = 0x10008209; // 
                pub const : usize = 0x203600; // 
                pub const : usize = 0x20803A00; // 
                pub const : usize = 0x4180; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0x10001205; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x10024600; // 
                pub const : usize = 0xA1C91000; // 
                pub const : usize = 0x8F16DC83; // 
                pub const : usize = 0x1A5A16DD; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF14B0010; // 
                pub const : usize = 0x8D00; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0xB0C62000; // 
                pub const 8: usize = 0x26AE800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310F1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BC000; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x48000; // 
                pub const : usize = 0x7094F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B4800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x5047B000; // 
                pub const : usize = 0x2798800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3104F000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1AEA00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x838000; // 
                pub const : usize = 0x6FA5F000; // 
                pub const : usize = 0x440100; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x383D600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7B00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3000; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2882800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDA; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x31115000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x78; // 
                pub const : usize = 0x1B0400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x71033000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x12FC00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x4F8EB000; // 
                pub const r: usize = 0x296C800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x30B69000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BE300; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x480000; // 
                pub const : usize = 0x6F98F000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x46D400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x783780BF; // 
                pub const : usize = 0x40260000; // 
                pub const : usize = 0x2A56800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFC; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x310B5000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF; // 
                pub const : usize = 0x1BBE00; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x258000; // 
                pub const : usize = 0x70F0B000; // 
                pub const : usize = 0x2040100; // 
                pub const : usize = 0x7B000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4200; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7834D094; // 
                pub const ��: usize = 0x40DC8000; // 
                pub const : usize = 0x2B40800; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC5; // 
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapNamedModelMeshGroupToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointOrientationToCPVelocity {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_RopeSpringConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_PositionWarpScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ForceControlPointStub {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_VectorNoise {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapParticleCountToScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_QuantizeFloat {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RemapModelVolumetoCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetToCP {
            }
            // Parent: None
            // Field count: 47
            pub mod ParticleControlPointDriver_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_ParentVortices {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetControlPointToCPVelocity {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ClientPhysics {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SpinYaw {
            }
            // Parent: None
            // Field count: 949
            pub mod PointDefinitionWithTimeValues_t {
                pub const : usize = 0x0; // 
                pub const : usize = 0x208E46; // 
                pub const : usize = 0x208E46; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x5555; // 
                pub const : usize = 0x3001062; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x3000065; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x10001A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x7; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x8000000; // 
                pub const : usize = 0x9000032; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7000001; // 
                pub const : usize = 0x100042; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x107E46; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x100AA6; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100006; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x107E46; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x100556; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x7000055; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x9000032; // 
                pub const : usize = 0xBF000000; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x10001A; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106000; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x107E46; // 
                pub const : usize = 0x5000036; // 
                pub const : usize = 0x100046; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x1000015; // 
                pub const : usize = 0x8000000; // 
                pub const : usize = 0x9000032; // 
                pub const : usize = 0xB; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8000000; // 
                pub const : usize = 0x9000032; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x700004F; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x414EB852; // 
                pub const : usize = 0xA000038; // 
                pub const : usize = 0x3ED55555; // 
                pub const : usize = 0x100072; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x9000037; // 
                pub const : usize = 0xA; // 
                pub const : usize = 0x3F372474; // 
                pub const : usize = 0x10000A; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x7000018; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x1000015; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x106000; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x100E46; // 
                pub const : usize = 0x7000020; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xA000038; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100032; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x9000037; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100206; // 
                pub const : usize = 0x3E000000; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x7000018; // 
                pub const : usize = 0x10000A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x1000015; // 
                pub const : usize = 0x100012; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x100012; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x10100A; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0xA; // 
                pub const : usize = 0x300001F; // 
                pub const : usize = 0x9000045; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10101A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100906; // 
                pub const : usize = 0x100006; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x8010003A; // 
                pub const : usize = 0x10000A; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100806; // 
                pub const : usize = 0x100032; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3B4D2E1C; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x100EA6; // 
                pub const : usize = 0x1000C2; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x80100006; // 
                pub const : usize = 0x10001A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x1000015; // 
                pub const : usize = 0x100006; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x300004C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100AA6; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x3000006; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x100906; // 
                pub const : usize = 0x100072; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xA000010; // 
                pub const : usize = 0x3DE978D5; // 
                pub const : usize = 0x3000006; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x7000001; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x100042; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xA000032; // 
                pub const : usize = 0x100AE6; // 
                pub const : usize = 0x3000006; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x80100AA6; // 
                pub const : usize = 0x100AA6; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xA000000; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x100E46; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0xBF800000; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x300001F; // 
                pub const : usize = 0x1000012; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100042; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x10100A; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x300001F; // 
                pub const : usize = 0x9000045; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10101A; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100E46; // 
                pub const : usize = 0x100AA6; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8010003A; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x100042; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x1000D2; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0xB000032; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x80100386; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x500002F; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x100072; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0xBD6147AE; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100E06; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x9000032; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5000056; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0xA000010; // 
                pub const : usize = 0x3DE978D5; // 
                pub const : usize = 0x100072; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100042; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x500002F; // 
                pub const : usize = 0x100046; // 
                pub const : usize = 0x100032; // 
                pub const : usize = 0x6; // 
                pub const : usize = 0xBD6147AE; // 
                pub const : usize = 0x3B4D2E1C; // 
                pub const : usize = 0x100106; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x7000000; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x5000056; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x1000015; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100A26; // 
                pub const : usize = 0x100012; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x107B46; // 
                pub const : usize = 0x5000036; // 
                pub const : usize = 0x1005D6; // 
                pub const : usize = 0x10000A; // 
                pub const : usize = 0x1000F2; // 
                pub const : usize = 0x1000015; // 
                pub const : usize = 0x5; // 
                pub const : usize = 0x106000; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x90901A; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20803A; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x414EB852; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xF000032; // 
                pub const : usize = 0x100032; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100AE6; // 
                pub const : usize = 0xBF000000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x100406; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x304001F; // 
                pub const : usize = 0x107E46; // 
                pub const : usize = 0x9000045; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100012; // 
                pub const : usize = 0x100012; // 
                pub const : usize = 0x9; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10001A; // 
                pub const : usize = 0x3000006; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100556; // 
                pub const : usize = 0x3000006; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0xA000010; // 
                pub const : usize = 0x3DE978D5; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0xA000032; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x3E991687; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0xA000032; // 
                pub const : usize = 0x100756; // 
                pub const : usize = 0x3000006; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5000036; // 
                pub const : usize = 0xC; // 
                pub const : usize = 0x7000001; // 
                pub const : usize = 0x100062; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0xA000032; // 
                pub const : usize = 0x100556; // 
                pub const : usize = 0x3000006; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x100E46; // 
                pub const : usize = 0x8000000; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x100E46; // 
                pub const : usize = 0x4001; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x3F000000; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1000002; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xA000032; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x10103A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x8020803A; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x8010002A; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10103A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x8000036; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0xA000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x208246; // 
                pub const : usize = 0x10002A; // 
                pub const : usize = 0x3F800000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x8000000; // 
                pub const : usize = 0x7000010; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10000A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10003A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8020803A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0xA000032; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x8; // 
                pub const : usize = 0x1B; // 
                pub const : usize = 0x50; // 
                pub const : usize = 0x1B80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x618; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE003FF87; // 
                pub const : usize = 0xE043FF88; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0xC0F85007; // 
                pub const : usize = 0xE2900001; // 
                pub const : usize = 0x49A00440; // 
                pub const : usize = 0x5C601380; // 
                pub const : usize = 0x5C601380; // 
                pub const : usize = 0x5C601780; // 
                pub const : usize = 0x5C601380; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x5C601380; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x3868103E; // 
                pub const : usize = 0x5BB68380; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x5CA00500; // 
                pub const : usize = 0xC0F85007; // 
                pub const : usize = 0xC0F85007; // 
                pub const : usize = 0x5CA00500; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x3968103F; // 
                pub const : usize = 0x49A00440; // 
                pub const : usize = 0x32A0053E; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x32A20B3F; // 
                pub const : usize = 0x32A003BF; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x5C5A5000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x5C5A1000; // 
                pub const : usize = 0x5C5B1000; // 
                pub const : usize = 0x51A00D40; // 
                pub const : usize = 0x5B450500; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x4B4B0540; // 
                pub const : usize = 0x3868103F; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0xE2900001; // 
                pub const : usize = 0x5C5B1000; // 
                pub const : usize = 0x3868103F; // 
                pub const : usize = 0x3868103E; // 
                pub const : usize = 0x5C980780; // 
                pub const : usize = 0x5C681000; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x58C68380; // 
                pub const : usize = 0x58C60380; // 
                pub const : usize = 0x5BB98380; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xE2900000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x49A00140; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x5BB68380; // 
                pub const : usize = 0x5C433000; // 
                pub const : usize = 0xE2A00000; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xE2900000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x5BB68380; // 
                pub const : usize = 0x5B5A2000; // 
                pub const : usize = 0x5B640400; // 
                pub const : usize = 0x5B6A2000; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x58C62000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0x5BB98480; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x5C433000; // 
                pub const : usize = 0xE2A00000; // 
                pub const : usize = 0x5BB6A080; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xE2900000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x49A00140; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x58C62080; // 
                pub const : usize = 0x5BB98400; // 
                pub const : usize = 0x5BB6A000; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x58C62000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0x5BB98480; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x5C433000; // 
                pub const : usize = 0xE2A00000; // 
                pub const : usize = 0x5BB6A080; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xE2900000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x49A00140; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x58C62080; // 
                pub const : usize = 0x5BB98400; // 
                pub const : usize = 0x5BB6A000; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x58C62000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0x5BB98480; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x5C433000; // 
                pub const : usize = 0xE2A00000; // 
                pub const : usize = 0x5BB6A080; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xE2900000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x49A00140; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x58C62080; // 
                pub const : usize = 0x5BB98400; // 
                pub const : usize = 0x5BB6A000; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD84C500F; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x58C62000; // 
                pub const : usize = 0xE2400000; // 
                pub const : usize = 0x5BB98480; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0xD850500F; // 
                pub const : usize = 0x49A00140; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x5C433000; // 
                pub const : usize = 0xE2A00000; // 
                pub const : usize = 0x5BB6A080; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0xE2900000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x58C62080; // 
                pub const : usize = 0x5BB98400; // 
                pub const : usize = 0x5BB6A000; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C980780; // 
                pub const : usize = 0x5C980780; // 
                pub const : usize = 0x5C980780; // 
                pub const : usize = 0xE2900000; // 
                pub const : usize = 0xF0F80000; // 
                pub const : usize = 0xE3400000; // 
                pub const : usize = 0x49A00040; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x50900380; // 
                pub const : usize = 0x5C423000; // 
                pub const : usize = 0x50900380; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x5C980780; // 
                pub const : usize = 0x5C980780; // 
                pub const : usize = 0x5C583000; // 
                pub const : usize = 0x5C591000; // 
                pub const : usize = 0x58A10B80; // 
                pub const : usize = 0x58A10B80; // 
                pub const : usize = 0x51A10040; // 
                pub const : usize = 0x5B450500; // 
                pub const : usize = 0x5C581000; // 
                pub const : usize = 0x49A000C0; // 
                pub const : usize = 0x59A10900; // 
                pub const : usize = 0x59A10280; // 
                pub const : usize = 0x5C601780; // 
                pub const : usize = 0x36B383B3; // 
                pub const : usize = 0xE2400FFF; // 
                pub const : usize = 0x50B00000; // 
                pub const : usize = 0x3B23D70A; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x808; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xA48; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xC78; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xDD8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1010; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1190; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1398; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15D0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1730; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1AF8; // 
                pub const : usize = 0x100022; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x8000034; // 
                pub const : usize = 0x7000038; // 
                pub const : usize = 0x100082; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100072; // 
                pub const : usize = 0x41; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8020803A; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x4002; // 
                pub const : usize = 0xA000032; // 
                pub const : usize = 0x100246; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xCAB1508F; // 
                pub const : usize = 0xDD7F5579; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x7FF8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2FFFF03; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const  : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7FF8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2FFFF03; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const  : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x20; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7FF8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2FFFF03; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7FF8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2FFFF03; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const  : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7FF8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2FFFF03; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFFFFFFFF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10001; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const  : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7FF8; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2FFFF03; // 
            }
            // Parent: None
            // Field count: 47
            pub mod RenderProjectedMaterial_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_INIT_SetFloatAttributeToVectorExpression {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_ExternalWindForce {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_ModelCull {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderSprites {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_PercentageBetweenTransformLerpCPs {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetPerChildControlPoint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderTreeShake {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_WorldCollideConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_SetAttributeToScalarExpression {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_OP_CycleScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RenderMaterialProxy {
            }
            // Parent: None
            // Field count: 47
            pub mod FloatInputMaterialVariable_t {
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
                pub const : usize = 0x14C3A0; // 
                pub const : usize = 0x740073; // 
                pub const : usize = 0x64005C; // 
                pub const : usize = 0x5C0065; // 
                pub const : usize = 0x6F006D; // 
                pub const : usize = 0x530000; // 
                pub const : usize = 0x5C0079; // 
                pub const : usize = 0x6E006F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x670062; // 
                pub const : usize = 0x6C006C; // 
                pub const : usize = 0x650074; // 
                pub const : usize = 0x61006D; // 
                pub const : usize = 0x200061; // 
                pub const : usize = 0x5C006E; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x7FF8; // 
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
            pub mod C_OP_RampScalarLinear {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_RotateVector {
            }
            // Parent: particles
            // Field count: 0
            pub mod C_INIT_InitVecCollection {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_RemapParticleCountToNamedModelMeshGroupScalar {
            }
            // Parent: None
            // Field count: 0
            pub mod C_INIT_SequenceFromCP {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_CPOffsetToPercentageBetweenCPs {
            }
            // Parent: None
            // Field count: 0
            pub mod C_OP_LerpEndCapScalar {
            }
        }
    }
}
