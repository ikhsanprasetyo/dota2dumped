// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

namespace Source2Dumper.Schemas {
    // Module: vphysics2.dll
    // Class count: 30
    // Enum count: 5
    public static class Vphysics2Dll {
        // Alignment: 4
        // Member count: 3
        public enum JointMotion_t : uint {
            JOINT_MOTION_FREE = 0x0,
            JOINT_MOTION_LOCKED = 0x1,
            JOINT_MOTION_COUNT = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum JointAxis_t : uint {
            JOINT_AXIS_X = 0x0,
            JOINT_AXIS_Y = 0x1,
            JOINT_AXIS_Z = 0x2,
            JOINT_AXIS_COUNT = 0x3
        }
        // Alignment: 1
        // Member count: 3
        public enum DynamicContinuousContactBehavior_t : byte {
            DYNAMIC_CONTINUOUS_ALLOW_IF_REQUESTED_BY_OTHER_BODY = 0x0,
            DYNAMIC_CONTINUOUS_ALWAYS = 0x1,
            DYNAMIC_CONTINUOUS_NEVER = 0x2
        }
        // Alignment: 4
        // Member count: 8
        public enum PhysInterfaceId_t : uint {
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
        public enum PhysGenericShapeType_t : byte {
            GENERIC_SHAPE_POINT = 0x0,
            GENERIC_SHAPE_SPHERE = 0x1,
            GENERIC_SHAPE_AABB = 0x2,
            GENERIC_SHAPE_CAPSULE = 0x3,
            GENERIC_SHAPE_HULL = 0x4
        }
        // Parent: None
        // Field count: 2
        public static class IPhysAggregateInstance {
            public const nint m_pSkeleton = 0x8; // void*
            public const nint m_bIsAxisAligned = 0x10; // bool
        }
        // Parent: None
        // Field count: 4
        public static class constraint_axislimit_t {
            public const nint flMinRotation = 0x0; // float32
            public const nint flMaxRotation = 0x4; // float32
            public const nint flMotorTargetAngSpeed = 0x8; // float32
            public const nint flMotorMaxTorque = 0xC; // float32
        }
        // Parent: None
        // Field count: 0
        public static class IPhysicsParticleRope {
        }
        // Parent: None
        // Field count: 0
        public static class IPhysicsBodyList {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class RnTriangle_t {
            public const nint m_nIndex = 0x0; // int32[3]
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class FeProxyVertexMap_t {
            public const nint m_Name = 0x0; // CUtlString
            public const nint m_flWeight = 0x8; // float32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class FePrism_t {
            public const nint nNode = 0x0; // uint16[6]
        }
        // Parent: None
        // Field count: 12
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class OldFeEdge_t {
            public const nint m_flK = 0x0; // float32[3]
            public const nint invA = 0xC; // float32
            public const nint t = 0x10; // float32
            public const nint flThetaRelaxed = 0x14; // float32
            public const nint flThetaFactor = 0x18; // float32
            public const nint c01 = 0x1C; // float32
            public const nint c02 = 0x20; // float32
            public const nint c03 = 0x24; // float32
            public const nint c04 = 0x28; // float32
            public const nint flAxialModelDist = 0x2C; // float32
            public const nint flAxialModelWeights = 0x30; // float32[4]
            public const nint m_nNode = 0x40; // uint16[4]
        }
        // Parent: None
        // Field count: 2
        public static class VertexPositionNormal_t {
            public const nint m_vPosition = 0x0; // Vector
            public const nint m_vNormal = 0xC; // Vector
        }
        // Parent: None
        // Field count: 0
        public static class IPhysicsRagdollControl {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class FeCtrlSoftOffset_t {
            public const nint nCtrlParent = 0x0; // uint16
            public const nint nCtrlChild = 0x2; // uint16
            public const nint vOffset = 0x4; // Vector
            public const nint flAlpha = 0x10; // float32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class FeAnimStrayRadius_t {
            public const nint nNode = 0x0; // uint16[2]
            public const nint flMaxDist = 0x4; // float32
            public const nint flRelaxationFactor = 0x8; // float32
        }
        // Parent: None
        // Field count: 0
        public static class IPhysicsJoint {
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class FeEdgeDesc_t {
            public const nint nEdge = 0x0; // uint16[2]
            public const nint nSide = 0x4; // uint16[2][2]
            public const nint nVirtElem = 0xC; // uint16[2]
        }
        // Parent: None
        // Field count: 1
        public static class CGenericShapeProxy {
            public const nint m_verts = 0x30; // CUtlLeanVectorFixedGrowable<Vector,8>
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class FeHingeLimit_t {
            public const nint nNode = 0x0; // uint16[6]
            public const nint nFlags = 0xC; // uint32
            public const nint flWeight4 = 0x10; // float32
            public const nint flWeight5 = 0x14; // float32
            public const nint flAngleCenter = 0x18; // float32
            public const nint flAngleExtents = 0x1C; // float32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class FeWeightedNode_t {
            public const nint nNode = 0x0; // uint16
            public const nint nWeight = 0x2; // uint16
        }
        // Parent: None
        // Field count: 0
        public static class IPhysicsPlayerController {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class RnVertex_t {
            public const nint m_nEdge = 0x0; // uint8
        }
        // Parent: None
        // Field count: 0
        public static class IPhysicsMotionController {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class FeDynKinLink_t {
            public const nint m_nParent = 0x0; // uint16
            public const nint m_nChild = 0x2; // uint16
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class RnFace_t {
            public const nint m_nEdge = 0x0; // uint8
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class FeSimdPrism_t {
            public const nint nNode = 0x0; // uint16[4][6]
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class FeSourceEdge_t {
            public const nint nNode = 0x0; // uint16[2]
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class FeNodeWindBase_t {
            public const nint nNodeX0 = 0x0; // uint16
            public const nint nNodeX1 = 0x2; // uint16
            public const nint nNodeY0 = 0x4; // uint16
            public const nint nNodeY1 = 0x6; // uint16
        }
        // Parent: None
        // Field count: 5
        public static class constraint_breakableparams_t {
            public const nint strength = 0x0; // float32
            public const nint forceLimit = 0x4; // float32
            public const nint torqueLimit = 0x8; // float32
            public const nint bodyMassScale = 0xC; // float32[2]
            public const nint isActive = 0x14; // bool
        }
        // Parent: None
        // Field count: 0
        public static class IPhysicsBody {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class FeTreeChildren_t {
            public const nint nChild = 0x0; // uint16[2]
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class RnHalfEdge_t {
            public const nint m_nNext = 0x0; // uint8
            public const nint m_nTwin = 0x1; // uint8
            public const nint m_nOrigin = 0x2; // uint8
            public const nint m_nFace = 0x3; // uint8
        }
        // Parent: None
        // Field count: 1
        public static class VertexPositionColor_t {
            public const nint m_vPosition = 0x0; // Vector
        }
    }
}
