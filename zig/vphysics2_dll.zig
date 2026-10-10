// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-10 12:12:06.506827800 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: vphysics2.dll
        // Class count: 30
        // Enum count: 5
        pub const vphysics2_dll = struct {
            // Alignment: 4
            // Member count: 3
            pub const JointMotion_t = enum(u32) {
                JOINT_MOTION_FREE = 0x0,
                JOINT_MOTION_LOCKED = 0x1,
                JOINT_MOTION_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const JointAxis_t = enum(u32) {
                JOINT_AXIS_X = 0x0,
                JOINT_AXIS_Y = 0x1,
                JOINT_AXIS_Z = 0x2,
                JOINT_AXIS_COUNT = 0x3
            };
            // Alignment: 1
            // Member count: 3
            pub const DynamicContinuousContactBehavior_t = enum(u8) {
                DYNAMIC_CONTINUOUS_ALLOW_IF_REQUESTED_BY_OTHER_BODY = 0x0,
                DYNAMIC_CONTINUOUS_ALWAYS = 0x1,
                DYNAMIC_CONTINUOUS_NEVER = 0x2
            };
            // Alignment: 4
            // Member count: 8
            pub const PhysInterfaceId_t = enum(u32) {
                PIID_UNKNOWN = 0x0,
                PIID_IPHYSICSBODY = 0x1,
                PIID_IPHYSAGGREGATE = 0x2,
                PIID_IPHYSICSJOINT = 0x3,
                PIID_IPHYSICSMOTIONCONTROLLER = 0x4,
                PIID_IPHYSICSPARTICLEROPE = 0x5,
                PIID_IPHYSICSRAGDOLLCONTROL = 0x6,
                PIID_NUM_TYPES = 0x7
            };
            // Alignment: 1
            // Member count: 5
            pub const PhysGenericShapeType_t = enum(u8) {
                GENERIC_SHAPE_POINT = 0x0,
                GENERIC_SHAPE_SPHERE = 0x1,
                GENERIC_SHAPE_AABB = 0x2,
                GENERIC_SHAPE_CAPSULE = 0x3,
                GENERIC_SHAPE_HULL = 0x4
            };
            // Parent: None
            // Field count: 2
            pub const IPhysAggregateInstance = struct {
                pub const m_pSkeleton: usize = 0x8; // void*
                pub const m_bIsAxisAligned: usize = 0x10; // bool
            };
            // Parent: None
            // Field count: 4
            pub const constraint_axislimit_t = struct {
                pub const flMinRotation: usize = 0x0; // float32
                pub const flMaxRotation: usize = 0x4; // float32
                pub const flMotorTargetAngSpeed: usize = 0x8; // float32
                pub const flMotorMaxTorque: usize = 0xC; // float32
            };
            // Parent: None
            // Field count: 0
            pub const IPhysicsParticleRope = struct {
            };
            // Parent: None
            // Field count: 0
            pub const IPhysicsBodyList = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const RnTriangle_t = struct {
                pub const m_nIndex: usize = 0x0; // int32[3]
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FeProxyVertexMap_t = struct {
                pub const m_Name: usize = 0x0; // CUtlString
                pub const m_flWeight: usize = 0x8; // float32
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const FePrism_t = struct {
                pub const nNode: usize = 0x0; // uint16[6]
            };
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const OldFeEdge_t = struct {
                pub const m_flK: usize = 0x0; // float32[3]
                pub const invA: usize = 0xC; // float32
                pub const t: usize = 0x10; // float32
                pub const flThetaRelaxed: usize = 0x14; // float32
                pub const flThetaFactor: usize = 0x18; // float32
                pub const c01: usize = 0x1C; // float32
                pub const c02: usize = 0x20; // float32
                pub const c03: usize = 0x24; // float32
                pub const c04: usize = 0x28; // float32
                pub const flAxialModelDist: usize = 0x2C; // float32
                pub const flAxialModelWeights: usize = 0x30; // float32[4]
                pub const m_nNode: usize = 0x40; // uint16[4]
            };
            // Parent: None
            // Field count: 2
            pub const VertexPositionNormal_t = struct {
                pub const m_vPosition: usize = 0x0; // Vector
                pub const m_vNormal: usize = 0xC; // Vector
            };
            // Parent: None
            // Field count: 0
            pub const IPhysicsRagdollControl = struct {
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FeCtrlSoftOffset_t = struct {
                pub const nCtrlParent: usize = 0x0; // uint16
                pub const nCtrlChild: usize = 0x2; // uint16
                pub const vOffset: usize = 0x4; // Vector
                pub const flAlpha: usize = 0x10; // float32
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const FeAnimStrayRadius_t = struct {
                pub const nNode: usize = 0x0; // uint16[2]
                pub const flMaxDist: usize = 0x4; // float32
                pub const flRelaxationFactor: usize = 0x8; // float32
            };
            // Parent: None
            // Field count: 0
            pub const IPhysicsJoint = struct {
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const FeEdgeDesc_t = struct {
                pub const nEdge: usize = 0x0; // uint16[2]
                pub const nSide: usize = 0x4; // uint16[2][2]
                pub const nVirtElem: usize = 0xC; // uint16[2]
            };
            // Parent: None
            // Field count: 1
            pub const CGenericShapeProxy = struct {
                pub const m_verts: usize = 0x30; // CUtlLeanVectorFixedGrowable<Vector,8>
            };
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const FeHingeLimit_t = struct {
                pub const nNode: usize = 0x0; // uint16[6]
                pub const nFlags: usize = 0xC; // uint32
                pub const flWeight4: usize = 0x10; // float32
                pub const flWeight5: usize = 0x14; // float32
                pub const flAngleCenter: usize = 0x18; // float32
                pub const flAngleExtents: usize = 0x1C; // float32
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const FeWeightedNode_t = struct {
                pub const nNode: usize = 0x0; // uint16
                pub const nWeight: usize = 0x2; // uint16
            };
            // Parent: None
            // Field count: 0
            pub const IPhysicsPlayerController = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const RnVertex_t = struct {
                pub const m_nEdge: usize = 0x0; // uint8
            };
            // Parent: None
            // Field count: 0
            pub const IPhysicsMotionController = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FeDynKinLink_t = struct {
                pub const m_nParent: usize = 0x0; // uint16
                pub const m_nChild: usize = 0x2; // uint16
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const RnFace_t = struct {
                pub const m_nEdge: usize = 0x0; // uint8
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FeSimdPrism_t = struct {
                pub const nNode: usize = 0x0; // uint16[4][6]
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FeSourceEdge_t = struct {
                pub const nNode: usize = 0x0; // uint16[2]
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FeNodeWindBase_t = struct {
                pub const nNodeX0: usize = 0x0; // uint16
                pub const nNodeX1: usize = 0x2; // uint16
                pub const nNodeY0: usize = 0x4; // uint16
                pub const nNodeY1: usize = 0x6; // uint16
            };
            // Parent: None
            // Field count: 5
            pub const constraint_breakableparams_t = struct {
                pub const strength: usize = 0x0; // float32
                pub const forceLimit: usize = 0x4; // float32
                pub const torqueLimit: usize = 0x8; // float32
                pub const bodyMassScale: usize = 0xC; // float32[2]
                pub const isActive: usize = 0x14; // bool
            };
            // Parent: None
            // Field count: 0
            pub const IPhysicsBody = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const FeTreeChildren_t = struct {
                pub const nChild: usize = 0x0; // uint16[2]
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const RnHalfEdge_t = struct {
                pub const m_nNext: usize = 0x0; // uint8
                pub const m_nTwin: usize = 0x1; // uint8
                pub const m_nOrigin: usize = 0x2; // uint8
                pub const m_nFace: usize = 0x3; // uint8
            };
            // Parent: None
            // Field count: 1
            pub const VertexPositionColor_t = struct {
                pub const m_vPosition: usize = 0x0; // Vector
            };
        };
    };
};
