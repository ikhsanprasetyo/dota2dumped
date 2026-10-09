// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: vphysics2.dll
        // Class count: 30
        // Enum count: 5
        pub mod vphysics2_dll {
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum JointMotion_t {
                JOINT_MOTION_FREE = 0x0,
                JOINT_MOTION_LOCKED = 0x1,
                JOINT_MOTION_COUNT = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum JointAxis_t {
                JOINT_AXIS_X = 0x0,
                JOINT_AXIS_Y = 0x1,
                JOINT_AXIS_Z = 0x2,
                JOINT_AXIS_COUNT = 0x3
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum DynamicContinuousContactBehavior_t {
                DYNAMIC_CONTINUOUS_ALLOW_IF_REQUESTED_BY_OTHER_BODY = 0x0,
                DYNAMIC_CONTINUOUS_ALWAYS = 0x1,
                DYNAMIC_CONTINUOUS_NEVER = 0x2
            }
            // Alignment: 4
            // Member count: 8
            #[repr(u32)]
            pub enum PhysInterfaceId_t {
                PIID_UNKNOWN = 0x0,
                PIID_IPHYSICSBODY = 0x1,
                PIID_IPHYSAGGREGATE = 0x2,
                PIID_IPHYSICSJOINT = 0x3,
                PIID_IPHYSICSMOTIONCONTROLLER = 0x4,
                PIID_IPHYSICSPARTICLEROPE = 0x5,
                PIID_IPHYSICSRAGDOLLCONTROL = 0x6,
                PIID_NUM_TYPES = 0x7
            }
            // Alignment: 1
            // Member count: 5
            #[repr(u8)]
            pub enum PhysGenericShapeType_t {
                GENERIC_SHAPE_POINT = 0x0,
                GENERIC_SHAPE_SPHERE = 0x1,
                GENERIC_SHAPE_AABB = 0x2,
                GENERIC_SHAPE_CAPSULE = 0x3,
                GENERIC_SHAPE_HULL = 0x4
            }
            // Parent: None
            // Field count: 2
            pub mod IPhysAggregateInstance {
                pub const m_pSkeleton: usize = 0x8; // void*
                pub const m_bIsAxisAligned: usize = 0x10; // bool
            }
            // Parent: None
            // Field count: 4
            pub mod constraint_axislimit_t {
                pub const flMinRotation: usize = 0x0; // float32
                pub const flMaxRotation: usize = 0x4; // float32
                pub const flMotorTargetAngSpeed: usize = 0x8; // float32
                pub const flMotorMaxTorque: usize = 0xC; // float32
            }
            // Parent: None
            // Field count: 0
            pub mod IPhysicsParticleRope {
            }
            // Parent: None
            // Field count: 0
            pub mod IPhysicsBodyList {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod RnTriangle_t {
                pub const m_nIndex: usize = 0x0; // int32[3]
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod FeProxyVertexMap_t {
                pub const m_Name: usize = 0x0; // CUtlString
                pub const m_flWeight: usize = 0x8; // float32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod FePrism_t {
                pub const nNode: usize = 0x0; // uint16[6]
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod OldFeEdge_t {
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
            }
            // Parent: None
            // Field count: 2
            pub mod VertexPositionNormal_t {
                pub const m_vPosition: usize = 0x0; // Vector
                pub const m_vNormal: usize = 0xC; // Vector
            }
            // Parent: None
            // Field count: 0
            pub mod IPhysicsRagdollControl {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod FeCtrlSoftOffset_t {
                pub const nCtrlParent: usize = 0x0; // uint16
                pub const nCtrlChild: usize = 0x2; // uint16
                pub const vOffset: usize = 0x4; // Vector
                pub const flAlpha: usize = 0x10; // float32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod FeAnimStrayRadius_t {
                pub const nNode: usize = 0x0; // uint16[2]
                pub const flMaxDist: usize = 0x4; // float32
                pub const flRelaxationFactor: usize = 0x8; // float32
            }
            // Parent: None
            // Field count: 0
            pub mod IPhysicsJoint {
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod FeEdgeDesc_t {
                pub const nEdge: usize = 0x0; // uint16[2]
                pub const nSide: usize = 0x4; // uint16[2][2]
                pub const nVirtElem: usize = 0xC; // uint16[2]
            }
            // Parent: None
            // Field count: 1
            pub mod CGenericShapeProxy {
                pub const m_verts: usize = 0x30; // CUtlLeanVectorFixedGrowable<Vector,8>
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod FeHingeLimit_t {
                pub const nNode: usize = 0x0; // uint16[6]
                pub const nFlags: usize = 0xC; // uint32
                pub const flWeight4: usize = 0x10; // float32
                pub const flWeight5: usize = 0x14; // float32
                pub const flAngleCenter: usize = 0x18; // float32
                pub const flAngleExtents: usize = 0x1C; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod FeWeightedNode_t {
                pub const nNode: usize = 0x0; // uint16
                pub const nWeight: usize = 0x2; // uint16
            }
            // Parent: None
            // Field count: 0
            pub mod IPhysicsPlayerController {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod RnVertex_t {
                pub const m_nEdge: usize = 0x0; // uint8
            }
            // Parent: None
            // Field count: 0
            pub mod IPhysicsMotionController {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod FeDynKinLink_t {
                pub const m_nParent: usize = 0x0; // uint16
                pub const m_nChild: usize = 0x2; // uint16
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod RnFace_t {
                pub const m_nEdge: usize = 0x0; // uint8
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod FeSimdPrism_t {
                pub const nNode: usize = 0x0; // uint16[4][6]
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod FeSourceEdge_t {
                pub const nNode: usize = 0x0; // uint16[2]
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod FeNodeWindBase_t {
                pub const nNodeX0: usize = 0x0; // uint16
                pub const nNodeX1: usize = 0x2; // uint16
                pub const nNodeY0: usize = 0x4; // uint16
                pub const nNodeY1: usize = 0x6; // uint16
            }
            // Parent: None
            // Field count: 5
            pub mod constraint_breakableparams_t {
                pub const strength: usize = 0x0; // float32
                pub const forceLimit: usize = 0x4; // float32
                pub const torqueLimit: usize = 0x8; // float32
                pub const bodyMassScale: usize = 0xC; // float32[2]
                pub const isActive: usize = 0x14; // bool
            }
            // Parent: None
            // Field count: 0
            pub mod IPhysicsBody {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod FeTreeChildren_t {
                pub const nChild: usize = 0x0; // uint16[2]
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod RnHalfEdge_t {
                pub const m_nNext: usize = 0x0; // uint8
                pub const m_nTwin: usize = 0x1; // uint8
                pub const m_nOrigin: usize = 0x2; // uint8
                pub const m_nFace: usize = 0x3; // uint8
            }
            // Parent: None
            // Field count: 1
            pub mod VertexPositionColor_t {
                pub const m_vPosition: usize = 0x0; // Vector
            }
        }
    }
}
