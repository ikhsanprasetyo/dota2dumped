# Generated using https://github.com/ikhsanprasetyo/source2-dumper
# 2026-09-30 23:53:47.474156300 +07:00

class Schemas:
    # Module: vphysics2.dll
    class Vphysics2Dll:
        class JointMotion_t:
            JOINT_MOTION_FREE = 0x0
            JOINT_MOTION_LOCKED = 0x1
            JOINT_MOTION_COUNT = 0x2
        class JointAxis_t:
            JOINT_AXIS_X = 0x0
            JOINT_AXIS_Y = 0x1
            JOINT_AXIS_Z = 0x2
            JOINT_AXIS_COUNT = 0x3
        class DynamicContinuousContactBehavior_t:
            DYNAMIC_CONTINUOUS_ALLOW_IF_REQUESTED_BY_OTHER_BODY = 0x0
            DYNAMIC_CONTINUOUS_ALWAYS = 0x1
            DYNAMIC_CONTINUOUS_NEVER = 0x2
        class PhysInterfaceId_t:
            PIID_UNKNOWN = 0x0
            PIID_IPHYSICSBODY = 0x1
            PIID_IPHYSAGGREGATE = 0x2
            PIID_IPHYSICSJOINT = 0x3
            PIID_IPHYSICSMOTIONCONTROLLER = 0x4
            PIID_IPHYSICSPARTICLEROPE = 0x5
            PIID_IPHYSICSRAGDOLLCONTROL = 0x6
            PIID_NUM_TYPES = 0x7
        class PhysGenericShapeType_t:
            GENERIC_SHAPE_POINT = 0x0
            GENERIC_SHAPE_SPHERE = 0x1
            GENERIC_SHAPE_AABB = 0x2
            GENERIC_SHAPE_CAPSULE = 0x3
            GENERIC_SHAPE_HULL = 0x4
        class IPhysAggregateInstance:
            m_pSkeleton = 0x8 # void*
            m_bIsAxisAligned = 0x10 # bool
        class constraint_axislimit_t:
            flMinRotation = 0x0 # float32
            flMaxRotation = 0x4 # float32
            flMotorTargetAngSpeed = 0x8 # float32
            flMotorMaxTorque = 0xC # float32
        class IPhysicsParticleRope:
            pass
        class IPhysicsBodyList:
            pass
        class RnTriangle_t:
            m_nIndex = 0x0 # int32[3]
        class OldFeEdge_t:
            m_flK = 0x0 # float32[3]
            invA = 0xC # float32
            t = 0x10 # float32
            flThetaRelaxed = 0x14 # float32
            flThetaFactor = 0x18 # float32
            c01 = 0x1C # float32
            c02 = 0x20 # float32
            c03 = 0x24 # float32
            c04 = 0x28 # float32
            flAxialModelDist = 0x2C # float32
            flAxialModelWeights = 0x30 # float32[4]
            m_nNode = 0x40 # uint16[4]
        class VertexPositionNormal_t:
            m_vPosition = 0x0 # Vector
            m_vNormal = 0xC # Vector
        class IPhysicsRagdollControl:
            pass
        class FeAnimStrayRadius_t:
            nNode = 0x0 # uint16[2]
            flMaxDist = 0x4 # float32
            flRelaxationFactor = 0x8 # float32
        class IPhysicsJoint:
            pass
        class FeEdgeDesc_t:
            nEdge = 0x0 # uint16[2]
            nSide = 0x4 # uint16[2][2]
            nVirtElem = 0xC # uint16[2]
        class CGenericShapeProxy:
            m_verts = 0x30 # CUtlLeanVectorFixedGrowable<Vector,8>
        class FeHingeLimit_t:
            nNode = 0x0 # uint16[6]
            nFlags = 0xC # uint32
            flWeight4 = 0x10 # float32
            flWeight5 = 0x14 # float32
            flAngleCenter = 0x18 # float32
            flAngleExtents = 0x1C # float32
        class IPhysicsPlayerController:
            pass
        class RnVertex_t:
            m_nEdge = 0x0 # uint8
        class IPhysicsMotionController:
            pass
        class FeDynKinLink_t:
            m_nParent = 0x0 # uint16
            m_nChild = 0x2 # uint16
        class RnFace_t:
            m_nEdge = 0x0 # uint8
        class FeNodeWindBase_t:
            nNodeX0 = 0x0 # uint16
            nNodeX1 = 0x2 # uint16
            nNodeY0 = 0x4 # uint16
            nNodeY1 = 0x6 # uint16
        class constraint_breakableparams_t:
            strength = 0x0 # float32
            forceLimit = 0x4 # float32
            torqueLimit = 0x8 # float32
            bodyMassScale = 0xC # float32[2]
            isActive = 0x14 # bool
        class IPhysicsBody:
            pass
        class FeTreeChildren_t:
            nChild = 0x0 # uint16[2]
        class RnHalfEdge_t:
            m_nNext = 0x0 # uint8
            m_nTwin = 0x1 # uint8
            m_nOrigin = 0x2 # uint8
            m_nFace = 0x3 # uint8
        class VertexPositionColor_t:
            m_vPosition = 0x0 # Vector
