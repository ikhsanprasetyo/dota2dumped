// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-08 20:27:02.884074100 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: particles.dll
        // Class count: 346
        // Enum count: 77
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
            // Member count: 2
            #[repr(u32)]
            pub enum PulseCursorWakePriority_t {
                WakeElegantly = 0x0,
                WakeImmediate = 0x1
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
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleEntityPos_t {
                PARTICLE_ABS_ORIGIN = 0x0,
                PARTICLE_WORLDSPACE_CENTER = 0x1,
                PARTICLE_EYES = 0x2,
                PARTICLE_FLASHLIGHT = 0x3
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
            // Member count: 13
            #[repr(u32)]
            pub enum EventTypeSelection_t {
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
            // Member count: 11
            #[repr(u32)]
            pub enum ParticleAttrBoxFlags_t {
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
            // Member count: 5
            #[repr(u32)]
            pub enum ParticleOmni2LighOrientationChoiceList_t {
                PARTICLE_OMNI2_LIGHT_ORIENTATION_ROTATIONS = 0x0,
                PARTICLE_OMNI2_LIGHT_ORIENTATION_NORMAL = 0x1,
                PARTICLE_OMNI2_LIGHT_ORIENTATION_NORMAL_ROLL = 0x2,
                PARTICLE_OMNI2_LIGHT_ORIENTATION_TARGET = 0x3,
                PARTICLE_OMNI2_LIGHT_ORIENTATION_TARGET_ROLL = 0x4
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
            // Member count: 5
            #[repr(u32)]
            pub enum BBoxVolumeType_t {
                BBOX_VOLUME = 0x0,
                BBOX_DIMENSIONS = 0x1,
                BBOX_MINS_MAXS = 0x2,
                BBOX_RADIUS = 0x3,
                BBOX_SURFACE_AREA = 0x4
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
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleVolumetricSmokeType_t {
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_EMISSION = 0x0,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_SINK = 0x1,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_REPEL = 0x2,
                PARTICLE_VOLUMETRIC_SMOKE_TYPE_TRACE = 0x3
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
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleOmni2LightTypeChoiceList_t {
                PARTICLE_OMNI2_LIGHT_TYPE_POINT = 0x0,
                PARTICLE_OMNI2_LIGHT_TYPE_SPHERE = 0x1,
                PARTICLE_OMNI2_LIGHT_TYPE_BARN = 0x2
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
            // Field count: 0
            pub mod IParticleCollection {
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
            pub mod C_OP_RemapGravityToVector {
                pub const m_vInput1: usize = 0x1E0; // CPerParticleVecInput
                pub const m_nOutputField: usize = 0x8B8; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x8BC; // ParticleSetMethod_t
                pub const m_bNormalizedOutput: usize = 0x8C0; // bool
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
            pub mod C_OP_RenderDeferredLight {
                pub const m_flRadiusScale: usize = 0x230; // float32
                pub const m_flAlphaScale: usize = 0x234; // float32
                pub const m_nAlpha2Field: usize = 0x238; // ParticleAttributeIndex_t
                pub const m_vecColorScale: usize = 0x240; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0x918; // ParticleColorBlendType_t
                pub const m_bUseTexture: usize = 0x91C; // bool
                pub const m_bUseAlphaTestWindow: usize = 0x91D; // bool
                pub const m_hTexture: usize = 0x920; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_nAlphaTestPointField: usize = 0x928; // ParticleAttributeIndex_t
                pub const m_nAlphaTestRangeField: usize = 0x92C; // ParticleAttributeIndex_t
                pub const m_nAlphaTestSharpnessField: usize = 0x930; // ParticleAttributeIndex_t
                pub const m_flLightDistance: usize = 0x934; // float32
                pub const m_flStartFalloff: usize = 0x938; // float32
                pub const m_flDistanceFalloff: usize = 0x93C; // float32
                pub const m_flSpotFoV: usize = 0x940; // float32
                pub const m_nHSVShiftControlPoint: usize = 0x944; // int32
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
            pub mod C_OP_RemapTransformToVelocity {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            // MPropertyFriendlyName
            pub mod CollisionGroupContext_t {
                pub const m_nCollisionGroupNumber: usize = 0x0; // int32
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
            pub mod C_OP_FadeOutSimple {
                pub const m_flFadeOutTime: usize = 0x1E0; // float32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
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
            pub mod C_OP_SpringToVectorConstraint {
                pub const m_flRestLength: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flMinDistance: usize = 0x358; // CPerParticleFloatInput
                pub const m_flMaxDistance: usize = 0x4D0; // CPerParticleFloatInput
                pub const m_flRestingLength: usize = 0x648; // CPerParticleFloatInput
                pub const m_vecAnchorVector: usize = 0x7C0; // CPerParticleVecInput
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
            pub mod C_INIT_StatusEffectCitadel {
                pub const m_flSFXColorWarpAmount: usize = 0x1E8; // float32
                pub const m_flSFXNormalAmount: usize = 0x1EC; // float32
                pub const m_flSFXMetalnessAmount: usize = 0x1F0; // float32
                pub const m_flSFXRoughnessAmount: usize = 0x1F4; // float32
                pub const m_flSFXSelfIllumAmount: usize = 0x1F8; // float32
                pub const m_flSFXSScale: usize = 0x1FC; // float32
                pub const m_flSFXSScrollX: usize = 0x200; // float32
                pub const m_flSFXSScrollY: usize = 0x204; // float32
                pub const m_flSFXSScrollZ: usize = 0x208; // float32
                pub const m_flSFXSOffsetX: usize = 0x20C; // float32
                pub const m_flSFXSOffsetY: usize = 0x210; // float32
                pub const m_flSFXSOffsetZ: usize = 0x214; // float32
                pub const m_nDetailCombo: usize = 0x218; // DetailCombo_t
                pub const m_flSFXSDetailAmount: usize = 0x21C; // float32
                pub const m_flSFXSDetailScale: usize = 0x220; // float32
                pub const m_flSFXSDetailScrollX: usize = 0x224; // float32
                pub const m_flSFXSDetailScrollY: usize = 0x228; // float32
                pub const m_flSFXSDetailScrollZ: usize = 0x22C; // float32
                pub const m_flSFXSUseModelUVs: usize = 0x230; // float32
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
            pub mod C_OP_RenderSound {
                pub const m_flDurationScale: usize = 0x230; // float32
                pub const m_flSndLvlScale: usize = 0x234; // float32
                pub const m_flPitchScale: usize = 0x238; // float32
                pub const m_flVolumeScale: usize = 0x23C; // float32
                pub const m_nSndLvlField: usize = 0x240; // ParticleAttributeIndex_t
                pub const m_nDurationField: usize = 0x244; // ParticleAttributeIndex_t
                pub const m_nPitchField: usize = 0x248; // ParticleAttributeIndex_t
                pub const m_nVolumeField: usize = 0x24C; // ParticleAttributeIndex_t
                pub const m_nChannel: usize = 0x250; // int32
                pub const m_nCPReference: usize = 0x254; // int32
                pub const m_pszSoundName: usize = 0x258; // char[256]
                pub const m_bSuppressStopSoundEvent: usize = 0x358; // bool
            }
            // Parent: None
            // Field count: 19
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // COLLISION_MODE_PER_PARTICLE_TRACE
            // COLLISION_MODE_USE_NEAREST_TRACE
            pub mod CParticleVisibilityInputs {
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
            pub mod C_OP_SetControlPointsToParticle {
                pub const m_nChildGroupID: usize = 0x1E0; // int32
                pub const m_nFirstControlPoint: usize = 0x1E4; // int32
                pub const m_nNumControlPoints: usize = 0x1E8; // int32
                pub const m_nFirstSourcePoint: usize = 0x1EC; // int32
                pub const m_bReverse: usize = 0x1F0; // bool
                pub const m_bSetOrientation: usize = 0x1F1; // bool
                pub const m_nOrientationMode: usize = 0x1F4; // ParticleOrientationSetMode_t
                pub const m_nSetParent: usize = 0x1F8; // ParticleParentSetMode_t
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
            pub mod C_OP_RemapCPVelocityToVector {
                pub const m_nControlPoint: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1E8; // float32
                pub const m_bNormalize: usize = 0x1EC; // bool
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
            pub mod C_OP_PointVectorAtNextParticle {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInterpolation: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_bPrevious: usize = 0x360; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // PARTICLE_WORLDSPACE_CENTER
            // PARTICLE_EYES
            pub mod ParticlePreviewBodyGroup_t {
                pub const m_bodyGroupName: usize = 0x0; // CUtlString
                pub const m_nValue: usize = 0x8; // int32
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
            pub mod C_OP_OscillateScalarSimple {
                pub const m_Rate: usize = 0x1E0; // float32
                pub const m_Frequency: usize = 0x1E4; // float32
                pub const m_nField: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_flOscMult: usize = 0x1EC; // float32
                pub const m_flOscAdd: usize = 0x1F0; // float32
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
            pub mod C_INIT_StatusEffect {
                pub const m_nDetail2Combo: usize = 0x1E8; // Detail2Combo_t
                pub const m_flDetail2Rotation: usize = 0x1EC; // float32
                pub const m_flDetail2Scale: usize = 0x1F0; // float32
                pub const m_flDetail2BlendFactor: usize = 0x1F4; // float32
                pub const m_flColorWarpIntensity: usize = 0x1F8; // float32
                pub const m_flDiffuseWarpBlendToFull: usize = 0x1FC; // float32
                pub const m_flEnvMapIntensity: usize = 0x200; // float32
                pub const m_flAmbientScale: usize = 0x204; // float32
                pub const m_specularColor: usize = 0x208; // Color
                pub const m_flSpecularScale: usize = 0x20C; // float32
                pub const m_flSpecularExponent: usize = 0x210; // float32
                pub const m_flSpecularExponentBlendToFull: usize = 0x214; // float32
                pub const m_flSpecularBlendToFull: usize = 0x218; // float32
                pub const m_rimLightColor: usize = 0x21C; // Color
                pub const m_flRimLightScale: usize = 0x220; // float32
                pub const m_flReflectionsTintByBaseBlendToNone: usize = 0x224; // float32
                pub const m_flMetalnessBlendToFull: usize = 0x228; // float32
                pub const m_flSelfIllumBlendToFull: usize = 0x22C; // float32
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
            pub mod C_INIT_RandomVector {
                pub const m_vecMin: usize = 0x1E8; // Vector
                pub const m_vecMax: usize = 0x1F4; // Vector
                pub const m_nFieldOutput: usize = 0x200; // ParticleAttributeIndex_t
                pub const m_randomnessParameters: usize = 0x204; // CRandomNumberGeneratorParameters
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
            pub mod C_INIT_InitialVelocityNoise {
                pub const m_vecAbsVal: usize = 0x1E8; // Vector
                pub const m_vecAbsValInv: usize = 0x1F4; // Vector
                pub const m_vecOffsetLoc: usize = 0x200; // CPerParticleVecInput
                pub const m_flOffset: usize = 0x8D8; // CPerParticleFloatInput
                pub const m_vecOutputMin: usize = 0xA50; // CPerParticleVecInput
                pub const m_vecOutputMax: usize = 0x1128; // CPerParticleVecInput
                pub const m_flNoiseScale: usize = 0x1800; // CPerParticleFloatInput
                pub const m_flNoiseScaleLoc: usize = 0x1978; // CPerParticleFloatInput
                pub const m_TransformInput: usize = 0x1AF0; // CParticleTransformInput
                pub const m_bIgnoreDt: usize = 0x1B58; // bool
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
            pub mod C_OP_RemapScalarOnceTimed {
                pub const m_bProportional: usize = 0x1E0; // bool
                pub const m_nFieldInput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1EC; // float32
                pub const m_flInputMax: usize = 0x1F0; // float32
                pub const m_flOutputMin: usize = 0x1F4; // float32
                pub const m_flOutputMax: usize = 0x1F8; // float32
                pub const m_flRemapTime: usize = 0x1FC; // float32
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
            pub mod C_INIT_RandomNamedModelSequence {
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
            pub mod C_OP_PlaneCull {
                pub const m_nPlaneControlPoint: usize = 0x1E0; // int32
                pub const m_vecPlaneDirection: usize = 0x1E8; // CParticleCollectionVecInput
                pub const m_bLocalSpace: usize = 0x8C0; // bool
                pub const m_flPlaneOffset: usize = 0x8C4; // float32
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
            pub mod C_OP_ModelDampenMovement {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_bBoundBox: usize = 0x1E4; // bool
                pub const m_bOutside: usize = 0x1E5; // bool
                pub const m_bUseBones: usize = 0x1E6; // bool
                pub const m_HitboxSetName: usize = 0x1E7; // char[128]
                pub const m_vecPosOffset: usize = 0x268; // CPerParticleVecInput
                pub const m_fDrag: usize = 0x940; // float32
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
            pub mod C_OP_TwistAroundAxis {
                pub const m_fForceAmount: usize = 0x1F0; // float32
                pub const m_TwistAxis: usize = 0x1F4; // Vector
                pub const m_bLocalSpace: usize = 0x200; // bool
                pub const m_nControlPointNumber: usize = 0x204; // int32
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
            pub mod CSpinUpdateBase {
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
            pub mod C_OP_OrientTo2dDirection {
                pub const m_vecInput: usize = 0x1E0; // CPerParticleVecInput
                pub const m_flRotOffset: usize = 0x8B8; // float32
                pub const m_flSpinStrength: usize = 0x8BC; // float32
                pub const m_nFieldOutput: usize = 0x8C0; // ParticleAttributeIndex_t
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
            pub mod C_OP_RemapDotProductToCP {
                pub const m_nInputCP1: usize = 0x1E8; // int32
                pub const m_nInputCP2: usize = 0x1EC; // int32
                pub const m_nOutputCP: usize = 0x1F0; // int32
                pub const m_nOutVectorField: usize = 0x1F4; // int32
                pub const m_flInputMin: usize = 0x1F8; // CParticleCollectionFloatInput
                pub const m_flInputMax: usize = 0x370; // CParticleCollectionFloatInput
                pub const m_flOutputMin: usize = 0x4E8; // CParticleCollectionFloatInput
                pub const m_flOutputMax: usize = 0x660; // CParticleCollectionFloatInput
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
            pub mod C_INIT_RemapParticleCountToNamedModelElementScalar {
                pub const m_hModel: usize = 0x218; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_outputMinName: usize = 0x220; // CUtlString
                pub const m_outputMaxName: usize = 0x228; // CUtlString
                pub const m_bModelFromRenderer: usize = 0x230; // bool
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
            pub mod C_OP_RenderTrails {
                pub const m_bEnableFadingAndClamping: usize = 0x3358; // bool
                pub const m_flStartFadeDot: usize = 0x335C; // float32
                pub const m_flEndFadeDot: usize = 0x3360; // float32
                pub const m_nPrevPntSource: usize = 0x3364; // ParticleAttributeIndex_t
                pub const m_flMaxLength: usize = 0x3368; // float32
                pub const m_flMinLength: usize = 0x336C; // float32
                pub const m_bIgnoreDT: usize = 0x3370; // bool
                pub const m_flConstrainRadiusToLengthRatio: usize = 0x3374; // float32
                pub const m_flLengthScale: usize = 0x3378; // float32
                pub const m_flLengthFadeInTime: usize = 0x337C; // float32
                pub const m_flRadiusHeadTaper: usize = 0x3380; // CPerParticleFloatInput
                pub const m_vecHeadColorScale: usize = 0x34F8; // CParticleCollectionVecInput
                pub const m_flHeadAlphaScale: usize = 0x3BD0; // CPerParticleFloatInput
                pub const m_flRadiusTaper: usize = 0x3D48; // CPerParticleFloatInput
                pub const m_vecTailColorScale: usize = 0x3EC0; // CParticleCollectionVecInput
                pub const m_flTailAlphaScale: usize = 0x4598; // CPerParticleFloatInput
                pub const m_nHorizCropField: usize = 0x4710; // ParticleAttributeIndex_t
                pub const m_nVertCropField: usize = 0x4714; // ParticleAttributeIndex_t
                pub const m_flForwardShift: usize = 0x4718; // float32
                pub const m_bFlipUVBasedOnPitchYaw: usize = 0x471C; // bool
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
            pub mod C_OP_SetControlPointPositionToTimeOfDayValue {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_pszTimeOfDayParameter: usize = 0x1EC; // char[128]
                pub const m_vecDefaultValue: usize = 0x26C; // Vector
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
            pub mod C_OP_DecayMaintainCount {
                pub const m_nParticlesToMaintain: usize = 0x1E0; // int32
                pub const m_flDecayDelay: usize = 0x1E4; // float32
                pub const m_nSnapshotControlPoint: usize = 0x1E8; // int32
                pub const m_strSnapshotSubset: usize = 0x1F0; // CUtlString
                pub const m_bLifespanDecay: usize = 0x1F8; // bool
                pub const m_flScale: usize = 0x200; // CParticleCollectionFloatInput
                pub const m_bKillNewest: usize = 0x378; // bool
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
            pub mod C_INIT_RandomModelSequence {
                pub const m_ActivityName: usize = 0x1E8; // char[256]
                pub const m_SequenceName: usize = 0x2E8; // char[256]
                pub const m_hModel: usize = 0x3E8; // CStrongHandle<InfoForResourceTypeCModel>
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
            pub mod C_OP_ExternalGameImpulseForce {
                pub const m_flForceScale: usize = 0x1F0; // CPerParticleFloatInput
                pub const m_bRopes: usize = 0x368; // bool
                pub const m_bRopesZOnly: usize = 0x369; // bool
                pub const m_bExplosions: usize = 0x36A; // bool
                pub const m_bParticles: usize = 0x36B; // bool
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
            pub mod C_OP_RemapAverageHitboxSpeedtoCP {
                pub const m_nInControlPointNumber: usize = 0x1E8; // int32
                pub const m_nOutControlPointNumber: usize = 0x1EC; // int32
                pub const m_nField: usize = 0x1F0; // int32
                pub const m_nHitboxDataType: usize = 0x1F4; // ParticleHitboxDataSelection_t
                pub const m_flInputMin: usize = 0x1F8; // CParticleCollectionFloatInput
                pub const m_flInputMax: usize = 0x370; // CParticleCollectionFloatInput
                pub const m_flOutputMin: usize = 0x4E8; // CParticleCollectionFloatInput
                pub const m_flOutputMax: usize = 0x660; // CParticleCollectionFloatInput
                pub const m_nHeightControlPointNumber: usize = 0x7D8; // int32
                pub const m_vecComparisonVelocity: usize = 0x7E0; // CParticleCollectionVecInput
                pub const m_HitboxSetName: usize = 0xEB8; // char[128]
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
            pub mod C_INIT_RandomAlpha {
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_nAlphaMin: usize = 0x1EC; // int32
                pub const m_nAlphaMax: usize = 0x1F0; // int32
                pub const m_flAlphaRandExponent: usize = 0x1FC; // float32
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
            pub mod C_OP_NormalizeVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1E4; // float32
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
            pub mod C_OP_RepeatedTriggerChildGroup {
                pub const m_nChildGroupID: usize = 0x1E8; // int32
                pub const m_flClusterRefireTime: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_flClusterSize: usize = 0x368; // CParticleCollectionFloatInput
                pub const m_flClusterCooldown: usize = 0x4E0; // CParticleCollectionFloatInput
                pub const m_bLimitChildCount: usize = 0x658; // bool
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
            pub mod C_OP_RemapVelocityToVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1E4; // float32
                pub const m_bNormalize: usize = 0x1E8; // bool
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
            pub mod C_INIT_RingWave {
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_flParticlesPerOrbit: usize = 0x250; // CParticleCollectionFloatInput
                pub const m_flInitialRadius: usize = 0x3C8; // CPerParticleFloatInput
                pub const m_flThickness: usize = 0x540; // CPerParticleFloatInput
                pub const m_flInitialSpeedMin: usize = 0x6B8; // CPerParticleFloatInput
                pub const m_flInitialSpeedMax: usize = 0x830; // CPerParticleFloatInput
                pub const m_flRoll: usize = 0x9A8; // CPerParticleFloatInput
                pub const m_flPitch: usize = 0xB20; // CPerParticleFloatInput
                pub const m_flYaw: usize = 0xC98; // CPerParticleFloatInput
                pub const m_bEvenDistribution: usize = 0xE10; // bool
                pub const m_bXYVelocityOnly: usize = 0xE11; // bool
                pub const m_nControlPoint: usize = 0x1E8; // int32
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
            pub mod C_INIT_RandomTrailLength {
                pub const m_flMinLength: usize = 0x1E8; // float32
                pub const m_flMaxLength: usize = 0x1EC; // float32
                pub const m_flLengthRandExponent: usize = 0x1F0; // float32
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
            pub mod C_OP_RemapScalar {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // float32
                pub const m_flInputMax: usize = 0x1EC; // float32
                pub const m_flOutputMin: usize = 0x1F0; // float32
                pub const m_flOutputMax: usize = 0x1F4; // float32
                pub const m_bOldCode: usize = 0x1F8; // bool
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
            pub mod C_OP_DistanceBetweenTransforms {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_TransformStart: usize = 0x1E8; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x250; // CParticleTransformInput
                pub const m_flInputMin: usize = 0x2B8; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x430; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x5A8; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x720; // CPerParticleFloatInput
                pub const m_flMaxTraceLength: usize = 0x898; // float32
                pub const m_flLOSScale: usize = 0x89C; // float32
                pub const m_CollisionGroupName: usize = 0x8A0; // char[128]
                pub const m_nTraceSet: usize = 0x920; // ParticleTraceSet_t
                pub const m_bLOS: usize = 0x924; // bool
                pub const m_nSetMethod: usize = 0x928; // ParticleSetMethod_t
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
            pub mod C_OP_DecayOffscreen {
                pub const m_flOffscreenTime: usize = 0x1E0; // CParticleCollectionFloatInput
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
            pub mod C_INIT_CreateSequentialPath {
                pub const m_fMaxDistance: usize = 0x1E8; // float32
                pub const m_flNumToAssign: usize = 0x1EC; // float32
                pub const m_bLoop: usize = 0x1F0; // bool
                pub const m_bCPPairs: usize = 0x1F1; // bool
                pub const m_bSaveOffset: usize = 0x1F2; // bool
                pub const m_PathParams: usize = 0x200; // CPathParameters
                pub const m_bKillUnused: usize = 0x1E8; // bool
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
            pub mod C_OP_OscillateVectorSimple {
                pub const m_Rate: usize = 0x1E0; // Vector
                pub const m_Frequency: usize = 0x1EC; // Vector
                pub const m_nField: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const m_flOscMult: usize = 0x1FC; // float32
                pub const m_flOscAdd: usize = 0x200; // float32
                pub const m_bOffset: usize = 0x204; // bool
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
            pub mod C_INIT_MoveBetweenPoints {
                pub const m_flSpeedMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flSpeedMax: usize = 0x360; // CPerParticleFloatInput
                pub const m_flEndSpread: usize = 0x4D8; // CPerParticleFloatInput
                pub const m_flStartOffset: usize = 0x650; // CPerParticleFloatInput
                pub const m_flEndOffset: usize = 0x7C8; // CPerParticleFloatInput
                pub const m_nEndControlPointNumber: usize = 0x940; // int32
                pub const m_bTrailBias: usize = 0x944; // bool
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
            pub mod C_INIT_StatusEffectTf {
                pub const m_flSFXColorWarpAmount: usize = 0x1E8; // float32
                pub const m_flSFXNormalAmount: usize = 0x1EC; // float32
                pub const m_flSFXMetalnessAmount: usize = 0x1F0; // float32
                pub const m_flSFXRoughnessAmount: usize = 0x1F4; // float32
                pub const m_flSFXSelfIllumAmount: usize = 0x1F8; // float32
                pub const m_flSFXSScale: usize = 0x1FC; // float32
                pub const m_flSFXSScrollX: usize = 0x200; // float32
                pub const m_flSFXSScrollY: usize = 0x204; // float32
                pub const m_flSFXSScrollZ: usize = 0x208; // float32
                pub const m_flSFXSOffsetX: usize = 0x20C; // float32
                pub const m_flSFXSOffsetY: usize = 0x210; // float32
                pub const m_flSFXSOffsetZ: usize = 0x214; // float32
                pub const m_nDetailCombo: usize = 0x218; // DetailCombo_t
                pub const m_flSFXSDetailAmount: usize = 0x21C; // float32
                pub const m_flSFXSDetailScale: usize = 0x220; // float32
                pub const m_flSFXSDetailScrollX: usize = 0x224; // float32
                pub const m_flSFXSDetailScrollY: usize = 0x228; // float32
                pub const m_flSFXSDetailScrollZ: usize = 0x22C; // float32
                pub const m_bAnimatedDetail: usize = 0x230; // bool
                pub const m_flSFXDetailAnimationTimePerFrame: usize = 0x234; // float32
                pub const m_flSFXDetailAnimationTimeOffset: usize = 0x238; // float32
                pub const m_flSFXSUseModelUVs: usize = 0x23C; // float32
                pub const m_flSFXEnvMapAmount: usize = 0x240; // float32
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
            pub mod C_OP_SetUserEvent {
                pub const m_flInput: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flRisingEdge: usize = 0x358; // CPerParticleFloatInput
                pub const m_nRisingEventType: usize = 0x4D0; // EventTypeSelection_t
                pub const m_flFallingEdge: usize = 0x4D8; // CPerParticleFloatInput
                pub const m_nFallingEventType: usize = 0x650; // EventTypeSelection_t
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
            pub mod C_OP_QuantizeFloat {
                pub const m_InputValue: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x358; // ParticleAttributeIndex_t
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
            pub mod C_INIT_RandomNamedModelElement {
                pub const m_hModel: usize = 0x1E8; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_names: usize = 0x1F0; // CUtlVector<CUtlString>
                pub const m_bShuffle: usize = 0x208; // bool
                pub const m_bLinear: usize = 0x209; // bool
                pub const m_bModelFromRenderer: usize = 0x20A; // bool
                pub const m_nFieldOutput: usize = 0x20C; // ParticleAttributeIndex_t
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
            pub mod C_OP_Callback {
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
            pub mod C_OP_GlobalLight {
                pub const m_flScale: usize = 0x1E0; // float32
                pub const m_bClampLowerRange: usize = 0x1E4; // bool
                pub const m_bClampUpperRange: usize = 0x1E5; // bool
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
            pub mod C_INIT_OffsetVectorToVector {
                pub const m_nFieldInput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_vecOutputMin: usize = 0x1F0; // Vector
                pub const m_vecOutputMax: usize = 0x1FC; // Vector
                pub const m_randomnessParameters: usize = 0x208; // CRandomNumberGeneratorParameters
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
            pub mod C_OP_SetPerChildControlPointFromAttribute {
                pub const m_nChildGroupID: usize = 0x1E0; // int32
                pub const m_nFirstControlPoint: usize = 0x1E4; // int32
                pub const m_nNumControlPoints: usize = 0x1E8; // int32
                pub const m_nParticleIncrement: usize = 0x1EC; // int32
                pub const m_nFirstSourcePoint: usize = 0x1F0; // int32
                pub const m_bNumBasedOnParticleCount: usize = 0x1F4; // bool
                pub const m_nAttributeToRead: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const m_nCPField: usize = 0x1FC; // int32
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
            pub mod C_OP_SetParentControlPointsToChildCP {
                pub const m_nChildGroupID: usize = 0x1E8; // int32
                pub const m_nChildControlPoint: usize = 0x1EC; // int32
                pub const m_nNumControlPoints: usize = 0x1F0; // int32
                pub const m_nFirstSourcePoint: usize = 0x1F4; // int32
                pub const m_bSetOrientation: usize = 0x1F8; // bool
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
            pub mod C_OP_BoxConstraint {
                pub const m_vecMin: usize = 0x1E0; // CParticleCollectionVecInput
                pub const m_vecMax: usize = 0x8B8; // CParticleCollectionVecInput
                pub const m_nCP: usize = 0xF90; // int32
                pub const m_bLocalSpace: usize = 0xF94; // bool
                pub const m_bAccountForRadius: usize = 0xF95; // bool
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
            pub mod C_INIT_CreatePhyllotaxis {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_nScaleCP: usize = 0x1EC; // int32
                pub const m_nComponent: usize = 0x1F0; // int32
                pub const m_fRadCentCore: usize = 0x1F4; // float32
                pub const m_fRadPerPoint: usize = 0x1F8; // float32
                pub const m_fRadPerPointTo: usize = 0x1FC; // float32
                pub const m_fpointAngle: usize = 0x200; // float32
                pub const m_fsizeOverall: usize = 0x204; // float32
                pub const m_fRadBias: usize = 0x208; // float32
                pub const m_fMinRad: usize = 0x20C; // float32
                pub const m_fDistBias: usize = 0x210; // float32
                pub const m_bUseLocalCoords: usize = 0x214; // bool
                pub const m_bUseWithContEmit: usize = 0x215; // bool
                pub const m_bUseOrigRadius: usize = 0x216; // bool
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
            pub mod C_OP_AttractToControlPoint {
                pub const m_vecComponentScale: usize = 0x1F0; // Vector
                pub const m_fForceAmount: usize = 0x200; // CPerParticleFloatInput
                pub const m_fMinimumDistance: usize = 0x378; // CPerParticleFloatInput
                pub const m_fFalloffPower: usize = 0x4F0; // float32
                pub const m_TransformInput: usize = 0x4F8; // CParticleTransformInput
                pub const m_fForceAmountMin: usize = 0x560; // CPerParticleFloatInput
                pub const m_bApplyMinForce: usize = 0x6D8; // bool
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
            pub mod C_INIT_RandomLifeTime {
                pub const m_fLifetimeMin: usize = 0x1E8; // float32
                pub const m_fLifetimeMax: usize = 0x1EC; // float32
                pub const m_fLifetimeRandExponent: usize = 0x1F0; // float32
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
            pub mod C_INIT_RemapParticleCountToNamedModelSequenceScalar {
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
            pub mod C_INIT_VelocityRadialRandom {
                pub const m_bPerParticleCenter: usize = 0x1E8; // bool
                pub const m_nControlPointNumber: usize = 0x1EC; // int32
                pub const m_vecPosition: usize = 0x1F0; // CPerParticleVecInput
                pub const m_vecFwd: usize = 0x8C8; // CPerParticleVecInput
                pub const m_fSpeedMin: usize = 0xFA0; // CPerParticleFloatInput
                pub const m_fSpeedMax: usize = 0x1118; // CPerParticleFloatInput
                pub const m_vecLocalCoordinateSystemSpeedScale: usize = 0x1290; // Vector
                pub const m_bIgnoreDelta: usize = 0x129D; // bool
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
            pub mod C_INIT_RandomRadius {
                pub const m_flRadiusMin: usize = 0x1E8; // float32
                pub const m_flRadiusMax: usize = 0x1EC; // float32
                pub const m_flRadiusRandExponent: usize = 0x1F0; // float32
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
            pub mod C_OP_Orient2DRelToCP {
                pub const m_flRotOffset: usize = 0x1E0; // float32
                pub const m_flSpinStrength: usize = 0x1E4; // float32
                pub const m_nCP: usize = 0x1E8; // int32
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // PARTICLE_TOPOLOGY_LINES
            pub mod ControlPointReference_t {
                pub const m_controlPointNameString: usize = 0x0; // int32
                pub const m_vOffsetFromControlPoint: usize = 0x4; // Vector
                pub const m_bOffsetInLocalSpace: usize = 0x10; // bool
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
            pub mod C_OP_LightningSnapshotGenerator {
                pub const m_nCPSnapshot: usize = 0x1E8; // int32
                pub const m_nCPStartPnt: usize = 0x1EC; // int32
                pub const m_nCPEndPnt: usize = 0x1F0; // int32
                pub const m_flSegments: usize = 0x1F8; // CParticleCollectionFloatInput
                pub const m_flOffset: usize = 0x370; // CParticleCollectionFloatInput
                pub const m_flOffsetDecay: usize = 0x4E8; // CParticleCollectionFloatInput
                pub const m_flRecalcRate: usize = 0x660; // CParticleCollectionFloatInput
                pub const m_flUVScale: usize = 0x7D8; // CParticleCollectionFloatInput
                pub const m_flUVOffset: usize = 0x950; // CParticleCollectionFloatInput
                pub const m_flSplitRate: usize = 0xAC8; // CParticleCollectionFloatInput
                pub const m_flRecursionSplitScale: usize = 0xC40; // CParticleCollectionFloatInput
                pub const m_bScaleBranchDistance: usize = 0xDB8; // bool
                pub const m_flBranchDistanceScale: usize = 0xDC0; // CParticleCollectionFloatInput
                pub const m_bScaleBranchOffset: usize = 0xF38; // bool
                pub const m_flBranchOffsetScale: usize = 0xF40; // CParticleCollectionFloatInput
                pub const m_flBranchTwist: usize = 0x10B8; // CParticleCollectionFloatInput
                pub const m_nBranchBehavior: usize = 0x1230; // ParticleLightnintBranchBehavior_t
                pub const m_flRadiusStart: usize = 0x1238; // CParticleCollectionFloatInput
                pub const m_flRadiusEnd: usize = 0x13B0; // CParticleCollectionFloatInput
                pub const m_flDedicatedPool: usize = 0x1528; // CParticleCollectionFloatInput
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
            pub mod C_OP_RemapNamedModelMeshGroupOnceTimed {
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
            pub mod C_INIT_RemapQAnglesToRotation {
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
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
            pub mod C_OP_SetControlPointFieldToScalarExpression {
                pub const m_nExpression: usize = 0x1E8; // ScalarExpressionType_t
                pub const m_flInput1: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_flInput2: usize = 0x368; // CParticleCollectionFloatInput
                pub const m_flOutputRemap: usize = 0x4E0; // CParticleRemapFloatInput
                pub const m_nOutputCP: usize = 0x658; // int32
                pub const m_nOutVectorField: usize = 0x65C; // int32
                pub const m_flInterpolation: usize = 0x660; // CParticleCollectionFloatInput
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
            pub mod C_OP_CreateParticleSystemRenderer {
                pub const m_hEffect: usize = 0x230; // CStrongHandle<InfoForResourceTypeIParticleSystemDefinition>
                pub const m_nEventType: usize = 0x238; // EventTypeSelection_t
                pub const m_vecCPs: usize = 0x240; // CUtlLeanVector<CPAssignment_t>
                pub const m_szParticleConfig: usize = 0x250; // CUtlString
                pub const m_AggregationPos: usize = 0x258; // CPerParticleVecInput
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
            pub mod C_INIT_RandomVectorComponent {
                pub const m_flMin: usize = 0x1E8; // float32
                pub const m_flMax: usize = 0x1EC; // float32
                pub const m_nFieldOutput: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_nComponent: usize = 0x1F4; // int32
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
            pub mod C_OP_InheritFromParentParticles {
                pub const m_flScale: usize = 0x1E0; // float32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x1E8; // int32
                pub const m_bRandomDistribution: usize = 0x1EC; // bool
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
            pub mod C_INIT_SetVectorAttributeToVectorExpression {
                pub const m_nExpression: usize = 0x1E8; // VectorExpressionType_t
                pub const m_vInput1: usize = 0x1F0; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x8C8; // CPerParticleVecInput
                pub const m_flLerp: usize = 0xFA0; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x1118; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x111C; // ParticleSetMethod_t
                pub const m_bNormalizedOutput: usize = 0x1120; // bool
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
            pub mod C_OP_RemapTransformVisibilityToVector {
                pub const m_nSetMethod: usize = 0x1E0; // ParticleSetMethod_t
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x250; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x254; // float32
                pub const m_flInputMax: usize = 0x258; // float32
                pub const m_vecOutputMin: usize = 0x25C; // Vector
                pub const m_vecOutputMax: usize = 0x268; // Vector
                pub const m_flRadius: usize = 0x274; // float32
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
            pub mod C_OP_DirectionBetweenVecsToVec {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vecPoint1: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vecPoint2: usize = 0x8C0; // CPerParticleVecInput
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
            pub mod C_OP_MovementLoopInsideSphere {
                pub const m_nCP: usize = 0x1E0; // int32
                pub const m_flDistance: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_vecScale: usize = 0x360; // CParticleCollectionVecInput
                pub const m_nDistSqrAttr: usize = 0xA38; // ParticleAttributeIndex_t
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
            pub mod C_OP_RenderSimpleModelCollection {
                pub const m_bCenterOffset: usize = 0x230; // bool
                pub const m_hModel: usize = 0x238; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_modelInput: usize = 0x240; // CParticleModelInput
                pub const m_fSizeCullScale: usize = 0x2A0; // CParticleCollectionFloatInput
                pub const m_bDisableShadows: usize = 0x418; // bool
                pub const m_bDisableMotionBlur: usize = 0x419; // bool
                pub const m_bAcceptsDecals: usize = 0x41A; // bool
                pub const m_fDrawFilter: usize = 0x420; // CPerParticleFloatInput
                pub const m_nAngularVelocityField: usize = 0x598; // ParticleAttributeIndex_t
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
            pub mod C_INIT_InitFloatCollection {
                pub const m_InputValue: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_nOutputField: usize = 0x360; // ParticleAttributeIndex_t
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
            pub mod CPathParameters {
                pub const m_nStartControlPointNumber: usize = 0x0; // int32
                pub const m_nMidControlPointNumber: usize = 0x4; // int32
                pub const m_nEndControlPointNumber: usize = 0x8; // int32
                pub const m_nBulgeControl: usize = 0xC; // int32
                pub const m_flBulge: usize = 0x10; // float32
                pub const m_flMidPoint: usize = 0x14; // float32
                pub const m_vStartPointOffset: usize = 0x18; // Vector
                pub const m_vMidPointOffset: usize = 0x24; // Vector
                pub const m_vEndOffset: usize = 0x30; // Vector
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
            pub mod C_OP_RemapScalarEndCap {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // float32
                pub const m_flInputMax: usize = 0x1EC; // float32
                pub const m_flOutputMin: usize = 0x1F0; // float32
                pub const m_flOutputMax: usize = 0x1F4; // float32
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
            pub mod C_INIT_CreateFromPlaneCache {
                pub const m_vecOffsetMin: usize = 0x1E8; // Vector
                pub const m_vecOffsetMax: usize = 0x1F4; // Vector
                pub const m_bUseNormal: usize = 0x201; // bool
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
            pub mod C_OP_LazyCullCompareFloat {
                pub const m_flComparsion1: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flComparsion2: usize = 0x358; // CPerParticleFloatInput
                pub const m_flCullTime: usize = 0x4D0; // CPerParticleFloatInput
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
            pub mod C_OP_ControlPointToRadialScreenSpace {
                pub const m_nCPIn: usize = 0x1E8; // int32
                pub const m_vecCP1Pos: usize = 0x1EC; // Vector
                pub const m_nCPOut: usize = 0x1F8; // int32
                pub const m_nCPOutField: usize = 0x1FC; // int32
                pub const m_nCPSSPosOut: usize = 0x200; // int32
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
            pub mod C_OP_SpinUpdate {
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
            pub mod C_INIT_NormalOffset {
                pub const m_OffsetMin: usize = 0x1E8; // Vector
                pub const m_OffsetMax: usize = 0x1F4; // Vector
                pub const m_nControlPointNumber: usize = 0x200; // int32
                pub const m_bLocalCoords: usize = 0x204; // bool
                pub const m_bNormalize: usize = 0x205; // bool
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
            pub mod C_INIT_CreationNoise {
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_bAbsVal: usize = 0x1EC; // bool
                pub const m_bAbsValInv: usize = 0x1ED; // bool
                pub const m_flOffset: usize = 0x1F0; // float32
                pub const m_flOutputMin: usize = 0x1F4; // float32
                pub const m_flOutputMax: usize = 0x1F8; // float32
                pub const m_flNoiseScale: usize = 0x1FC; // float32
                pub const m_flNoiseScaleLoc: usize = 0x200; // float32
                pub const m_vecOffsetLoc: usize = 0x204; // Vector
                pub const m_flWorldTimeScale: usize = 0x210; // float32
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
            pub mod C_OP_Spin {
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
            pub mod C_OP_GameLiquidSpill {
                pub const m_flLiquidContentsField: usize = 0x230; // CParticleCollectionFloatInput
                pub const m_flExpirationTime: usize = 0x3A8; // CParticleCollectionFloatInput
                pub const m_flRadius: usize = 0x520; // CParticleCollectionFloatInput
                pub const m_bCheckExposedToSky: usize = 0x698; // bool
                pub const m_nAmountAttribute: usize = 0x69C; // ParticleAttributeIndex_t
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
            pub mod C_OP_ConstrainLineLength {
                pub const m_flMinDistance: usize = 0x1E0; // float32
                pub const m_flMaxDistance: usize = 0x1E4; // float32
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
            pub mod C_INIT_LifespanFromVelocity {
                pub const m_vecComponentScale: usize = 0x1E8; // Vector
                pub const m_flTraceOffset: usize = 0x1F4; // float32
                pub const m_flMaxTraceLength: usize = 0x1F8; // float32
                pub const m_flTraceTolerance: usize = 0x1FC; // float32
                pub const m_nMaxPlanes: usize = 0x200; // int32
                pub const m_CollisionGroupName: usize = 0x208; // char[128]
                pub const m_nTraceSet: usize = 0x288; // ParticleTraceSet_t
                pub const m_bIncludeWater: usize = 0x298; // bool
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
            pub mod C_INIT_VelocityFromCP {
                pub const m_velocityInput: usize = 0x1E8; // CParticleCollectionVecInput
                pub const m_transformInput: usize = 0x8C0; // CParticleTransformInput
                pub const m_flVelocityScale: usize = 0x928; // float32
                pub const m_bDirectionOnly: usize = 0x92C; // bool
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
            pub mod C_OP_MovementSkinnedPositionFromCPSnapshot {
                pub const m_nSnapshotControlPointNumber: usize = 0x1E0; // int32
                pub const m_nControlPointNumber: usize = 0x1E4; // int32
                pub const m_bRandom: usize = 0x1E8; // bool
                pub const m_nRandomSeed: usize = 0x1EC; // int32
                pub const m_bSetNormal: usize = 0x1F0; // bool
                pub const m_bSetRadius: usize = 0x1F1; // bool
                pub const m_nIndexType: usize = 0x1F4; // SnapshotIndexType_t
                pub const m_flReadIndex: usize = 0x1F8; // CPerParticleFloatInput
                pub const m_flIncrement: usize = 0x370; // CParticleCollectionFloatInput
                pub const m_nFullLoopIncrement: usize = 0x4E8; // CParticleCollectionFloatInput
                pub const m_nSnapShotStartPoint: usize = 0x660; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0x7D8; // CPerParticleFloatInput
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
            pub mod C_OP_OscillateVector {
                pub const m_RateMin: usize = 0x1E0; // Vector
                pub const m_RateMax: usize = 0x1EC; // Vector
                pub const m_FrequencyMin: usize = 0x1F8; // Vector
                pub const m_FrequencyMax: usize = 0x204; // Vector
                pub const m_nField: usize = 0x210; // ParticleAttributeIndex_t
                pub const m_bProportional: usize = 0x214; // bool
                pub const m_bProportionalOp: usize = 0x215; // bool
                pub const m_bOffset: usize = 0x216; // bool
                pub const m_flStartTime_min: usize = 0x218; // float32
                pub const m_flStartTime_max: usize = 0x21C; // float32
                pub const m_flEndTime_min: usize = 0x220; // float32
                pub const m_flEndTime_max: usize = 0x224; // float32
                pub const m_flOscMult: usize = 0x228; // CPerParticleFloatInput
                pub const m_flOscAdd: usize = 0x3A0; // CPerParticleFloatInput
                pub const m_flRateScale: usize = 0x518; // CPerParticleFloatInput
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
            pub mod C_OP_PositionLock {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_flStartTime_min: usize = 0x248; // float32
                pub const m_flStartTime_max: usize = 0x24C; // float32
                pub const m_flStartTime_exp: usize = 0x250; // float32
                pub const m_flEndTime_min: usize = 0x254; // float32
                pub const m_flEndTime_max: usize = 0x258; // float32
                pub const m_flEndTime_exp: usize = 0x25C; // float32
                pub const m_flRange: usize = 0x260; // float32
                pub const m_flRangeBias: usize = 0x268; // CParticleCollectionFloatInput
                pub const m_flJumpThreshold: usize = 0x3E0; // float32
                pub const m_flPrevPosScale: usize = 0x3E4; // float32
                pub const m_bLockRot: usize = 0x3E8; // bool
                pub const m_vecScale: usize = 0x3F0; // CParticleCollectionVecInput
                pub const m_nFieldOutput: usize = 0xAC8; // ParticleAttributeIndex_t
                pub const m_nFieldOutputPrev: usize = 0xACC; // ParticleAttributeIndex_t
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
            pub mod C_OP_RenderVRHapticEvent {
                pub const m_nHand: usize = 0x230; // ParticleVRHandChoiceList_t
                pub const m_nOutputHandCP: usize = 0x234; // int32
                pub const m_nOutputField: usize = 0x238; // int32
                pub const m_flAmplitude: usize = 0x240; // CPerParticleFloatInput
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
            pub mod C_OP_SetControlPointToImpactPoint {
                pub const m_nCPOut: usize = 0x1E8; // int32
                pub const m_nCPIn: usize = 0x1EC; // int32
                pub const m_flUpdateRate: usize = 0x1F0; // float32
                pub const m_flTraceLength: usize = 0x1F8; // CParticleCollectionFloatInput
                pub const m_flStartOffset: usize = 0x370; // float32
                pub const m_flOffset: usize = 0x374; // float32
                pub const m_vecTraceDir: usize = 0x378; // Vector
                pub const m_CollisionGroupName: usize = 0x384; // char[128]
                pub const m_nTraceSet: usize = 0x404; // ParticleTraceSet_t
                pub const m_bSetToEndpoint: usize = 0x408; // bool
                pub const m_bTraceToClosestSurface: usize = 0x409; // bool
                pub const m_bIncludeWater: usize = 0x40A; // bool
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
            pub mod C_OP_ReinitializeScalarEndCap {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flOutputMin: usize = 0x1E4; // float32
                pub const m_flOutputMax: usize = 0x1E8; // float32
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
            pub mod C_OP_TurbulenceForce {
                pub const m_flNoiseCoordScale0: usize = 0x1F0; // float32
                pub const m_flNoiseCoordScale1: usize = 0x1F4; // float32
                pub const m_flNoiseCoordScale2: usize = 0x1F8; // float32
                pub const m_flNoiseCoordScale3: usize = 0x1FC; // float32
                pub const m_vecNoiseAmount0: usize = 0x200; // Vector
                pub const m_vecNoiseAmount1: usize = 0x20C; // Vector
                pub const m_vecNoiseAmount2: usize = 0x218; // Vector
                pub const m_vecNoiseAmount3: usize = 0x224; // Vector
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
            pub mod C_OP_RemapNamedModelElementOnceTimed {
                pub const m_hModel: usize = 0x1E0; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_inNames: usize = 0x1E8; // CUtlVector<CUtlString>
                pub const m_outNames: usize = 0x200; // CUtlVector<CUtlString>
                pub const m_fallbackNames: usize = 0x218; // CUtlVector<CUtlString>
                pub const m_bModelFromRenderer: usize = 0x230; // bool
                pub const m_bProportional: usize = 0x231; // bool
                pub const m_nFieldInput: usize = 0x234; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x238; // ParticleAttributeIndex_t
                pub const m_flRemapTime: usize = 0x23C; // float32
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
            pub mod C_OP_SetControlPointToPlayer {
                pub const m_nCP1: usize = 0x1E8; // int32
                pub const m_vecCP1Pos: usize = 0x1EC; // Vector
                pub const m_bOrientToEyes: usize = 0x1F8; // bool
                pub const m_nPosition: usize = 0x1FC; // ParticleEntityPos_t
                pub const m_nRadiusCP: usize = 0x200; // int32
                pub const m_nRadiusCPField: usize = 0x204; // int32
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
            pub mod C_OP_EndCapTimedFreeze {
                pub const m_flFreezeTime: usize = 0x1E0; // CParticleCollectionFloatInput
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
            pub mod C_OP_RenderGpuImplicit {
                pub const m_bUsePerParticleRadius: usize = 0x230; // bool
                pub const m_nVertexCountKb: usize = 0x234; // uint32
                pub const m_nIndexCountKb: usize = 0x238; // uint32
                pub const m_fGridSize: usize = 0x240; // CParticleCollectionRendererFloatInput
                pub const m_fRadiusScale: usize = 0x3B8; // CParticleCollectionRendererFloatInput
                pub const m_fIsosurfaceThreshold: usize = 0x530; // CParticleCollectionRendererFloatInput
                pub const m_nScaleCP: usize = 0x6A8; // int32
                pub const m_hMaterial: usize = 0x6B0; // CStrongHandle<InfoForResourceTypeIMaterial2>
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
            pub mod C_OP_RenderVolumetricEmitter {
                pub const m_strChannelType: usize = 0x230; // CUtlString
                pub const m_nType: usize = 0x238; // ParticleVolumetricSmokeType_t
                pub const m_nCreationType: usize = 0x23C; // ParticleVolumetricSmokeCreationType_t
                pub const m_nEventType: usize = 0x240; // EventTypeSelection_t
                pub const m_vecPos: usize = 0x248; // CPerParticleVecInput
                pub const m_vecVelocity: usize = 0x920; // CPerParticleVecInput
                pub const m_vPrevPosition: usize = 0xFF8; // CPerParticleVecInput
                pub const m_flSpeed: usize = 0x16D0; // CPerParticleFloatInput
                pub const m_flRadius: usize = 0x1848; // CPerParticleFloatInput
                pub const m_flDensity: usize = 0x19C0; // CPerParticleFloatInput
                pub const m_flTemperature: usize = 0x1B38; // CPerParticleFloatInput
                pub const m_flMagnitude: usize = 0x1CB0; // CPerParticleFloatInput
                pub const m_flKillRadius: usize = 0x1E28; // CPerParticleFloatInput
                pub const m_flKillDensityScale: usize = 0x1FA0; // CPerParticleFloatInput
                pub const m_flFalloff: usize = 0x2118; // CPerParticleFloatInput
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
            pub mod C_OP_RemapTransformVisibilityToScalar {
                pub const m_nSetMethod: usize = 0x1E0; // ParticleSetMethod_t
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x250; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x254; // float32
                pub const m_flInputMax: usize = 0x258; // float32
                pub const m_flOutputMin: usize = 0x25C; // float32
                pub const m_flOutputMax: usize = 0x260; // float32
                pub const m_flRadius: usize = 0x264; // float32
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
            pub mod C_OP_RemapControlPointDirectionToVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1E4; // float32
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
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
            pub mod C_OP_ScreenSpacePositionOfTarget {
                pub const m_vecTargetPosition: usize = 0x1E0; // CPerParticleVecInput
                pub const m_bOututBehindness: usize = 0x8B8; // bool
                pub const m_nBehindFieldOutput: usize = 0x8BC; // ParticleAttributeIndex_t
                pub const m_flBehindOutputRemap: usize = 0x8C0; // CParticleRemapFloatInput
                pub const m_nBehindSetMethod: usize = 0xA38; // ParticleSetMethod_t
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
            pub mod C_OP_DragRelativeToPlane {
                pub const m_flDragAtPlane: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_flFalloff: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_bDirectional: usize = 0x4D0; // bool
                pub const m_vecPlaneNormal: usize = 0x4D8; // CParticleCollectionVecInput
                pub const m_nControlPointNumber: usize = 0xBB0; // int32
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
            pub mod C_OP_SetCPtoVector {
                pub const m_nCPInput: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
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
            pub mod C_INIT_RandomYaw {
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
            pub mod C_OP_SnapshotRigidSkinToBones {
                pub const m_bTransformNormals: usize = 0x1E0; // bool
                pub const m_bTransformRadii: usize = 0x1E1; // bool
                pub const m_nControlPointNumber: usize = 0x1E4; // int32
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
            pub mod C_OP_SetSingleControlPointPosition {
                pub const m_bSetOnce: usize = 0x1E8; // bool
                pub const m_nCP1: usize = 0x1EC; // int32
                pub const m_vecCP1Pos: usize = 0x1F0; // CParticleCollectionVecInput
                pub const m_transformInput: usize = 0x8C8; // CParticleTransformInput
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
            pub mod C_OP_RemapCPtoScalar {
                pub const m_nCPInput: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nField: usize = 0x1E8; // int32
                pub const m_flInputMin: usize = 0x1EC; // float32
                pub const m_flInputMax: usize = 0x1F0; // float32
                pub const m_flOutputMin: usize = 0x1F4; // float32
                pub const m_flOutputMax: usize = 0x1F8; // float32
                pub const m_flStartTime: usize = 0x1FC; // float32
                pub const m_flEndTime: usize = 0x200; // float32
                pub const m_flInterpRate: usize = 0x204; // float32
                pub const m_nSetMethod: usize = 0x208; // ParticleSetMethod_t
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
            pub mod C_OP_RemapNamedModelMeshGroupEndCap {
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
            pub mod C_OP_PercentageBetweenTransformsVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_vecOutputMin: usize = 0x1EC; // Vector
                pub const m_vecOutputMax: usize = 0x1F8; // Vector
                pub const m_TransformStart: usize = 0x208; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x270; // CParticleTransformInput
                pub const m_nSetMethod: usize = 0x2D8; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x2DC; // bool
                pub const m_bRadialCheck: usize = 0x2DD; // bool
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
            pub mod C_OP_RenderScreenVelocityRotate {
                pub const m_flRotateRateDegrees: usize = 0x230; // float32
                pub const m_flForwardDegrees: usize = 0x234; // float32
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
            pub mod C_OP_UpdateLightSource {
                pub const m_vColorTint: usize = 0x1E0; // Color
                pub const m_flBrightnessScale: usize = 0x1E4; // float32
                pub const m_flRadiusScale: usize = 0x1E8; // float32
                pub const m_flMinimumLightingRadius: usize = 0x1EC; // float32
                pub const m_flMaximumLightingRadius: usize = 0x1F0; // float32
                pub const m_flPositionDampingConstant: usize = 0x1F4; // float32
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
            pub mod C_INIT_CreateWithinBox {
                pub const m_vecMin: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vecMax: usize = 0x8C0; // CPerParticleVecInput
                pub const m_nControlPointNumber: usize = 0xF98; // int32
                pub const m_bLocalSpace: usize = 0xF9C; // bool
                pub const m_randomnessParameters: usize = 0xFA0; // CRandomNumberGeneratorParameters
                pub const m_bUseNewCode: usize = 0xFA8; // bool
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
            pub mod C_OP_ChooseRandomChildrenInGroup {
                pub const m_nChildGroupID: usize = 0x1E8; // int32
                pub const m_flNumberOfChildren: usize = 0x1F0; // CParticleCollectionFloatInput
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
            pub mod C_OP_ControlpointLight {
                pub const m_flScale: usize = 0x1E0; // float32
                pub const m_nControlPoint1: usize = 0x670; // int32
                pub const m_nControlPoint2: usize = 0x674; // int32
                pub const m_nControlPoint3: usize = 0x678; // int32
                pub const m_nControlPoint4: usize = 0x67C; // int32
                pub const m_vecCPOffset1: usize = 0x680; // Vector
                pub const m_vecCPOffset2: usize = 0x68C; // Vector
                pub const m_vecCPOffset3: usize = 0x698; // Vector
                pub const m_vecCPOffset4: usize = 0x6A4; // Vector
                pub const m_LightFiftyDist1: usize = 0x6B0; // float32
                pub const m_LightZeroDist1: usize = 0x6B4; // float32
                pub const m_LightFiftyDist2: usize = 0x6B8; // float32
                pub const m_LightZeroDist2: usize = 0x6BC; // float32
                pub const m_LightFiftyDist3: usize = 0x6C0; // float32
                pub const m_LightZeroDist3: usize = 0x6C4; // float32
                pub const m_LightFiftyDist4: usize = 0x6C8; // float32
                pub const m_LightZeroDist4: usize = 0x6CC; // float32
                pub const m_LightColor1: usize = 0x6D0; // Color
                pub const m_LightColor2: usize = 0x6D4; // Color
                pub const m_LightColor3: usize = 0x6D8; // Color
                pub const m_LightColor4: usize = 0x6DC; // Color
                pub const m_bLightType1: usize = 0x6E0; // bool
                pub const m_bLightType2: usize = 0x6E1; // bool
                pub const m_bLightType3: usize = 0x6E2; // bool
                pub const m_bLightType4: usize = 0x6E3; // bool
                pub const m_bLightDynamic1: usize = 0x6E4; // bool
                pub const m_bLightDynamic2: usize = 0x6E5; // bool
                pub const m_bLightDynamic3: usize = 0x6E6; // bool
                pub const m_bLightDynamic4: usize = 0x6E7; // bool
                pub const m_bUseNormal: usize = 0x6E8; // bool
                pub const m_bUseHLambert: usize = 0x6E9; // bool
                pub const m_bClampLowerRange: usize = 0x6EE; // bool
                pub const m_bClampUpperRange: usize = 0x6EF; // bool
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
            pub mod C_OP_VectorFieldSnapshot {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nAttributeToWrite: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x1E8; // int32
                pub const m_flInterpolation: usize = 0x1F0; // CPerParticleFloatInput
                pub const m_vecScale: usize = 0x368; // CPerParticleVecInput
                pub const m_flBoundaryDampening: usize = 0xA40; // float32
                pub const m_bSetVelocity: usize = 0xA44; // bool
                pub const m_bLockToSurface: usize = 0xA45; // bool
                pub const m_flGridSpacing: usize = 0xA48; // float32
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
            pub mod C_OP_CylindricalDistanceToTransform {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x360; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4D8; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x650; // CPerParticleFloatInput
                pub const m_TransformStart: usize = 0x7C8; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x830; // CParticleTransformInput
                pub const m_nSetMethod: usize = 0x898; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x89C; // bool
                pub const m_bAdditive: usize = 0x89D; // bool
                pub const m_bCapsule: usize = 0x89E; // bool
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            pub mod C_INIT_PositionPlaceOnGround {
                pub const m_flOffset: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flMaxTraceLength: usize = 0x360; // CPerParticleFloatInput
                pub const m_vecTraceDir: usize = 0x4D8; // CPerParticleVecInput
                pub const m_CollisionGroupName: usize = 0xBB0; // char[128]
                pub const m_nTraceSet: usize = 0xC30; // ParticleTraceSet_t
                pub const m_nTraceMissBehavior: usize = 0xC40; // ParticleTraceMissBehavior_t
                pub const m_bIncludeWater: usize = 0xC44; // bool
                pub const m_nAttribute: usize = 0xC48; // ParticleAttributeIndex_t
                pub const m_bSetPXYZOnly: usize = 0xC4C; // bool
                pub const m_bSetNormal: usize = 0xC4D; // bool
                pub const m_nGroundNormalAttribute: usize = 0xC50; // ParticleAttributeIndex_t
                pub const m_bOffsetonColOnly: usize = 0xC54; // bool
                pub const m_flOffsetByRadiusFactor: usize = 0xC58; // float32
                pub const m_nPreserveOffsetCP: usize = 0xC5C; // int32
                pub const m_nIgnoreCP: usize = 0xC60; // int32
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
            pub mod C_INIT_RandomScalar {
                pub const m_flMin: usize = 0x1E8; // float32
                pub const m_flMax: usize = 0x1EC; // float32
                pub const m_flExponent: usize = 0x1F0; // float32
                pub const m_nFieldOutput: usize = 0x1F4; // ParticleAttributeIndex_t
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
            pub mod C_OP_RenderPostProcessing {
                pub const m_flPostProcessStrength: usize = 0x230; // CPerParticleFloatInput
                pub const m_hPostTexture: usize = 0x3A8; // CStrongHandle<InfoForResourceTypeCPostProcessingResource>
                pub const m_nPriority: usize = 0x3B0; // ParticlePostProcessPriorityGroup_t
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
            pub mod C_OP_OscillateScalar {
                pub const m_RateMin: usize = 0x1E0; // float32
                pub const m_RateMax: usize = 0x1E4; // float32
                pub const m_FrequencyMin: usize = 0x1E8; // float32
                pub const m_FrequencyMax: usize = 0x1EC; // float32
                pub const m_nField: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_bProportional: usize = 0x1F4; // bool
                pub const m_bProportionalOp: usize = 0x1F5; // bool
                pub const m_flStartTime_min: usize = 0x1F8; // float32
                pub const m_flStartTime_max: usize = 0x1FC; // float32
                pub const m_flEndTime_min: usize = 0x200; // float32
                pub const m_flEndTime_max: usize = 0x204; // float32
                pub const m_flOscMult: usize = 0x208; // float32
                pub const m_flOscAdd: usize = 0x20C; // float32
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
            pub mod C_OP_FadeOut {
                pub const m_flFadeOutTimeMin: usize = 0x1E0; // float32
                pub const m_flFadeOutTimeMax: usize = 0x1E4; // float32
                pub const m_flFadeOutTimeExp: usize = 0x1E8; // float32
                pub const m_flFadeBias: usize = 0x1EC; // float32
                pub const m_bProportional: usize = 0x220; // bool
                pub const m_bEaseInAndOut: usize = 0x221; // bool
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
            pub mod C_OP_WaterImpulseRenderer {
                pub const m_vecPos: usize = 0x230; // CPerParticleVecInput
                pub const m_flRadius: usize = 0x908; // CPerParticleFloatInput
                pub const m_flMagnitude: usize = 0xA80; // CPerParticleFloatInput
                pub const m_flShape: usize = 0xBF8; // CPerParticleFloatInput
                pub const m_flWindSpeed: usize = 0xD70; // CPerParticleFloatInput
                pub const m_flWobble: usize = 0xEE8; // CPerParticleFloatInput
                pub const m_bIsRadialWind: usize = 0x1060; // bool
                pub const m_nEventType: usize = 0x1064; // EventTypeSelection_t
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
            pub mod C_OP_RampScalarSplineSimple {
                pub const m_Rate: usize = 0x1E0; // float32
                pub const m_flStartTime: usize = 0x1E4; // float32
                pub const m_flEndTime: usize = 0x1E8; // float32
                pub const m_nField: usize = 0x210; // ParticleAttributeIndex_t
                pub const m_bEaseOut: usize = 0x214; // bool
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
            pub mod C_INIT_DistanceCull {
                pub const m_nControlPoint: usize = 0x1E8; // int32
                pub const m_flDistance: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_bCullInside: usize = 0x368; // bool
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
            pub mod C_INIT_InitFromVectorFieldSnapshot {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_nLocalSpaceCP: usize = 0x1EC; // int32
                pub const m_nWeightUpdateCP: usize = 0x1F0; // int32
                pub const m_bUseVerticalVelocity: usize = 0x1F4; // bool
                pub const m_vecScale: usize = 0x1F8; // CPerParticleVecInput
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
            pub mod C_OP_SetVectorAttributeToVectorExpression {
                pub const m_nExpression: usize = 0x1E0; // VectorExpressionType_t
                pub const m_vInput1: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x8C0; // CPerParticleVecInput
                pub const m_flLerp: usize = 0xF98; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x1110; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x1114; // ParticleSetMethod_t
                pub const m_bNormalizedOutput: usize = 0x1118; // bool
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
            pub mod C_INIT_AddVectorToVector {
                pub const m_vecScale: usize = 0x1E8; // Vector
                pub const m_nFieldOutput: usize = 0x1F4; // ParticleAttributeIndex_t
                pub const m_nFieldInput: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const m_vOffsetMin: usize = 0x1FC; // Vector
                pub const m_vOffsetMax: usize = 0x208; // Vector
                pub const m_randomnessParameters: usize = 0x214; // CRandomNumberGeneratorParameters
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
            pub mod C_INIT_RemapInitialVisibilityScalar {
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1F0; // float32
                pub const m_flInputMax: usize = 0x1F4; // float32
                pub const m_flOutputMin: usize = 0x1F8; // float32
                pub const m_flOutputMax: usize = 0x1FC; // float32
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
            pub mod C_OP_RemapTransformOrientationToYaw {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x248; // ParticleAttributeIndex_t
                pub const m_flRotOffset: usize = 0x24C; // float32
                pub const m_flSpinStrength: usize = 0x250; // float32
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
            pub mod C_OP_RenderStatusEffect {
                pub const m_pTextureColorWarp: usize = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureDetail2: usize = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureDiffuseWarp: usize = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureFresnelColorWarp: usize = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureFresnelWarp: usize = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureSpecularWarp: usize = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureEnvMap: usize = 0x260; // CStrongHandle<InfoForResourceTypeCTextureBase>
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
            pub mod C_OP_RandomForce {
                pub const m_MinForce: usize = 0x1F0; // Vector
                pub const m_MaxForce: usize = 0x1FC; // Vector
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
            pub mod C_OP_RemapParticleCountOnScalarEndCap {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nInputMin: usize = 0x1E4; // int32
                pub const m_nInputMax: usize = 0x1E8; // int32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_bBackwards: usize = 0x1F4; // bool
                pub const m_nSetMethod: usize = 0x1F8; // ParticleSetMethod_t
            }
            // Parent: None
            // Field count: 18
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // INHERITABLE_BOOL_FALSE
            // INHERITABLE_BOOL_TRUE
            pub mod ParticlePreviewState_t {
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
            pub mod C_OP_LocalAccelerationForce {
                pub const m_nCP: usize = 0x1F0; // int32
                pub const m_nScaleCP: usize = 0x1F4; // int32
                pub const m_vecAccel: usize = 0x1F8; // CParticleCollectionVecInput
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
            pub mod C_OP_ModelCull {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_bBoundBox: usize = 0x1E4; // bool
                pub const m_bCullOutside: usize = 0x1E5; // bool
                pub const m_bUseBones: usize = 0x1E6; // bool
                pub const m_HitboxSetName: usize = 0x1E7; // char[128]
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
            pub mod C_OP_SetFloat {
                pub const m_InputValue: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x358; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x35C; // ParticleSetMethod_t
                pub const m_Lerp: usize = 0x360; // CPerParticleFloatInput
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
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
            pub mod C_INIT_RemapTransformToVector {
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_vInputMin: usize = 0x1EC; // Vector
                pub const m_vInputMax: usize = 0x1F8; // Vector
                pub const m_vOutputMin: usize = 0x204; // Vector
                pub const m_vOutputMax: usize = 0x210; // Vector
                pub const m_TransformInput: usize = 0x220; // CParticleTransformInput
                pub const m_LocalSpaceTransform: usize = 0x288; // CParticleTransformInput
                pub const m_flStartTime: usize = 0x2F0; // float32
                pub const m_flEndTime: usize = 0x2F4; // float32
                pub const m_nSetMethod: usize = 0x2F8; // ParticleSetMethod_t
                pub const m_bOffset: usize = 0x2FC; // bool
                pub const m_bAccelerate: usize = 0x2FD; // bool
                pub const m_flRemapBias: usize = 0x300; // float32
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
            pub mod C_OP_ScreenSpaceDistanceToEdge {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flMaxDistFromEdge: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flOutputRemap: usize = 0x360; // CParticleRemapFloatInput
                pub const m_nSetMethod: usize = 0x4D8; // ParticleSetMethod_t
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
            pub mod C_OP_RenderStatusEffectTf {
                pub const m_pTextureColorWarp: usize = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureNormal: usize = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureMetalness: usize = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureRoughness: usize = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureSelfIllum: usize = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureDetail: usize = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureEnvMap: usize = 0x260; // CStrongHandle<InfoForResourceTypeCTextureBase>
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
            pub mod C_OP_RemapVectortoCP {
                pub const m_nOutControlPointNumber: usize = 0x1E0; // int32
                pub const m_nFieldInput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nParticleNumber: usize = 0x1E8; // int32
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
            pub mod C_OP_SetFromCPSnapshot {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_strSnapshotSubset: usize = 0x1E8; // CUtlString
                pub const m_nAttributeToRead: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_nAttributeToWrite: usize = 0x1F4; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x1F8; // int32
                pub const m_bRandom: usize = 0x1FC; // bool
                pub const m_bReverse: usize = 0x1FD; // bool
                pub const m_nRandomSeed: usize = 0x200; // int32
                pub const m_nSnapShotStartPoint: usize = 0x208; // CParticleCollectionFloatInput
                pub const m_nSnapShotIncrement: usize = 0x380; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0x4F8; // CPerParticleFloatInput
                pub const m_bSubSample: usize = 0x670; // bool
                pub const m_bPrev: usize = 0x671; // bool
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
            pub mod C_OP_DistanceBetweenCPsToCP {
                pub const m_nStartCP: usize = 0x1E8; // int32
                pub const m_nEndCP: usize = 0x1EC; // int32
                pub const m_nOutputCP: usize = 0x1F0; // int32
                pub const m_nOutputCPField: usize = 0x1F4; // int32
                pub const m_bSetOnce: usize = 0x1F8; // bool
                pub const m_flInputMin: usize = 0x1FC; // float32
                pub const m_flInputMax: usize = 0x200; // float32
                pub const m_flOutputMin: usize = 0x204; // float32
                pub const m_flOutputMax: usize = 0x208; // float32
                pub const m_flMaxTraceLength: usize = 0x20C; // float32
                pub const m_flLOSScale: usize = 0x210; // float32
                pub const m_bLOS: usize = 0x214; // bool
                pub const m_CollisionGroupName: usize = 0x215; // char[128]
                pub const m_nTraceSet: usize = 0x298; // ParticleTraceSet_t
                pub const m_nSetParent: usize = 0x29C; // ParticleParentSetMode_t
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
            pub mod C_OP_SetControlPointToHand {
                pub const m_nCP1: usize = 0x1E8; // int32
                pub const m_nHand: usize = 0x1EC; // int32
                pub const m_vecCP1Pos: usize = 0x1F0; // Vector
                pub const m_bOrientToHand: usize = 0x1FC; // bool
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
            pub mod C_OP_DistanceCull {
                pub const m_nControlPoint: usize = 0x1E0; // int32
                pub const m_vecPointOffset: usize = 0x1E4; // Vector
                pub const m_flDistance: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_bCullInside: usize = 0x368; // bool
                pub const m_nAttribute: usize = 0x36C; // ParticleAttributeIndex_t
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
            pub mod C_INIT_CreateAlongPath {
                pub const m_fMaxDistance: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_fT: usize = 0x360; // CPerParticleFloatInput
                pub const m_PathParams: usize = 0x4E0; // CPathParameters
                pub const m_bUseRandomCPs: usize = 0x520; // bool
                pub const m_vEndOffset: usize = 0x524; // Vector
                pub const m_bSaveOffset: usize = 0x530; // bool
                pub const m_nXCount: usize = 0x1E8; // CParticleCollectionFloatInput
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
            pub mod C_OP_SetControlPointsToModelParticles {
                pub const m_HitboxSetName: usize = 0x1E0; // char[128]
                pub const m_AttachmentName: usize = 0x260; // char[128]
                pub const m_nFirstControlPoint: usize = 0x2E0; // int32
                pub const m_nNumControlPoints: usize = 0x2E4; // int32
                pub const m_nFirstSourcePoint: usize = 0x2E8; // int32
                pub const m_bSkin: usize = 0x2EC; // bool
                pub const m_bAttachment: usize = 0x2ED; // bool
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
            pub mod C_OP_ColorInterpolateRandom {
                pub const m_ColorFadeMin: usize = 0x1E0; // Color
                pub const m_ColorFadeMax: usize = 0x1FC; // Color
                pub const m_flFadeStartTime: usize = 0x20C; // float32
                pub const m_flFadeEndTime: usize = 0x210; // float32
                pub const m_nFieldOutput: usize = 0x214; // ParticleAttributeIndex_t
                pub const m_bEaseInOut: usize = 0x218; // bool
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
            pub mod C_INIT_RemapNamedModelSequenceToScalar {
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
            pub mod C_OP_RenderLights {
                pub const m_flAnimationRate: usize = 0x238; // float32
                pub const m_nAnimationType: usize = 0x23C; // AnimationType_t
                pub const m_bAnimateInFPS: usize = 0x240; // bool
                pub const m_flMinSize: usize = 0x244; // float32
                pub const m_flMaxSize: usize = 0x248; // float32
                pub const m_flStartFadeSize: usize = 0x24C; // float32
                pub const m_flEndFadeSize: usize = 0x250; // float32
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
            pub mod C_OP_DecayClampCount {
                pub const m_nCount: usize = 0x1E0; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CRandomNumberGeneratorParameters {
                pub const m_bDistributeEvenly: usize = 0x0; // bool
                pub const m_nSeed: usize = 0x4; // int32
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
            pub mod C_INIT_ColorLitPerParticle {
                pub const m_ColorMin: usize = 0x200; // Color
                pub const m_ColorMax: usize = 0x204; // Color
                pub const m_TintMin: usize = 0x208; // Color
                pub const m_TintMax: usize = 0x20C; // Color
                pub const m_flTintPerc: usize = 0x210; // float32
                pub const m_nTintBlendMode: usize = 0x214; // ParticleColorBlendMode_t
                pub const m_flLightAmplification: usize = 0x218; // float32
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
            pub mod C_INIT_CreateOnGrid {
                pub const m_nXCount: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_nYCount: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_nZCount: usize = 0x4D8; // CParticleCollectionFloatInput
                pub const m_nXSpacing: usize = 0x650; // CParticleCollectionFloatInput
                pub const m_nYSpacing: usize = 0x7C8; // CParticleCollectionFloatInput
                pub const m_nZSpacing: usize = 0x940; // CParticleCollectionFloatInput
                pub const m_nControlPointNumber: usize = 0xAB8; // int32
                pub const m_bLocalSpace: usize = 0xABC; // bool
                pub const m_bCenter: usize = 0xABD; // bool
                pub const m_bHollow: usize = 0xABE; // bool
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
            pub mod C_OP_RampCPLinearRandom {
                pub const m_nOutControlPointNumber: usize = 0x1E8; // int32
                pub const m_vecRateMin: usize = 0x1EC; // Vector
                pub const m_vecRateMax: usize = 0x1F8; // Vector
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
            pub mod C_OP_VelocityMatchingForce {
                pub const m_flDirScale: usize = 0x1E0; // float32
                pub const m_flSpdScale: usize = 0x1E4; // float32
                pub const m_flNeighborDistance: usize = 0x1E8; // float32
                pub const m_flFacingStrength: usize = 0x1EC; // float32
                pub const m_bUseAABB: usize = 0x1F0; // bool
                pub const m_nCPBroadcast: usize = 0x1F4; // int32
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
            pub mod C_INIT_RandomAlphaWindowThreshold {
                pub const m_flMin: usize = 0x1E8; // float32
                pub const m_flMax: usize = 0x1EC; // float32
                pub const m_flExponent: usize = 0x1F0; // float32
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
            pub mod C_INIT_CreateOnModelAtHeight {
                pub const m_bUseBones: usize = 0x1E8; // bool
                pub const m_bForceZ: usize = 0x1E9; // bool
                pub const m_nControlPointNumber: usize = 0x1EC; // int32
                pub const m_nHeightCP: usize = 0x1F0; // int32
                pub const m_bUseWaterHeight: usize = 0x1F4; // bool
                pub const m_flDesiredHeight: usize = 0x1F8; // CParticleCollectionFloatInput
                pub const m_vecHitBoxScale: usize = 0x370; // CParticleCollectionVecInput
                pub const m_vecDirectionBias: usize = 0xA48; // CParticleCollectionVecInput
                pub const m_nBiasType: usize = 0x1120; // ParticleHitboxBiasType_t
                pub const m_bLocalCoords: usize = 0x1124; // bool
                pub const m_bPreferMovingBoxes: usize = 0x1125; // bool
                pub const m_HitboxSetName: usize = 0x1126; // char[128]
                pub const m_flHitboxVelocityScale: usize = 0x11A8; // CParticleCollectionFloatInput
                pub const m_flMaxBoneVelocity: usize = 0x1320; // CParticleCollectionFloatInput
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
            pub mod C_OP_ModelSurfaceSnapshotGenerator {
                pub const m_nCPSnapshot: usize = 0x1E8; // int32
                pub const m_modelInput: usize = 0x1F0; // CParticleModelInput
                pub const m_flRecalcRate: usize = 0x250; // CParticleCollectionFloatInput
                pub const m_flUSpacing: usize = 0x3C8; // CParticleCollectionFloatInput
                pub const m_flVSpacing: usize = 0x540; // CParticleCollectionFloatInput
                pub const m_flSurfaceOffset: usize = 0x6B8; // CParticleCollectionFloatInput
                pub const m_bSetNormal: usize = 0x830; // bool
                pub const m_bSetUp: usize = 0x831; // bool
                pub const m_bSetGravity: usize = 0x832; // bool
                pub const m_bSetUV: usize = 0x833; // bool
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
            pub mod C_OP_RestartAfterDuration {
                pub const m_flDurationMin: usize = 0x1E0; // float32
                pub const m_flDurationMax: usize = 0x1E4; // float32
                pub const m_nCP: usize = 0x1E8; // int32
                pub const m_nCPField: usize = 0x1EC; // int32
                pub const m_nChildGroupID: usize = 0x1F0; // int32
                pub const m_bOnlyChildren: usize = 0x1F4; // bool
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
            pub mod C_OP_RenderClothForce {
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
            pub mod C_OP_RemapVisibilityScalar {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // float32
                pub const m_flInputMax: usize = 0x1EC; // float32
                pub const m_flOutputMin: usize = 0x1F0; // float32
                pub const m_flOutputMax: usize = 0x1F4; // float32
                pub const m_flRadiusScale: usize = 0x1F8; // float32
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
            pub mod C_INIT_CreateSequentialPathV2 {
                pub const m_fMaxDistance: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flNumToAssign: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_bLoop: usize = 0x4D8; // bool
                pub const m_bCPPairs: usize = 0x4D9; // bool
                pub const m_bSaveOffset: usize = 0x4DA; // bool
                pub const m_PathParams: usize = 0x4E0; // CPathParameters
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
            pub mod VecInputMaterialVariable_t {
                pub const m_strVariable: usize = 0x0; // CUtlString
                pub const m_vecInput: usize = 0x8; // CParticleCollectionVecInput
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
            pub mod C_INIT_RemapInitialDirectionToTransformToVector {
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x250; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x254; // float32
                pub const m_flOffsetRot: usize = 0x258; // float32
                pub const m_vecOffsetAxis: usize = 0x25C; // Vector
                pub const m_bNormalize: usize = 0x268; // bool
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
            pub mod C_OP_LockToSavedSequentialPathV2 {
                pub const m_flFadeStart: usize = 0x1E0; // float32
                pub const m_flFadeEnd: usize = 0x1E4; // float32
                pub const m_bCPPairs: usize = 0x1E8; // bool
                pub const m_PathParams: usize = 0x1F0; // CPathParameters
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
            pub mod C_OP_NormalLock {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
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
            pub mod C_INIT_RemapTransformOrientationToRotations {
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_vecRotation: usize = 0x250; // Vector
                pub const m_bUseQuat: usize = 0x25C; // bool
                pub const m_bWriteNormal: usize = 0x25D; // bool
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
            pub mod C_OP_Cull {
                pub const m_flCullPerc: usize = 0x1E0; // float32
                pub const m_flCullStart: usize = 0x1E4; // float32
                pub const m_flCullEnd: usize = 0x1E8; // float32
                pub const m_flCullExp: usize = 0x1EC; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            pub mod SequenceWeightedList_t {
                pub const m_nSequence: usize = 0x0; // int32
                pub const m_flRelativeWeight: usize = 0x4; // float32
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
            pub mod C_OP_ReadFromNeighboringParticle {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x1E8; // int32
                pub const m_DistanceCheck: usize = 0x1F0; // CPerParticleFloatInput
                pub const m_flInterpolation: usize = 0x368; // CPerParticleFloatInput
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
            pub mod C_OP_RenderText {
                pub const m_OutlineColor: usize = 0x230; // Color
                pub const m_DefaultText: usize = 0x238; // CUtlString
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
            pub mod C_OP_LerpToInitialPosition {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_flInterpolation: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_nCacheField: usize = 0x360; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x368; // CParticleCollectionFloatInput
                pub const m_vecScale: usize = 0x4E0; // CParticleCollectionVecInput
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
            pub mod C_INIT_RandomRotation {
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
            pub mod C_OP_LerpEndCapVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vecOutput: usize = 0x1E4; // Vector
                pub const m_flLerpTime: usize = 0x1F0; // float32
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
            pub mod C_OP_VelocityDecay {
                pub const m_flMinVelocity: usize = 0x1E0; // float32
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
            pub mod C_OP_SetCPOrientationToPointAtCP {
                pub const m_nInputCP: usize = 0x1E8; // int32
                pub const m_nOutputCP: usize = 0x1EC; // int32
                pub const m_flInterpolation: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_b2DOrientation: usize = 0x368; // bool
                pub const m_bAvoidSingularity: usize = 0x369; // bool
                pub const m_bPointAway: usize = 0x36A; // bool
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
            pub mod C_OP_LockToPointList {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_pointList: usize = 0x1E8; // CUtlVector<PointDefinition_t>
                pub const m_bPlaceAlongPath: usize = 0x200; // bool
                pub const m_bClosedLoop: usize = 0x201; // bool
                pub const m_nNumPointsAlongPath: usize = 0x204; // int32
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
            pub mod C_OP_MovementPlaceOnGround {
                pub const m_flOffset: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flMaxTraceLength: usize = 0x358; // float32
                pub const m_flTolerance: usize = 0x35C; // float32
                pub const m_vecTraceDir: usize = 0x360; // CPerParticleVecInput
                pub const m_flTraceOffset: usize = 0xA38; // float32
                pub const m_flLerpRate: usize = 0xA3C; // float32
                pub const m_CollisionGroupName: usize = 0xA40; // char[128]
                pub const m_nTraceSet: usize = 0xAC0; // ParticleTraceSet_t
                pub const m_nRefCP1: usize = 0xAC4; // int32
                pub const m_nRefCP2: usize = 0xAC8; // int32
                pub const m_nLerpCP: usize = 0xACC; // int32
                pub const m_nTraceMissBehavior: usize = 0xAD8; // ParticleTraceMissBehavior_t
                pub const m_bIncludeShotHull: usize = 0xADC; // bool
                pub const m_bIncludeWater: usize = 0xADD; // bool
                pub const m_bSetNormal: usize = 0xAE0; // bool
                pub const m_bScaleOffset: usize = 0xAE1; // bool
                pub const m_nPreserveOffsetCP: usize = 0xAE4; // int32
                pub const m_nIgnoreCP: usize = 0xAE8; // int32
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
            pub mod C_OP_SetCPOrientationToDirection {
                pub const m_nInputControlPoint: usize = 0x1E0; // int32
                pub const m_nOutputControlPoint: usize = 0x1E4; // int32
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
            pub mod C_OP_RemapCrossProductOfTwoVectorsToVector {
                pub const m_InputVec1: usize = 0x1E0; // CPerParticleVecInput
                pub const m_InputVec2: usize = 0x8B8; // CPerParticleVecInput
                pub const m_nFieldOutput: usize = 0xF90; // ParticleAttributeIndex_t
                pub const m_bNormalize: usize = 0xF94; // bool
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
            pub mod C_OP_RemapTransformOrientationToRotations {
                pub const m_TransformInput: usize = 0x1E0; // CParticleTransformInput
                pub const m_vecRotation: usize = 0x248; // Vector
                pub const m_bUseQuat: usize = 0x254; // bool
                pub const m_bWriteNormal: usize = 0x255; // bool
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
            pub mod C_INIT_RandomRotationSpeed {
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
            pub mod C_OP_InheritFromParentParticlesV2 {
                pub const m_flScale: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nFieldOutput: usize = 0x358; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x360; // CPerParticleFloatInput
                pub const m_bSubSample: usize = 0x4D8; // bool
                pub const m_bRandomDistribution: usize = 0x4D9; // bool
                pub const m_bReverse: usize = 0x4DA; // bool
                pub const m_nMissingParentBehavior: usize = 0x4DC; // MissingParentInheritBehavior_t
                pub const m_flInterpolation: usize = 0x4E0; // CPerParticleFloatInput
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
            pub mod C_INIT_RandomSecondSequence {
                pub const m_nSequenceMin: usize = 0x1E8; // int32
                pub const m_nSequenceMax: usize = 0x1EC; // int32
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
            pub mod C_OP_SetFloatCollection {
                pub const m_InputValue: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_nOutputField: usize = 0x358; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x35C; // ParticleSetMethod_t
                pub const m_Lerp: usize = 0x360; // CParticleCollectionFloatInput
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertySuppressExpr
            pub mod PointDefinition_t {
                pub const m_nControlPoint: usize = 0x0; // int32
                pub const m_bLocalCoords: usize = 0x4; // bool
                pub const m_vOffset: usize = 0x8; // Vector
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
            pub mod C_OP_Diffusion {
                pub const m_flRadiusScale: usize = 0x1E0; // float32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nVoxelGridResolution: usize = 0x1E8; // int32
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
            pub mod C_INIT_AgeNoise {
                pub const m_bAbsVal: usize = 0x1E8; // bool
                pub const m_bAbsValInv: usize = 0x1E9; // bool
                pub const m_flOffset: usize = 0x1EC; // float32
                pub const m_flAgeMin: usize = 0x1F0; // float32
                pub const m_flAgeMax: usize = 0x1F4; // float32
                pub const m_flNoiseScale: usize = 0x1F8; // float32
                pub const m_flNoiseScaleLoc: usize = 0x1FC; // float32
                pub const m_vecOffsetLoc: usize = 0x200; // Vector
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
            pub mod C_OP_RemapVectorComponentToScalar {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nComponent: usize = 0x1E8; // int32
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
            pub mod CGeneralRandomRotation {
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_flDegrees: usize = 0x1EC; // float32
                pub const m_flDegreesMin: usize = 0x1F0; // float32
                pub const m_flDegreesMax: usize = 0x1F4; // float32
                pub const m_flRotationRandExponent: usize = 0x1F8; // float32
                pub const m_bRandomlyFlipDirection: usize = 0x1FC; // bool
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
            pub mod C_OP_DistanceBetweenVecs {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vecPoint1: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vecPoint2: usize = 0x8C0; // CPerParticleVecInput
                pub const m_flInputMin: usize = 0xF98; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x1110; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x1288; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x1400; // CPerParticleFloatInput
                pub const m_nSetMethod: usize = 0x1578; // ParticleSetMethod_t
                pub const m_bDeltaTime: usize = 0x157C; // bool
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
            pub mod C_OP_DampenToCP {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_flRange: usize = 0x1E4; // float32
                pub const m_flScale: usize = 0x1E8; // float32
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
            pub mod C_OP_RemapNamedModelBodyPartOnceTimed {
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
            pub mod C_OP_ScreenSpaceRotateTowardTarget {
                pub const m_vecTargetPosition: usize = 0x1E0; // CPerParticleVecInput
                pub const m_flOutputRemap: usize = 0x8B8; // CParticleRemapFloatInput
                pub const m_nSetMethod: usize = 0xA30; // ParticleSetMethod_t
                pub const m_flScreenEdgeAlignmentDistance: usize = 0xA38; // CPerParticleFloatInput
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
            pub mod C_OP_MovementMaintainOffset {
                pub const m_vecOffset: usize = 0x1E0; // Vector
                pub const m_nCP: usize = 0x1EC; // int32
                pub const m_bRadiusScale: usize = 0x1F0; // bool
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
            pub mod C_INIT_CreateWithinCapsuleTransform {
                pub const m_fRadiusMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_fRadiusMax: usize = 0x360; // CPerParticleFloatInput
                pub const m_fHeight: usize = 0x4D8; // CPerParticleFloatInput
                pub const m_TransformInput: usize = 0x650; // CParticleTransformInput
                pub const m_fSpeedMin: usize = 0x6B8; // CPerParticleFloatInput
                pub const m_fSpeedMax: usize = 0x830; // CPerParticleFloatInput
                pub const m_fSpeedRandExp: usize = 0x9A8; // float32
                pub const m_LocalCoordinateSystemSpeedMin: usize = 0x9B0; // CPerParticleVecInput
                pub const m_LocalCoordinateSystemSpeedMax: usize = 0x1088; // CPerParticleVecInput
                pub const m_nFieldOutput: usize = 0x1760; // ParticleAttributeIndex_t
                pub const m_nFieldVelocity: usize = 0x1764; // ParticleAttributeIndex_t
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
            pub mod C_OP_SetVec {
                pub const m_InputValue: usize = 0x1E0; // CPerParticleVecInput
                pub const m_nOutputField: usize = 0x8B8; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x8BC; // ParticleSetMethod_t
                pub const m_Lerp: usize = 0x8C0; // CPerParticleFloatInput
                pub const m_bNormalizedOutput: usize = 0xA38; // bool
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
            pub mod C_INIT_CreateFromParentParticles {
                pub const m_flVelocityScale: usize = 0x1E8; // float32
                pub const m_flIncrement: usize = 0x1EC; // float32
                pub const m_bRandomDistribution: usize = 0x1F0; // bool
                pub const m_nRandomSeed: usize = 0x1F4; // int32
                pub const m_bSubFrame: usize = 0x1F8; // bool
                pub const m_bSetRopeSegmentID: usize = 0x1F9; // bool
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
            pub mod C_INIT_CheckParticleForWater {
                pub const m_flRadius: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_nFieldOutput: usize = 0x360; // ParticleAttributeIndex_t
                pub const m_flOutputRemap: usize = 0x368; // CParticleRemapFloatInput
                pub const m_nSetMethod: usize = 0x4E0; // ParticleSetMethod_t
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
            pub mod C_INIT_RandomNamedModelBodyPart {
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
            pub mod C_OP_RenderOmni2Light {
                pub const m_nLightType: usize = 0x230; // ParticleOmni2LightTypeChoiceList_t
                pub const m_nMaxAllowed: usize = 0x234; // uint16
                pub const m_vColorBlend: usize = 0x238; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0x910; // ParticleColorBlendType_t
                pub const m_strLightStyle: usize = 0x918; // CUtlString
                pub const m_flLightStyleTime: usize = 0x920; // CPerParticleFloatInput
                pub const m_nBrightnessUnit: usize = 0xA98; // ParticleLightUnitChoiceList_t
                pub const m_flBrightnessLumens: usize = 0xAA0; // CPerParticleFloatInput
                pub const m_flBrightnessCandelas: usize = 0xC18; // CPerParticleFloatInput
                pub const m_bCastShadows: usize = 0xD90; // bool
                pub const m_bDynamicBounce: usize = 0xD91; // bool
                pub const m_flBounceScale: usize = 0xD98; // CParticleCollectionFloatInput
                pub const m_bFog: usize = 0xF10; // bool
                pub const m_flFogScale: usize = 0xF18; // CPerParticleFloatInput
                pub const m_flLuminaireRadius: usize = 0x1090; // CPerParticleFloatInput
                pub const m_nOrientationType: usize = 0x1208; // ParticleOmni2LighOrientationChoiceList_t
                pub const m_vNormal: usize = 0x1210; // CPerParticleVecInput
                pub const m_vTarget: usize = 0x18E8; // CPerParticleVecInput
                pub const m_flFOVAngle: usize = 0x1FC0; // CPerParticleFloatInput
                pub const m_flBarnShape: usize = 0x2138; // CPerParticleFloatInput
                pub const m_flBarnNearSizeX: usize = 0x22B0; // CPerParticleFloatInput
                pub const m_flBarnNearSizeY: usize = 0x2428; // CPerParticleFloatInput
                pub const m_flBarnSoftX: usize = 0x25A0; // CPerParticleFloatInput
                pub const m_flBarnSoftY: usize = 0x2718; // CPerParticleFloatInput
                pub const m_flSkirt: usize = 0x2890; // CPerParticleFloatInput
                pub const m_flRange: usize = 0x2A08; // CPerParticleFloatInput
                pub const m_flInnerConeAngle: usize = 0x2B80; // CPerParticleFloatInput
                pub const m_flOuterConeAngle: usize = 0x2CF8; // CPerParticleFloatInput
                pub const m_hLightCookie: usize = 0x2E70; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_bSphericalCookie: usize = 0x2E78; // bool
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            pub mod CPAssignment_t {
                pub const m_nCPNumber: usize = 0x0; // int32
                pub const m_Pos: usize = 0x8; // CPerParticleVecInput
                pub const m_nOrientationMode: usize = 0x6E0; // ParticleOrientationSetMode_t
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
            pub mod C_INIT_RemapParticleCountToNamedModelBodyPartScalar {
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
            pub mod C_OP_LagCompensation {
                pub const m_nDesiredVelocityCP: usize = 0x1E0; // int32
                pub const m_nLatencyCP: usize = 0x1E4; // int32
                pub const m_nLatencyCPField: usize = 0x1E8; // int32
                pub const m_nDesiredVelocityCPField: usize = 0x1EC; // int32
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
            pub mod C_OP_CollideWithSelf {
                pub const m_flRadiusScale: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flMinimumSpeed: usize = 0x358; // CPerParticleFloatInput
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
            pub mod C_OP_Noise {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flOutputMin: usize = 0x1E4; // float32
                pub const m_flOutputMax: usize = 0x1E8; // float32
                pub const m_fl4NoiseScale: usize = 0x1EC; // float32
                pub const m_bAdditive: usize = 0x1F0; // bool
                pub const m_flNoiseAnimationTimeScale: usize = 0x1F4; // float32
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
            pub mod C_OP_FadeAndKillForTracers {
                pub const m_flStartFadeInTime: usize = 0x1E0; // float32
                pub const m_flEndFadeInTime: usize = 0x1E4; // float32
                pub const m_flStartFadeOutTime: usize = 0x1E8; // float32
                pub const m_flEndFadeOutTime: usize = 0x1EC; // float32
                pub const m_flStartAlpha: usize = 0x1F0; // float32
                pub const m_flEndAlpha: usize = 0x1F4; // float32
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
            pub mod CParticleMassCalculationParameters {
                pub const m_nMassMode: usize = 0x0; // ParticleMassMode_t
                pub const m_flRadius: usize = 0x8; // CPerParticleFloatInput
                pub const m_flNominalRadius: usize = 0x180; // CPerParticleFloatInput
                pub const m_flScale: usize = 0x2F8; // CPerParticleFloatInput
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
            pub mod C_OP_SequenceFromModel {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nFieldOutputAnim: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1EC; // float32
                pub const m_flInputMax: usize = 0x1F0; // float32
                pub const m_flOutputMin: usize = 0x1F4; // float32
                pub const m_flOutputMax: usize = 0x1F8; // float32
                pub const m_nSetMethod: usize = 0x1FC; // ParticleSetMethod_t
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
            pub mod C_OP_AlphaDecay {
                pub const m_flMinAlpha: usize = 0x1E0; // float32
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
            pub mod C_INIT_InitVec {
                pub const m_InputValue: usize = 0x1E8; // CPerParticleVecInput
                pub const m_nOutputField: usize = 0x8C0; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x8C4; // ParticleSetMethod_t
                pub const m_bNormalizedOutput: usize = 0x8C8; // bool
                pub const m_bWritePreviousPosition: usize = 0x8C9; // bool
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
            pub mod C_INIT_SetHitboxToModel {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_nForceInModel: usize = 0x1EC; // int32
                pub const m_bEvenDistribution: usize = 0x1F0; // bool
                pub const m_nDesiredHitbox: usize = 0x1F4; // int32
                pub const m_vecHitBoxScale: usize = 0x1F8; // CParticleCollectionVecInput
                pub const m_vecDirectionBias: usize = 0x8D0; // Vector
                pub const m_bMaintainHitbox: usize = 0x8DC; // bool
                pub const m_bUseBones: usize = 0x8DD; // bool
                pub const m_HitboxSetName: usize = 0x8DE; // char[128]
                pub const m_flShellSize: usize = 0x960; // CParticleCollectionFloatInput
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
            pub mod C_OP_MovementMoveAlongSkinnedCPSnapshot {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nSnapshotControlPointNumber: usize = 0x1E4; // int32
                pub const m_bSetNormal: usize = 0x1E8; // bool
                pub const m_bSetRadius: usize = 0x1E9; // bool
                pub const m_flInterpolation: usize = 0x1F0; // CPerParticleFloatInput
                pub const m_flTValue: usize = 0x368; // CPerParticleFloatInput
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
            pub mod C_OP_LerpScalar {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flOutput: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flStartTime: usize = 0x360; // float32
                pub const m_flEndTime: usize = 0x364; // float32
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
            pub mod C_INIT_InitialRepulsionVelocity {
                pub const m_CollisionGroupName: usize = 0x1E8; // char[128]
                pub const m_nTraceSet: usize = 0x268; // ParticleTraceSet_t
                pub const m_vecOutputMin: usize = 0x26C; // Vector
                pub const m_vecOutputMax: usize = 0x278; // Vector
                pub const m_nControlPointNumber: usize = 0x284; // int32
                pub const m_bPerParticle: usize = 0x288; // bool
                pub const m_bTranslate: usize = 0x289; // bool
                pub const m_bProportional: usize = 0x28A; // bool
                pub const m_flTraceLength: usize = 0x28C; // float32
                pub const m_bPerParticleTR: usize = 0x290; // bool
                pub const m_bInherit: usize = 0x291; // bool
                pub const m_nChildCP: usize = 0x294; // int32
                pub const m_nChildGroupID: usize = 0x298; // int32
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
            pub mod C_OP_ClampScalar {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flOutputMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x360; // CPerParticleFloatInput
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
            pub mod C_OP_SetControlPointToHMD {
                pub const m_nCP1: usize = 0x1E8; // int32
                pub const m_vecCP1Pos: usize = 0x1EC; // Vector
                pub const m_bOrientToHMD: usize = 0x1F8; // bool
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
            pub mod C_OP_DifferencePreviousParticle {
                pub const m_nFieldInput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // float32
                pub const m_flInputMax: usize = 0x1EC; // float32
                pub const m_flOutputMin: usize = 0x1F0; // float32
                pub const m_flOutputMax: usize = 0x1F4; // float32
                pub const m_nSetMethod: usize = 0x1F8; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x1FC; // bool
                pub const m_bSetPreviousParticle: usize = 0x1FD; // bool
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
            pub mod C_OP_PercentageBetweenTransforms {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_TransformStart: usize = 0x1F8; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x260; // CParticleTransformInput
                pub const m_nSetMethod: usize = 0x2C8; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x2CC; // bool
                pub const m_bRadialCheck: usize = 0x2CD; // bool
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
            pub mod C_INIT_PlaneCull {
                pub const m_nControlPoint: usize = 0x1E8; // int32
                pub const m_flDistance: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_bCullInside: usize = 0x368; // bool
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
            pub mod C_OP_RemapNamedModelSequenceEndCap {
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
            pub mod C_INIT_InitFromCPSnapshot {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_strSnapshotSubset: usize = 0x1F0; // CUtlString
                pub const m_nAttributeToRead: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const m_nAttributeToWrite: usize = 0x1FC; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x200; // int32
                pub const m_bRandom: usize = 0x204; // bool
                pub const m_bReverse: usize = 0x205; // bool
                pub const m_nSnapShotIncrement: usize = 0x208; // CParticleCollectionFloatInput
                pub const m_bResetSnapshotIndexOnChanges: usize = 0x380; // bool
                pub const m_nManualSnapshotIndex: usize = 0x388; // CPerParticleFloatInput
                pub const m_nRandomSeed: usize = 0x500; // int32
                pub const m_bLocalSpaceAngles: usize = 0x504; // bool
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
            pub mod C_OP_RenderCables {
                pub const m_flRadiusScale: usize = 0x230; // CParticleCollectionFloatInput
                pub const m_flAlphaScale: usize = 0x3A8; // CParticleCollectionFloatInput
                pub const m_vecColorScale: usize = 0x520; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0xBF8; // ParticleColorBlendType_t
                pub const m_hMaterial: usize = 0xC00; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_nTextureRepetitionMode: usize = 0xC08; // TextureRepetitionMode_t
                pub const m_flTextureRepeatsPerSegment: usize = 0xC10; // CParticleCollectionFloatInput
                pub const m_flTextureRepeatsCircumference: usize = 0xD88; // CParticleCollectionFloatInput
                pub const m_flColorMapOffsetV: usize = 0xF00; // CParticleCollectionFloatInput
                pub const m_flColorMapOffsetU: usize = 0x1078; // CParticleCollectionFloatInput
                pub const m_flNormalMapOffsetV: usize = 0x11F0; // CParticleCollectionFloatInput
                pub const m_flNormalMapOffsetU: usize = 0x1368; // CParticleCollectionFloatInput
                pub const m_bDrawCableCaps: usize = 0x14E0; // bool
                pub const m_flCapRoundness: usize = 0x14E4; // float32
                pub const m_flCapOffsetAmount: usize = 0x14E8; // float32
                pub const m_flTessScale: usize = 0x14EC; // float32
                pub const m_nMinTesselation: usize = 0x14F0; // int32
                pub const m_nMaxTesselation: usize = 0x14F4; // int32
                pub const m_nRoundness: usize = 0x14F8; // int32
                pub const m_nForceRoundnessFixed: usize = 0x14FC; // bool
                pub const m_bOnlyRenderInEffectsBloomPass: usize = 0x14FD; // bool
                pub const m_LightingTransform: usize = 0x1500; // CParticleTransformInput
                pub const m_MaterialFloatVars: usize = 0x1568; // CUtlLeanVector<FloatInputMaterialVariable_t>
                pub const m_MaterialVecVars: usize = 0x1588; // CUtlLeanVector<VecInputMaterialVariable_t>
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
            pub mod C_OP_SetChildControlPoints {
                pub const m_nChildGroupID: usize = 0x1E0; // int32
                pub const m_nFirstControlPoint: usize = 0x1E4; // int32
                pub const m_nNumControlPoints: usize = 0x1E8; // int32
                pub const m_nFirstSourcePoint: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_bReverse: usize = 0x368; // bool
                pub const m_bSetOrientation: usize = 0x369; // bool
                pub const m_nOrientation: usize = 0x36C; // ParticleOrientationType_t
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
            pub mod C_OP_ChladniWave {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x360; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4D8; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x650; // CPerParticleFloatInput
                pub const m_vecWaveLength: usize = 0x7C8; // CPerParticleVecInput
                pub const m_vecHarmonics: usize = 0xEA0; // CPerParticleVecInput
                pub const m_nSetMethod: usize = 0x1578; // ParticleSetMethod_t
                pub const m_nLocalSpaceControlPoint: usize = 0x157C; // int32
                pub const m_b3D: usize = 0x1580; // bool
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
            pub mod C_OP_RemapDirectionToCPToVector {
                pub const m_nCP: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0x1E8; // float32
                pub const m_flOffsetRot: usize = 0x1EC; // float32
                pub const m_vecOffsetAxis: usize = 0x1F0; // Vector
                pub const m_bNormalize: usize = 0x1FC; // bool
                pub const m_nFieldStrength: usize = 0x200; // ParticleAttributeIndex_t
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
            pub mod C_OP_DriveCPFromGlobalSoundFloat {
                pub const m_nOutputControlPoint: usize = 0x1E8; // int32
                pub const m_nOutputField: usize = 0x1EC; // int32
                pub const m_flInputMin: usize = 0x1F0; // float32
                pub const m_flInputMax: usize = 0x1F4; // float32
                pub const m_flOutputMin: usize = 0x1F8; // float32
                pub const m_flOutputMax: usize = 0x1FC; // float32
                pub const m_StackName: usize = 0x200; // CUtlString
                pub const m_OperatorName: usize = 0x208; // CUtlString
                pub const m_FieldName: usize = 0x210; // CUtlString
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
            pub mod C_INIT_ScreenSpacePositionOfTarget {
                pub const m_vecTargetPosition: usize = 0x1E8; // CPerParticleVecInput
                pub const m_bOututBehindness: usize = 0x8C0; // bool
                pub const m_nBehindFieldOutput: usize = 0x8C4; // ParticleAttributeIndex_t
                pub const m_flBehindOutputRemap: usize = 0x8C8; // CParticleRemapFloatInput
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
            pub mod C_OP_RtEnvCull {
                pub const m_vecTestDir: usize = 0x1E0; // Vector
                pub const m_vecTestNormal: usize = 0x1EC; // Vector
                pub const m_bCullOnMiss: usize = 0x1F8; // bool
                pub const m_bStickInsteadOfCull: usize = 0x1F9; // bool
                pub const m_RtEnvName: usize = 0x1FA; // char[128]
                pub const m_nRTEnvCP: usize = 0x27C; // int32
                pub const m_nComponent: usize = 0x280; // int32
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
            pub mod C_OP_PinParticleToCP {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_vecOffset: usize = 0x1E8; // CParticleCollectionVecInput
                pub const m_bOffsetLocal: usize = 0x8C0; // bool
                pub const m_nParticleSelection: usize = 0x8C4; // ParticleSelection_t
                pub const m_nParticleNumber: usize = 0x8C8; // CParticleCollectionFloatInput
                pub const m_nPinBreakType: usize = 0xA40; // ParticlePinDistance_t
                pub const m_flBreakDistance: usize = 0xA48; // CParticleCollectionFloatInput
                pub const m_flBreakSpeed: usize = 0xBC0; // CParticleCollectionFloatInput
                pub const m_flAge: usize = 0xD38; // CParticleCollectionFloatInput
                pub const m_nBreakControlPointNumber: usize = 0xEB0; // int32
                pub const m_nBreakControlPointNumber2: usize = 0xEB4; // int32
                pub const m_flBreakValue: usize = 0xEB8; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0x1030; // CPerParticleFloatInput
                pub const m_bRetainInitialVelocity: usize = 0x11A8; // bool
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
            pub mod C_OP_RemapCPtoVector {
                pub const m_nCPInput: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nLocalSpaceCP: usize = 0x1E8; // int32
                pub const m_vInputMin: usize = 0x1EC; // Vector
                pub const m_vInputMax: usize = 0x1F8; // Vector
                pub const m_vOutputMin: usize = 0x204; // Vector
                pub const m_vOutputMax: usize = 0x210; // Vector
                pub const m_flStartTime: usize = 0x21C; // float32
                pub const m_flEndTime: usize = 0x220; // float32
                pub const m_flInterpRate: usize = 0x224; // float32
                pub const m_nSetMethod: usize = 0x228; // ParticleSetMethod_t
                pub const m_bOffset: usize = 0x22C; // bool
                pub const m_bAccelerate: usize = 0x22D; // bool
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
            pub mod C_OP_DensityForce {
                pub const m_flRadiusScale: usize = 0x1F0; // float32
                pub const m_flForceScale: usize = 0x1F4; // float32
                pub const m_flTargetDensity: usize = 0x1F8; // float32
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
            pub mod C_INIT_CreateInEpitrochoid {
                pub const m_nComponent1: usize = 0x1E8; // int32
                pub const m_nComponent2: usize = 0x1EC; // int32
                pub const m_TransformInput: usize = 0x1F0; // CParticleTransformInput
                pub const m_flParticleDensity: usize = 0x258; // CPerParticleFloatInput
                pub const m_flOffset: usize = 0x3D0; // CPerParticleFloatInput
                pub const m_flRadius1: usize = 0x548; // CPerParticleFloatInput
                pub const m_flRadius2: usize = 0x6C0; // CPerParticleFloatInput
                pub const m_bUseCount: usize = 0x838; // bool
                pub const m_bUseLocalCoords: usize = 0x839; // bool
                pub const m_bOffsetExistingPos: usize = 0x83A; // bool
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
            pub mod C_OP_SetControlPointPositions {
                pub const m_bUseWorldLocation: usize = 0x1E8; // bool
                pub const m_bOrient: usize = 0x1E9; // bool
                pub const m_bSetOnce: usize = 0x1EA; // bool
                pub const m_nCP1: usize = 0x1EC; // int32
                pub const m_nCP2: usize = 0x1F0; // int32
                pub const m_nCP3: usize = 0x1F4; // int32
                pub const m_nCP4: usize = 0x1F8; // int32
                pub const m_vecCP1Pos: usize = 0x1FC; // Vector
                pub const m_vecCP2Pos: usize = 0x208; // Vector
                pub const m_vecCP3Pos: usize = 0x214; // Vector
                pub const m_vecCP4Pos: usize = 0x220; // Vector
                pub const m_nHeadLocation: usize = 0x22C; // int32
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
            pub mod C_OP_SetFloatAttributeToVectorExpression {
                pub const m_nExpression: usize = 0x1E0; // VectorFloatExpressionType_t
                pub const m_vInput1: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x8C0; // CPerParticleVecInput
                pub const m_flOutputRemap: usize = 0xF98; // CParticleRemapFloatInput
                pub const m_nOutputField: usize = 0x1110; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x1114; // ParticleSetMethod_t
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
            pub mod C_OP_MovementRotateParticleAroundAxis {
                pub const m_vecRotAxis: usize = 0x1E0; // CParticleCollectionVecInput
                pub const m_flRotRate: usize = 0x8B8; // CParticleCollectionFloatInput
                pub const m_TransformInput: usize = 0xA30; // CParticleTransformInput
                pub const m_bLocalSpace: usize = 0xA98; // bool
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
            pub mod C_OP_IntraParticleForce {
                pub const m_flAttractionMinDistance: usize = 0x1F0; // float32
                pub const m_flAttractionMaxDistance: usize = 0x1F4; // float32
                pub const m_flAttractionMaxStrength: usize = 0x1F8; // float32
                pub const m_flRepulsionMinDistance: usize = 0x1FC; // float32
                pub const m_flRepulsionMaxDistance: usize = 0x200; // float32
                pub const m_flRepulsionMaxStrength: usize = 0x204; // float32
                pub const m_bUseAABB: usize = 0x208; // bool
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
            pub mod C_INIT_InitFloat {
                pub const m_InputValue: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x360; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x364; // ParticleSetMethod_t
                pub const m_InputStrength: usize = 0x368; // CPerParticleFloatInput
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
            pub mod C_OP_InheritFromPeerSystem {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nFieldInput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x1E8; // int32
                pub const m_nGroupID: usize = 0x1EC; // int32
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
            pub mod C_OP_PerParticleForce {
                pub const m_flForceScale: usize = 0x1F0; // CPerParticleFloatInput
                pub const m_vForce: usize = 0x368; // CPerParticleVecInput
                pub const m_nCP: usize = 0xA40; // int32
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
            pub mod C_OP_MaxVelocity {
                pub const m_flMaxVelocity: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_flMinVelocity: usize = 0x358; // CPerParticleFloatInput
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
            pub mod C_INIT_VelocityFromNormal {
                pub const m_fSpeedMin: usize = 0x1E8; // float32
                pub const m_fSpeedMax: usize = 0x1EC; // float32
                pub const m_bIgnoreDt: usize = 0x1F0; // bool
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
            pub mod C_INIT_PositionOffsetToCP {
                pub const m_nControlPointNumberStart: usize = 0x1E8; // int32
                pub const m_nControlPointNumberEnd: usize = 0x1EC; // int32
                pub const m_bLocalCoords: usize = 0x1F0; // bool
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
            pub mod C_INIT_RemapInitialTransformDirectionToRotation {
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_nFieldOutput: usize = 0x250; // ParticleAttributeIndex_t
                pub const m_flOffsetRot: usize = 0x254; // float32
                pub const m_nComponent: usize = 0x258; // int32
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
            pub mod C_OP_FadeAndKill {
                pub const m_flStartFadeInTime: usize = 0x1E0; // float32
                pub const m_flEndFadeInTime: usize = 0x1E4; // float32
                pub const m_flStartFadeOutTime: usize = 0x1E8; // float32
                pub const m_flEndFadeOutTime: usize = 0x1EC; // float32
                pub const m_flStartAlpha: usize = 0x1F0; // float32
                pub const m_flEndAlpha: usize = 0x1F4; // float32
                pub const m_bForcePreserveParticleOrder: usize = 0x1F8; // bool
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
            pub mod C_OP_ColorInterpolate {
                pub const m_ColorFade: usize = 0x1E0; // Color
                pub const m_flFadeStartTime: usize = 0x1F0; // float32
                pub const m_flFadeEndTime: usize = 0x1F4; // float32
                pub const m_nFieldOutput: usize = 0x1F8; // ParticleAttributeIndex_t
                pub const m_bEaseInOut: usize = 0x1FC; // bool
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
            pub mod C_OP_RampScalarSpline {
                pub const m_RateMin: usize = 0x1E0; // float32
                pub const m_RateMax: usize = 0x1E4; // float32
                pub const m_flStartTime_min: usize = 0x1E8; // float32
                pub const m_flStartTime_max: usize = 0x1EC; // float32
                pub const m_flEndTime_min: usize = 0x1F0; // float32
                pub const m_flEndTime_max: usize = 0x1F4; // float32
                pub const m_flBias: usize = 0x1F8; // float32
                pub const m_nField: usize = 0x220; // ParticleAttributeIndex_t
                pub const m_bProportionalOp: usize = 0x224; // bool
                pub const m_bEaseOut: usize = 0x225; // bool
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
            pub mod C_OP_RemapNamedModelSequenceOnceTimed {
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
            pub mod C_OP_MaintainSequentialPath {
                pub const m_fMaxDistance: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_flNumToAssign: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_flCohesionStrength: usize = 0x4D0; // CParticleCollectionFloatInput
                pub const m_flTolerance: usize = 0x648; // float32
                pub const m_bLoop: usize = 0x64C; // bool
                pub const m_bUseParticleCount: usize = 0x64D; // bool
                pub const m_PathParams: usize = 0x650; // CPathParameters
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
            pub mod C_OP_RemapNamedModelBodyPartEndCap {
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
            pub mod C_OP_StopAfterCPDuration {
                pub const m_flDuration: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_bDestroyImmediately: usize = 0x360; // bool
                pub const m_bPlayEndCap: usize = 0x361; // bool
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
            pub mod CGeneralSpin {
                pub const m_nSpinRateDegrees: usize = 0x1E0; // int32
                pub const m_nSpinRateMinDegrees: usize = 0x1E4; // int32
                pub const m_fSpinRateStopTime: usize = 0x1EC; // float32
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
            pub mod C_INIT_RemapNamedModelElementToScalar {
                pub const m_hModel: usize = 0x1E8; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_names: usize = 0x1F0; // CUtlVector<CUtlString>
                pub const m_values: usize = 0x208; // CUtlVector<float32>
                pub const m_nFieldInput: usize = 0x220; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x224; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x228; // ParticleSetMethod_t
                pub const m_bModelFromRenderer: usize = 0x22C; // bool
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
            pub mod C_OP_ClampVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vecOutputMin: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vecOutputMax: usize = 0x8C0; // CPerParticleVecInput
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
            pub mod C_OP_RenderStatusEffectCitadel {
                pub const m_pTextureColorWarp: usize = 0x230; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureNormal: usize = 0x238; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureMetalness: usize = 0x240; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureRoughness: usize = 0x248; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureSelfIllum: usize = 0x250; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_pTextureDetail: usize = 0x258; // CStrongHandle<InfoForResourceTypeCTextureBase>
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
            pub mod C_INIT_VelocityFromWind {
                pub const m_vecSamplePosition: usize = 0x1E8; // CPerParticleVecInput
                pub const m_flScale: usize = 0x8C0; // CPerParticleFloatInput
                pub const m_bSampleFans: usize = 0xA38; // bool
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
            pub mod C_OP_WindForce {
                pub const m_vForce: usize = 0x1F0; // Vector
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
            pub mod C_OP_RenderStandardLight {
                pub const m_nLightType: usize = 0x230; // ParticleLightTypeChoiceList_t
                pub const m_nMaxAllowed: usize = 0x234; // uint16
                pub const m_vecColorScale: usize = 0x238; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0x910; // ParticleColorBlendType_t
                pub const m_strLightStyle: usize = 0x918; // CUtlString
                pub const m_flLightStyleTime: usize = 0x920; // CPerParticleFloatInput
                pub const m_flIntensity: usize = 0xA98; // CPerParticleFloatInput
                pub const m_bCastShadows: usize = 0xC10; // bool
                pub const m_bDynamicBounce: usize = 0xC11; // bool
                pub const m_flBounceScale: usize = 0xC18; // CParticleCollectionFloatInput
                pub const m_flTheta: usize = 0xD90; // CParticleCollectionFloatInput
                pub const m_flPhi: usize = 0xF08; // CParticleCollectionFloatInput
                pub const m_flRadiusMultiplier: usize = 0x1080; // CParticleCollectionFloatInput
                pub const m_nAttenuationStyle: usize = 0x11F8; // StandardLightingAttenuationStyle_t
                pub const m_flFalloffLinearity: usize = 0x1200; // CParticleCollectionFloatInput
                pub const m_flFiftyPercentFalloff: usize = 0x1378; // CParticleCollectionFloatInput
                pub const m_flZeroPercentFalloff: usize = 0x14F0; // CParticleCollectionFloatInput
                pub const m_bRenderDiffuse: usize = 0x1668; // bool
                pub const m_bRenderSpecular: usize = 0x1669; // bool
                pub const m_lightCookie: usize = 0x1670; // CUtlString
                pub const m_nPriority: usize = 0x1678; // int32
                pub const m_nFogLightingMode: usize = 0x167C; // ParticleLightFogLightingMode_t
                pub const m_flFogContribution: usize = 0x1680; // CParticleCollectionRendererFloatInput
                pub const m_nCapsuleLightBehavior: usize = 0x17F8; // ParticleLightBehaviorChoiceList_t
                pub const m_flCapsuleLength: usize = 0x17FC; // float32
                pub const m_bReverseOrder: usize = 0x1800; // bool
                pub const m_bClosedLoop: usize = 0x1801; // bool
                pub const m_nPrevPntSource: usize = 0x1804; // ParticleAttributeIndex_t
                pub const m_flMaxLength: usize = 0x1808; // float32
                pub const m_flMinLength: usize = 0x180C; // float32
                pub const m_bIgnoreDT: usize = 0x1810; // bool
                pub const m_flConstrainRadiusToLengthRatio: usize = 0x1814; // float32
                pub const m_flLengthScale: usize = 0x1818; // float32
                pub const m_flLengthFadeInTime: usize = 0x181C; // float32
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
            pub mod C_OP_DistanceToTransform {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x360; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4D8; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x650; // CPerParticleFloatInput
                pub const m_TransformStart: usize = 0x7C8; // CParticleTransformInput
                pub const m_bLOS: usize = 0x830; // bool
                pub const m_CollisionGroupName: usize = 0x831; // char[128]
                pub const m_nTraceSet: usize = 0x8B4; // ParticleTraceSet_t
                pub const m_flMaxTraceLength: usize = 0x8B8; // float32
                pub const m_flLOSScale: usize = 0x8BC; // float32
                pub const m_nSetMethod: usize = 0x8C0; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x8C4; // bool
                pub const m_bAdditive: usize = 0x8C5; // bool
                pub const m_vecComponentScale: usize = 0x8C8; // CPerParticleVecInput
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
            pub mod C_OP_RemapControlPointOrientationToRotation {
                pub const m_nCP: usize = 0x1E0; // int32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flOffsetRot: usize = 0x1E8; // float32
                pub const m_nComponent: usize = 0x1EC; // int32
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
            pub mod C_OP_SetControlPointToCenter {
                pub const m_nCP1: usize = 0x1E8; // int32
                pub const m_vecCP1Pos: usize = 0x1EC; // Vector
                pub const m_bUseAvgParticlePos: usize = 0x1F8; // bool
                pub const m_nSetParent: usize = 0x1FC; // ParticleParentSetMode_t
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
            pub mod C_OP_RemapAverageScalarValuetoCP {
                pub const m_nExpression: usize = 0x1E8; // SetStatisticExpressionType_t
                pub const m_flDecimalPlaces: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_nOutControlPointNumber: usize = 0x368; // int32
                pub const m_nOutVectorField: usize = 0x36C; // int32
                pub const m_nField: usize = 0x370; // ParticleAttributeIndex_t
                pub const m_flOutputRemap: usize = 0x378; // CParticleRemapFloatInput
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
            pub mod C_OP_RemapDotProductToScalar {
                pub const m_nInputCP1: usize = 0x1E0; // int32
                pub const m_nInputCP2: usize = 0x1E4; // int32
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1EC; // float32
                pub const m_flInputMax: usize = 0x1F0; // float32
                pub const m_flOutputMin: usize = 0x1F4; // float32
                pub const m_flOutputMax: usize = 0x1F8; // float32
                pub const m_bUseParticleVelocity: usize = 0x1FC; // bool
                pub const m_nSetMethod: usize = 0x200; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x204; // bool
                pub const m_bUseParticleNormal: usize = 0x205; // bool
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            pub mod C_OP_CurlNoiseForce {
                pub const m_nNoiseType: usize = 0x1F0; // ParticleDirectionNoiseType_t
                pub const m_vecNoiseFreq: usize = 0x1F8; // CPerParticleVecInput
                pub const m_vecNoiseScale: usize = 0x8D0; // CPerParticleVecInput
                pub const m_vecOffset: usize = 0xFA8; // CPerParticleVecInput
                pub const m_vecOffsetRate: usize = 0x1680; // CPerParticleVecInput
                pub const m_flWorleySeed: usize = 0x1D58; // CPerParticleFloatInput
                pub const m_flWorleyJitter: usize = 0x1ED0; // CPerParticleFloatInput
                pub const m_nCP: usize = 0x1F0; // int32
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
            pub mod C_INIT_Orient2DRelToCP {
                pub const m_nCP: usize = 0x1E8; // int32
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_flRotOffset: usize = 0x1F0; // float32
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
            pub mod C_OP_FadeIn {
                pub const m_flFadeInTimeMin: usize = 0x1E0; // float32
                pub const m_flFadeInTimeMax: usize = 0x1E4; // float32
                pub const m_flFadeInTimeExp: usize = 0x1E8; // float32
                pub const m_bProportional: usize = 0x1EC; // bool
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
            pub mod C_OP_RemapVectorToRotations {
                pub const m_vecInput: usize = 0x1E0; // CPerParticleVecInput
                pub const m_vecRotation: usize = 0x8B8; // CPerParticleVecInput
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
            pub mod C_INIT_GlobalScale {
                pub const m_flScale: usize = 0x1E8; // float32
                pub const m_nScaleControlPointNumber: usize = 0x1EC; // int32
                pub const m_nControlPointNumber: usize = 0x1F0; // int32
                pub const m_bScaleRadius: usize = 0x1F4; // bool
                pub const m_bScalePosition: usize = 0x1F5; // bool
                pub const m_bScaleVelocity: usize = 0x1F6; // bool
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
            pub mod C_INIT_RadiusFromCPObject {
                pub const m_nControlPoint: usize = 0x1E8; // int32
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
            pub mod C_INIT_InitialVelocityFromHitbox {
                pub const m_flVelocityMin: usize = 0x1E8; // float32
                pub const m_flVelocityMax: usize = 0x1EC; // float32
                pub const m_nControlPointNumber: usize = 0x1F0; // int32
                pub const m_HitboxSetName: usize = 0x1F4; // char[128]
                pub const m_bUseBones: usize = 0x274; // bool
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
            pub mod C_OP_LerpVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vecOutput: usize = 0x1E4; // Vector
                pub const m_flStartTime: usize = 0x1F0; // float32
                pub const m_flEndTime: usize = 0x1F4; // float32
                pub const m_nSetMethod: usize = 0x1F8; // ParticleSetMethod_t
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
            pub mod C_OP_SetControlPointFieldToWater {
                pub const m_nSourceCP: usize = 0x1E8; // int32
                pub const m_nDestCP: usize = 0x1EC; // int32
                pub const m_nCPField: usize = 0x1F0; // int32
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
            pub mod TextureGroup_t {
                pub const m_bEnabled: usize = 0x0; // bool
                pub const m_bReplaceTextureWithGradient: usize = 0x1; // bool
                pub const m_hTexture: usize = 0x8; // CStrongHandle<InfoForResourceTypeCTextureBase>
                pub const m_Gradient: usize = 0x10; // CColorGradient
                pub const m_nTextureType: usize = 0x28; // SpriteCardTextureType_t
                pub const m_nTextureChannels: usize = 0x2C; // SpriteCardTextureChannel_t
                pub const m_nTextureBlendMode: usize = 0x30; // ParticleTextureLayerBlendType_t
                pub const m_flTextureBlend: usize = 0x38; // CParticleCollectionRendererFloatInput
                pub const m_TextureControls: usize = 0x1B0; // TextureControls_t
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
            pub mod C_OP_TimeVaryingForce {
                pub const m_flStartLerpTime: usize = 0x1F0; // float32
                pub const m_StartingForce: usize = 0x1F4; // Vector
                pub const m_flEndLerpTime: usize = 0x200; // float32
                pub const m_EndingForce: usize = 0x204; // Vector
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
            pub mod C_OP_SetCPOrientationToGroundNormal {
                pub const m_flInterpRate: usize = 0x1E0; // float32
                pub const m_flMaxTraceLength: usize = 0x1E4; // float32
                pub const m_flTolerance: usize = 0x1E8; // float32
                pub const m_flTraceOffset: usize = 0x1EC; // float32
                pub const m_CollisionGroupName: usize = 0x1F0; // char[128]
                pub const m_nTraceSet: usize = 0x270; // ParticleTraceSet_t
                pub const m_nInputCP: usize = 0x274; // int32
                pub const m_nOutputCP: usize = 0x278; // int32
                pub const m_bIncludeWater: usize = 0x288; // bool
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
            pub mod C_INIT_CreateWithinSphereTransform {
                pub const m_fRadiusMin: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_fRadiusMax: usize = 0x360; // CPerParticleFloatInput
                pub const m_vecDistanceBias: usize = 0x4D8; // CPerParticleVecInput
                pub const m_vecDistanceBiasAbs: usize = 0xBB0; // Vector
                pub const m_TransformInput: usize = 0xBC0; // CParticleTransformInput
                pub const m_fSpeedMin: usize = 0xC28; // CPerParticleFloatInput
                pub const m_fSpeedMax: usize = 0xDA0; // CPerParticleFloatInput
                pub const m_fSpeedRandExp: usize = 0xF18; // float32
                pub const m_bLocalCoords: usize = 0xF1C; // bool
                pub const m_LocalCoordinateSystemSpeedMin: usize = 0xF20; // CPerParticleVecInput
                pub const m_LocalCoordinateSystemSpeedMax: usize = 0x15F8; // CPerParticleVecInput
                pub const m_nFieldOutput: usize = 0x1CD0; // ParticleAttributeIndex_t
                pub const m_nFieldVelocity: usize = 0x1CD4; // ParticleAttributeIndex_t
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
            pub mod C_OP_RadiusDecay {
                pub const m_flMinRadius: usize = 0x1E0; // float32
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
            pub mod C_INIT_RemapNamedModelBodyPartToScalar {
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
            pub mod C_INIT_RemapScalarToVector {
                pub const m_nFieldInput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1F0; // float32
                pub const m_flInputMax: usize = 0x1F4; // float32
                pub const m_vecOutputMin: usize = 0x1F8; // Vector
                pub const m_vecOutputMax: usize = 0x204; // Vector
                pub const m_flStartTime: usize = 0x210; // float32
                pub const m_flEndTime: usize = 0x214; // float32
                pub const m_nSetMethod: usize = 0x218; // ParticleSetMethod_t
                pub const m_nControlPointNumber: usize = 0x21C; // int32
                pub const m_bLocalCoords: usize = 0x220; // bool
                pub const m_flRemapBias: usize = 0x224; // float32
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
            pub mod C_INIT_InitialSequenceFromModel {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_nFieldOutputAnim: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1F4; // float32
                pub const m_flInputMax: usize = 0x1F8; // float32
                pub const m_flOutputMin: usize = 0x1FC; // float32
                pub const m_flOutputMax: usize = 0x200; // float32
                pub const m_nSetMethod: usize = 0x204; // ParticleSetMethod_t
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
            pub mod C_OP_SelectivelyEnableChildren {
                pub const m_nChildGroupID: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_nFirstChild: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_nNumChildrenToEnable: usize = 0x4D8; // CParticleCollectionFloatInput
                pub const m_bPlayEndcapOnStop: usize = 0x650; // bool
                pub const m_bDestroyImmediately: usize = 0x651; // bool
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
            pub mod ModelReference_t {
                pub const m_model: usize = 0x0; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_flRelativeProbabilityOfSpawn: usize = 0x8; // float32
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
            pub mod C_INIT_CreateFromCPs {
                pub const m_nIncrement: usize = 0x1E8; // int32
                pub const m_nMinCP: usize = 0x1EC; // int32
                pub const m_nMaxCP: usize = 0x1F0; // int32
                pub const m_nDynamicCPCount: usize = 0x1F8; // CParticleCollectionFloatInput
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
            pub mod C_INIT_CreateSpiralSphere {
                pub const m_TransformInput: usize = 0x1E8; // CParticleTransformInput
                pub const m_flDensity: usize = 0x250; // CPerParticleFloatInput
                pub const m_flInitialRadius: usize = 0x3C8; // CPerParticleFloatInput
                pub const m_flInitialSpeedMin: usize = 0x540; // CPerParticleFloatInput
                pub const m_flInitialSpeedMax: usize = 0x6B8; // CPerParticleFloatInput
                pub const m_bUseParticleCount: usize = 0x830; // bool
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
            pub mod C_OP_CPVelocityForce {
                pub const m_nControlPointNumber: usize = 0x1F0; // int32
                pub const m_flScale: usize = 0x1F8; // CPerParticleFloatInput
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
            pub mod C_INIT_ScaleVelocity {
                pub const m_vecScale: usize = 0x1E8; // CPerParticleVecInput
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
            pub mod C_OP_MoveToHitbox {
                pub const m_modelInput: usize = 0x1E0; // CParticleModelInput
                pub const m_transformInput: usize = 0x240; // CParticleTransformInput
                pub const m_flLifeTimeLerpStart: usize = 0x2AC; // float32
                pub const m_flLifeTimeLerpEnd: usize = 0x2B0; // float32
                pub const m_flPrevPosScale: usize = 0x2B4; // float32
                pub const m_HitboxSetName: usize = 0x2B8; // char[128]
                pub const m_bUseBones: usize = 0x338; // bool
                pub const m_nLerpType: usize = 0x33C; // HitboxLerpType_t
                pub const m_flInterpolation: usize = 0x340; // CPerParticleFloatInput
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
            pub mod C_OP_PinRopeSegmentParticleToParent {
                pub const m_nParticleSelection: usize = 0x1E0; // ParticleSelection_t
                pub const m_nParticleNumber: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_flInterpolation: usize = 0x360; // CPerParticleFloatInput
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
            pub mod C_INIT_PointList {
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_pointList: usize = 0x1F0; // CUtlVector<PointDefinition_t>
                pub const m_bPlaceAlongPath: usize = 0x208; // bool
                pub const m_bClosedLoop: usize = 0x209; // bool
                pub const m_nNumPointsAlongPath: usize = 0x20C; // int32
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
            pub mod C_OP_LerpToOtherAttribute {
                pub const m_flInterpolation: usize = 0x1E0; // CPerParticleFloatInput
                pub const m_nFieldInputFrom: usize = 0x358; // ParticleAttributeIndex_t
                pub const m_nFieldInput: usize = 0x35C; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x360; // ParticleAttributeIndex_t
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
            pub mod C_INIT_RandomColor {
                pub const m_ColorMin: usize = 0x204; // Color
                pub const m_ColorMax: usize = 0x208; // Color
                pub const m_TintMin: usize = 0x20C; // Color
                pub const m_TintMax: usize = 0x210; // Color
                pub const m_flTintPerc: usize = 0x214; // float32
                pub const m_flUpdateThreshold: usize = 0x218; // float32
                pub const m_nTintCP: usize = 0x21C; // int32
                pub const m_nFieldOutput: usize = 0x220; // ParticleAttributeIndex_t
                pub const m_nTintBlendMode: usize = 0x224; // ParticleColorBlendMode_t
                pub const m_flLightAmplification: usize = 0x228; // float32
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
            pub mod C_INIT_InheritFromParentParticles {
                pub const m_flScale: usize = 0x1E8; // float32
                pub const m_nFieldOutput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_nIncrement: usize = 0x1F0; // int32
                pub const m_bRandomDistribution: usize = 0x1F4; // bool
                pub const m_nRandomSeed: usize = 0x1F8; // int32
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
            pub mod C_OP_RampScalarLinearSimple {
                pub const m_Rate: usize = 0x1E0; // float32
                pub const m_flStartTime: usize = 0x1E4; // float32
                pub const m_flEndTime: usize = 0x1E8; // float32
                pub const m_nField: usize = 0x210; // ParticleAttributeIndex_t
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
            pub mod C_INIT_ChaoticAttractor {
                pub const m_flAParm: usize = 0x1E8; // float32
                pub const m_flBParm: usize = 0x1EC; // float32
                pub const m_flCParm: usize = 0x1F0; // float32
                pub const m_flDParm: usize = 0x1F4; // float32
                pub const m_flScale: usize = 0x1F8; // float32
                pub const m_flSpeedMin: usize = 0x1FC; // float32
                pub const m_flSpeedMax: usize = 0x200; // float32
                pub const m_nBaseCP: usize = 0x204; // int32
                pub const m_bUniformSpeed: usize = 0x208; // bool
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
            pub mod C_OP_MovementRigidAttachToCP {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_nScaleControlPoint: usize = 0x1E4; // int32
                pub const m_nScaleCPField: usize = 0x1E8; // int32
                pub const m_nFieldInput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_bOffsetLocal: usize = 0x1F4; // bool
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
            pub mod C_INIT_SkyVisCull {
                pub const m_vecTestPosition: usize = 0x1E8; // CPerParticleVecInput
                pub const m_vecTestDir: usize = 0x8C0; // CParticleCollectionVecInput
                pub const m_nTraceSet: usize = 0xF98; // ParticleTraceSet_t
                pub const m_bCullOnSky: usize = 0xF9C; // bool
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
            pub mod C_INIT_DistanceToCPInit {
                pub const m_nFieldOutput: usize = 0x1E8; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1F0; // CPerParticleFloatInput
                pub const m_flInputMax: usize = 0x368; // CPerParticleFloatInput
                pub const m_flOutputMin: usize = 0x4E0; // CPerParticleFloatInput
                pub const m_flOutputMax: usize = 0x658; // CPerParticleFloatInput
                pub const m_nStartCP: usize = 0x7D0; // int32
                pub const m_bLOS: usize = 0x7D4; // bool
                pub const m_CollisionGroupName: usize = 0x7D5; // char[128]
                pub const m_nTraceSet: usize = 0x858; // ParticleTraceSet_t
                pub const m_flMaxTraceLength: usize = 0x860; // CPerParticleFloatInput
                pub const m_flLOSScale: usize = 0x9D8; // float32
                pub const m_nSetMethod: usize = 0x9DC; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x9E0; // bool
                pub const m_vecDistanceScale: usize = 0x9E4; // Vector
                pub const m_flRemapBias: usize = 0x9F0; // float32
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
            pub mod C_OP_EndCapDecay {
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
            pub mod C_OP_ForceBasedOnDistanceToPlane {
                pub const m_flMinDist: usize = 0x1F0; // float32
                pub const m_vecForceAtMinDist: usize = 0x1F4; // Vector
                pub const m_flMaxDist: usize = 0x200; // float32
                pub const m_vecForceAtMaxDist: usize = 0x204; // Vector
                pub const m_vecPlaneNormal: usize = 0x210; // Vector
                pub const m_nControlPointNumber: usize = 0x21C; // int32
                pub const m_flExponent: usize = 0x220; // float32
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
            pub mod C_OP_RemapDensityToVector {
                pub const m_flRadiusScale: usize = 0x1E0; // float32
                pub const m_nFieldOutput: usize = 0x1E4; // ParticleAttributeIndex_t
                pub const m_flDensityMin: usize = 0x1E8; // float32
                pub const m_flDensityMax: usize = 0x1EC; // float32
                pub const m_vecOutputMin: usize = 0x1F0; // Vector
                pub const m_vecOutputMax: usize = 0x1FC; // Vector
                pub const m_bUseParentDensity: usize = 0x208; // bool
                pub const m_nVoxelGridResolution: usize = 0x20C; // int32
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
            pub mod ParticleControlPointConfiguration_t {
                pub const m_name: usize = 0x0; // CUtlString
                pub const m_drivers: usize = 0x8; // CUtlVector<ParticleControlPointDriver_t>
                pub const m_previewState: usize = 0x20; // ParticlePreviewState_t
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
            pub mod C_INIT_SetRigidAttachment {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_nFieldInput: usize = 0x1EC; // ParticleAttributeIndex_t
                pub const m_nFieldOutput: usize = 0x1F0; // ParticleAttributeIndex_t
                pub const m_bLocalSpace: usize = 0x1F4; // bool
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
            pub mod MaterialVariable_t {
                pub const m_strVariable: usize = 0x0; // CUtlString
                pub const m_nVariableField: usize = 0x8; // ParticleAttributeIndex_t
                pub const m_flScale: usize = 0xC; // float32
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
            pub mod C_OP_RemapSpeed {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_flOutputMin: usize = 0x1EC; // float32
                pub const m_flOutputMax: usize = 0x1F0; // float32
                pub const m_nSetMethod: usize = 0x1F4; // ParticleSetMethod_t
                pub const m_bIgnoreDelta: usize = 0x1F8; // bool
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
            pub mod C_OP_RenderModels {
                pub const m_bOnlyRenderInEffectsBloomPass: usize = 0x230; // bool
                pub const m_bOnlyRenderInEffectsWaterPass: usize = 0x231; // bool
                pub const m_bUseMixedResolutionRendering: usize = 0x232; // bool
                pub const m_bOnlyRenderInEffecsGameOverlay: usize = 0x233; // bool
                pub const m_ModelList: usize = 0x238; // CUtlVector<ModelReference_t>
                pub const m_nBodyGroupField: usize = 0x250; // ParticleAttributeIndex_t
                pub const m_nSubModelField: usize = 0x254; // ParticleAttributeIndex_t
                pub const m_bIgnoreNormal: usize = 0x258; // bool
                pub const m_bOrientZ: usize = 0x259; // bool
                pub const m_bCenterOffset: usize = 0x25A; // bool
                pub const m_vecLocalOffset: usize = 0x260; // CPerParticleVecInput
                pub const m_vecLocalRotation: usize = 0x938; // CPerParticleVecInput
                pub const m_bIgnoreRadius: usize = 0x1010; // bool
                pub const m_nModelScaleCP: usize = 0x1014; // int32
                pub const m_vecComponentScale: usize = 0x1018; // CPerParticleVecInput
                pub const m_bLocalScale: usize = 0x16F0; // bool
                pub const m_nSizeCullBloat: usize = 0x16F4; // int32
                pub const m_bAnimated: usize = 0x16F8; // bool
                pub const m_flAnimationRate: usize = 0x1700; // CPerParticleFloatInput
                pub const m_bScaleAnimationRate: usize = 0x1878; // bool
                pub const m_bForceLoopingAnimation: usize = 0x1879; // bool
                pub const m_bResetAnimOnStop: usize = 0x187A; // bool
                pub const m_bManualAnimFrame: usize = 0x187B; // bool
                pub const m_nAnimationScaleField: usize = 0x187C; // ParticleAttributeIndex_t
                pub const m_nAnimationField: usize = 0x1880; // ParticleAttributeIndex_t
                pub const m_nManualFrameField: usize = 0x1884; // ParticleAttributeIndex_t
                pub const m_ActivityName: usize = 0x1888; // char[256]
                pub const m_SequenceName: usize = 0x1988; // char[256]
                pub const m_bEnableClothSimulation: usize = 0x1A88; // bool
                pub const m_bDisableClothGroundCollision: usize = 0x1A89; // bool
                pub const m_ClothEffectName: usize = 0x1A8A; // char[64]
                pub const m_hOverrideMaterial: usize = 0x1AD0; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_bOverrideTranslucentMaterials: usize = 0x1AD8; // bool
                pub const m_nSkin: usize = 0x1AE0; // CPerParticleFloatInput
                pub const m_MaterialVars: usize = 0x1C58; // CUtlVector<MaterialVariable_t>
                pub const m_flRenderFilter: usize = 0x1C70; // CPerParticleFloatInput
                pub const m_flManualModelSelection: usize = 0x1DE8; // CPerParticleFloatInput
                pub const m_modelInput: usize = 0x1F60; // CParticleModelInput
                pub const m_nLOD: usize = 0x1FC0; // int32
                pub const m_EconSlotName: usize = 0x1FC4; // char[256]
                pub const m_bOriginalModel: usize = 0x20C4; // bool
                pub const m_bSuppressTint: usize = 0x20C5; // bool
                pub const m_nSubModelFieldType: usize = 0x20C8; // RenderModelSubModelFieldType_t
                pub const m_bDisableShadows: usize = 0x20CC; // bool
                pub const m_bDisableDepthPrepass: usize = 0x20CD; // bool
                pub const m_bAcceptsDecals: usize = 0x20CE; // bool
                pub const m_bForceDrawInterlevedWithSiblings: usize = 0x20CF; // bool
                pub const m_bDoNotDrawInParticlePass: usize = 0x20D0; // bool
                pub const m_bAllowApproximateTransforms: usize = 0x20D1; // bool
                pub const m_szRenderAttribute: usize = 0x20D2; // char[260]
                pub const m_flRadiusScale: usize = 0x21D8; // CParticleCollectionFloatInput
                pub const m_flAlphaScale: usize = 0x2350; // CParticleCollectionFloatInput
                pub const m_flRollScale: usize = 0x24C8; // CParticleCollectionFloatInput
                pub const m_nAlpha2Field: usize = 0x2640; // ParticleAttributeIndex_t
                pub const m_vecColorScale: usize = 0x2648; // CParticleCollectionVecInput
                pub const m_nColorBlendType: usize = 0x2D20; // ParticleColorBlendType_t
                pub const m_strLightStyle: usize = 0x2D28; // CUtlString
                pub const m_flLightStyleTime: usize = 0x2D30; // CPerParticleFloatInput
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
            pub mod C_INIT_RemapNamedModelMeshGroupToScalar {
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
            pub mod C_OP_RopeSpringConstraint {
                pub const m_flRestLength: usize = 0x1E0; // CParticleCollectionFloatInput
                pub const m_flMinDistance: usize = 0x358; // CParticleCollectionFloatInput
                pub const m_flMaxDistance: usize = 0x4D0; // CParticleCollectionFloatInput
                pub const m_flAdjustmentScale: usize = 0x648; // float32
                pub const m_flInitialRestingLength: usize = 0x650; // CParticleCollectionFloatInput
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
            pub mod C_INIT_PositionWarpScalar {
                pub const m_vecWarpMin: usize = 0x1E8; // Vector
                pub const m_vecWarpMax: usize = 0x1F4; // Vector
                pub const m_InputValue: usize = 0x200; // CPerParticleFloatInput
                pub const m_flPrevPosScale: usize = 0x378; // float32
                pub const m_nScaleControlPointNumber: usize = 0x37C; // int32
                pub const m_nControlPointNumber: usize = 0x380; // int32
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
            pub mod C_OP_ForceControlPointStub {
                pub const m_ControlPoint: usize = 0x1E8; // int32
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
            pub mod C_OP_VectorNoise {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vecOutputMin: usize = 0x1E4; // Vector
                pub const m_vecOutputMax: usize = 0x1F0; // Vector
                pub const m_fl4NoiseScale: usize = 0x1FC; // float32
                pub const m_bAdditive: usize = 0x200; // bool
                pub const m_bOffset: usize = 0x201; // bool
                pub const m_flNoiseAnimationTimeScale: usize = 0x204; // float32
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
            pub mod C_OP_RemapParticleCountToScalar {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_nInputMin: usize = 0x1E8; // CParticleCollectionFloatInput
                pub const m_nInputMax: usize = 0x360; // CParticleCollectionFloatInput
                pub const m_flOutputMin: usize = 0x4D8; // CParticleCollectionFloatInput
                pub const m_flOutputMax: usize = 0x650; // CParticleCollectionFloatInput
                pub const m_bActiveRange: usize = 0x7C8; // bool
                pub const m_nSetMethod: usize = 0x7CC; // ParticleSetMethod_t
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
            pub mod C_INIT_QuantizeFloat {
                pub const m_InputValue: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_nOutputField: usize = 0x360; // ParticleAttributeIndex_t
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
            pub mod C_OP_SetToCP {
                pub const m_nControlPointNumber: usize = 0x1E0; // int32
                pub const m_vecOffset: usize = 0x1E4; // Vector
                pub const m_bOffsetLocal: usize = 0x1F0; // bool
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
            pub mod ParticleControlPointDriver_t {
                pub const m_iControlPoint: usize = 0x0; // ParticleParamID_t
                pub const m_iAttachType: usize = 0x10; // ParticleAttachment_t
                pub const m_attachmentName: usize = 0x18; // CUtlString
                pub const m_vecOffset: usize = 0x20; // Vector
                pub const m_angOffset: usize = 0x2C; // QAngle
                pub const m_entityName: usize = 0x38; // CUtlString
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
            pub mod C_OP_ParentVortices {
                pub const m_flForceScale: usize = 0x1F0; // float32
                pub const m_vecTwistAxis: usize = 0x1F4; // Vector
                pub const m_bFlipBasedOnYaw: usize = 0x200; // bool
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
            pub mod C_OP_SetControlPointToCPVelocity {
                pub const m_nCPInput: usize = 0x1E8; // int32
                pub const m_nCPOutputVel: usize = 0x1EC; // int32
                pub const m_bNormalize: usize = 0x1F0; // bool
                pub const m_nCPOutputMag: usize = 0x1F4; // int32
                pub const m_nCPField: usize = 0x1F8; // int32
                pub const m_vecComparisonVelocity: usize = 0x200; // CParticleCollectionVecInput
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
            pub mod C_OP_SpinYaw {
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
            pub mod RenderProjectedMaterial_t {
                pub const m_hMaterial: usize = 0x0; // CStrongHandle<InfoForResourceTypeIMaterial2>
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
            pub mod C_INIT_SetFloatAttributeToVectorExpression {
                pub const m_nExpression: usize = 0x1E8; // VectorFloatExpressionType_t
                pub const m_vInput1: usize = 0x1F0; // CPerParticleVecInput
                pub const m_vInput2: usize = 0x8C8; // CPerParticleVecInput
                pub const m_flOutputRemap: usize = 0xFA0; // CParticleRemapFloatInput
                pub const m_nOutputField: usize = 0x1118; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x111C; // ParticleSetMethod_t
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
            pub mod C_INIT_ModelCull {
                pub const m_nControlPointNumber: usize = 0x1E8; // int32
                pub const m_bBoundBox: usize = 0x1EC; // bool
                pub const m_bCullOutside: usize = 0x1ED; // bool
                pub const m_bUseBones: usize = 0x1EE; // bool
                pub const m_HitboxSetName: usize = 0x1EF; // char[128]
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
            pub mod C_OP_PercentageBetweenTransformLerpCPs {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_flInputMin: usize = 0x1E4; // float32
                pub const m_flInputMax: usize = 0x1E8; // float32
                pub const m_TransformStart: usize = 0x1F0; // CParticleTransformInput
                pub const m_TransformEnd: usize = 0x258; // CParticleTransformInput
                pub const m_nOutputStartCP: usize = 0x2C0; // int32
                pub const m_nOutputStartField: usize = 0x2C4; // int32
                pub const m_nOutputEndCP: usize = 0x2C8; // int32
                pub const m_nOutputEndField: usize = 0x2CC; // int32
                pub const m_nSetMethod: usize = 0x2D0; // ParticleSetMethod_t
                pub const m_bActiveRange: usize = 0x2D4; // bool
                pub const m_bRadialCheck: usize = 0x2D5; // bool
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
            pub mod C_OP_SetPerChildControlPoint {
                pub const m_nChildGroupID: usize = 0x1E0; // int32
                pub const m_nFirstControlPoint: usize = 0x1E4; // int32
                pub const m_nNumControlPoints: usize = 0x1E8; // int32
                pub const m_nParticleIncrement: usize = 0x1F0; // CParticleCollectionFloatInput
                pub const m_nFirstSourcePoint: usize = 0x368; // CParticleCollectionFloatInput
                pub const m_bSetOrientation: usize = 0x4E0; // bool
                pub const m_nOrientationField: usize = 0x4E4; // ParticleAttributeIndex_t
                pub const m_bNumBasedOnParticleCount: usize = 0x4E8; // bool
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
            pub mod C_OP_WorldCollideConstraint {
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
            pub mod C_OP_SetAttributeToScalarExpression {
                pub const m_nExpression: usize = 0x1E0; // ScalarExpressionType_t
                pub const m_flInput1: usize = 0x1E8; // CPerParticleFloatInput
                pub const m_flInput2: usize = 0x360; // CPerParticleFloatInput
                pub const m_flOutputRemap: usize = 0x4D8; // CParticleRemapFloatInput
                pub const m_nOutputField: usize = 0x650; // ParticleAttributeIndex_t
                pub const m_nSetMethod: usize = 0x654; // ParticleSetMethod_t
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
            pub mod C_OP_RenderMaterialProxy {
                pub const m_nMaterialControlPoint: usize = 0x230; // int32
                pub const m_nProxyType: usize = 0x234; // MaterialProxyType_t
                pub const m_MaterialVars: usize = 0x238; // CUtlVector<MaterialVariable_t>
                pub const m_hOverrideMaterial: usize = 0x250; // CStrongHandle<InfoForResourceTypeIMaterial2>
                pub const m_flMaterialOverrideEnabled: usize = 0x258; // CParticleCollectionFloatInput
                pub const m_vecColorScale: usize = 0x3D0; // CParticleCollectionVecInput
                pub const m_flAlpha: usize = 0xAA8; // CPerParticleFloatInput
                pub const m_nColorBlendType: usize = 0xC20; // ParticleColorBlendType_t
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
            pub mod FloatInputMaterialVariable_t {
                pub const m_strVariable: usize = 0x0; // CUtlString
                pub const m_flInput: usize = 0x8; // CParticleCollectionFloatInput
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
            pub mod C_OP_RampScalarLinear {
                pub const m_RateMin: usize = 0x1E0; // float32
                pub const m_RateMax: usize = 0x1E4; // float32
                pub const m_flStartTime_min: usize = 0x1E8; // float32
                pub const m_flStartTime_max: usize = 0x1EC; // float32
                pub const m_flEndTime_min: usize = 0x1F0; // float32
                pub const m_flEndTime_max: usize = 0x1F4; // float32
                pub const m_nField: usize = 0x220; // ParticleAttributeIndex_t
                pub const m_bProportionalOp: usize = 0x224; // bool
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
            pub mod C_OP_RotateVector {
                pub const m_nFieldOutput: usize = 0x1E0; // ParticleAttributeIndex_t
                pub const m_vecRotAxisMin: usize = 0x1E4; // Vector
                pub const m_vecRotAxisMax: usize = 0x1F0; // Vector
                pub const m_flRotRateMin: usize = 0x1FC; // float32
                pub const m_flRotRateMax: usize = 0x200; // float32
                pub const m_bNormalize: usize = 0x204; // bool
                pub const m_flScale: usize = 0x208; // CPerParticleFloatInput
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
            pub mod C_INIT_InitVecCollection {
                pub const m_InputValue: usize = 0x1E8; // CParticleCollectionVecInput
                pub const m_nOutputField: usize = 0x8C0; // ParticleAttributeIndex_t
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
            pub mod C_INIT_RemapParticleCountToNamedModelMeshGroupScalar {
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
            pub mod C_INIT_SequenceFromCP {
                pub const m_bKillUnused: usize = 0x1E8; // bool
                pub const m_bRadiusScale: usize = 0x1E9; // bool
                pub const m_nCP: usize = 0x1EC; // int32
                pub const m_vecOffset: usize = 0x1F0; // Vector
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
            pub mod C_OP_CPOffsetToPercentageBetweenCPs {
                pub const m_flInputMin: usize = 0x1E0; // float32
                pub const m_flInputMax: usize = 0x1E4; // float32
                pub const m_flInputBias: usize = 0x1E8; // float32
                pub const m_nStartCP: usize = 0x1EC; // int32
                pub const m_nEndCP: usize = 0x1F0; // int32
                pub const m_nOffsetCP: usize = 0x1F4; // int32
                pub const m_nOuputCP: usize = 0x1F8; // int32
                pub const m_nInputCP: usize = 0x1FC; // int32
                pub const m_bRadialCheck: usize = 0x200; // bool
                pub const m_bScaleOffset: usize = 0x201; // bool
                pub const m_vecOffset: usize = 0x204; // Vector
            }
        }
    }
}
