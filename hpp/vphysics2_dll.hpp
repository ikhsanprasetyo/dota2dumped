// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-10 12:12:06.506827800 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: vphysics2.dll
        // Class count: 30
        // Enum count: 5
        namespace vphysics2_dll {
            // Alignment: 4
            // Member count: 3
            enum class JointMotion_t : uint32_t {
                JOINT_MOTION_FREE = 0x0,
                JOINT_MOTION_LOCKED = 0x1,
                JOINT_MOTION_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class JointAxis_t : uint32_t {
                JOINT_AXIS_X = 0x0,
                JOINT_AXIS_Y = 0x1,
                JOINT_AXIS_Z = 0x2,
                JOINT_AXIS_COUNT = 0x3
            };
            // Alignment: 1
            // Member count: 3
            enum class DynamicContinuousContactBehavior_t : uint8_t {
                DYNAMIC_CONTINUOUS_ALLOW_IF_REQUESTED_BY_OTHER_BODY = 0x0,
                DYNAMIC_CONTINUOUS_ALWAYS = 0x1,
                DYNAMIC_CONTINUOUS_NEVER = 0x2
            };
            // Alignment: 4
            // Member count: 8
            enum class PhysInterfaceId_t : uint32_t {
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
            enum class PhysGenericShapeType_t : uint8_t {
                GENERIC_SHAPE_POINT = 0x0,
                GENERIC_SHAPE_SPHERE = 0x1,
                GENERIC_SHAPE_AABB = 0x2,
                GENERIC_SHAPE_CAPSULE = 0x3,
                GENERIC_SHAPE_HULL = 0x4
            };
            // Parent: None
            // Field count: 2
            namespace IPhysAggregateInstance {
                constexpr std::ptrdiff_t m_pSkeleton = 0x8; // void*
                constexpr std::ptrdiff_t m_bIsAxisAligned = 0x10; // bool
            }
            // Parent: None
            // Field count: 4
            namespace constraint_axislimit_t {
                constexpr std::ptrdiff_t flMinRotation = 0x0; // float32
                constexpr std::ptrdiff_t flMaxRotation = 0x4; // float32
                constexpr std::ptrdiff_t flMotorTargetAngSpeed = 0x8; // float32
                constexpr std::ptrdiff_t flMotorMaxTorque = 0xC; // float32
            }
            // Parent: None
            // Field count: 0
            namespace IPhysicsParticleRope {
            }
            // Parent: None
            // Field count: 0
            namespace IPhysicsBodyList {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace RnTriangle_t {
                constexpr std::ptrdiff_t m_nIndex = 0x0; // int32[3]
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace FeProxyVertexMap_t {
                constexpr std::ptrdiff_t m_Name = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_flWeight = 0x8; // float32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace FePrism_t {
                constexpr std::ptrdiff_t nNode = 0x0; // uint16[6]
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace OldFeEdge_t {
                constexpr std::ptrdiff_t m_flK = 0x0; // float32[3]
                constexpr std::ptrdiff_t invA = 0xC; // float32
                constexpr std::ptrdiff_t t = 0x10; // float32
                constexpr std::ptrdiff_t flThetaRelaxed = 0x14; // float32
                constexpr std::ptrdiff_t flThetaFactor = 0x18; // float32
                constexpr std::ptrdiff_t c01 = 0x1C; // float32
                constexpr std::ptrdiff_t c02 = 0x20; // float32
                constexpr std::ptrdiff_t c03 = 0x24; // float32
                constexpr std::ptrdiff_t c04 = 0x28; // float32
                constexpr std::ptrdiff_t flAxialModelDist = 0x2C; // float32
                constexpr std::ptrdiff_t flAxialModelWeights = 0x30; // float32[4]
                constexpr std::ptrdiff_t m_nNode = 0x40; // uint16[4]
            }
            // Parent: None
            // Field count: 2
            namespace VertexPositionNormal_t {
                constexpr std::ptrdiff_t m_vPosition = 0x0; // Vector
                constexpr std::ptrdiff_t m_vNormal = 0xC; // Vector
            }
            // Parent: None
            // Field count: 0
            namespace IPhysicsRagdollControl {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace FeCtrlSoftOffset_t {
                constexpr std::ptrdiff_t nCtrlParent = 0x0; // uint16
                constexpr std::ptrdiff_t nCtrlChild = 0x2; // uint16
                constexpr std::ptrdiff_t vOffset = 0x4; // Vector
                constexpr std::ptrdiff_t flAlpha = 0x10; // float32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace FeAnimStrayRadius_t {
                constexpr std::ptrdiff_t nNode = 0x0; // uint16[2]
                constexpr std::ptrdiff_t flMaxDist = 0x4; // float32
                constexpr std::ptrdiff_t flRelaxationFactor = 0x8; // float32
            }
            // Parent: None
            // Field count: 0
            namespace IPhysicsJoint {
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace FeEdgeDesc_t {
                constexpr std::ptrdiff_t nEdge = 0x0; // uint16[2]
                constexpr std::ptrdiff_t nSide = 0x4; // uint16[2][2]
                constexpr std::ptrdiff_t nVirtElem = 0xC; // uint16[2]
            }
            // Parent: None
            // Field count: 1
            namespace CGenericShapeProxy {
                constexpr std::ptrdiff_t m_verts = 0x30; // CUtlLeanVectorFixedGrowable<Vector,8>
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace FeHingeLimit_t {
                constexpr std::ptrdiff_t nNode = 0x0; // uint16[6]
                constexpr std::ptrdiff_t nFlags = 0xC; // uint32
                constexpr std::ptrdiff_t flWeight4 = 0x10; // float32
                constexpr std::ptrdiff_t flWeight5 = 0x14; // float32
                constexpr std::ptrdiff_t flAngleCenter = 0x18; // float32
                constexpr std::ptrdiff_t flAngleExtents = 0x1C; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace FeWeightedNode_t {
                constexpr std::ptrdiff_t nNode = 0x0; // uint16
                constexpr std::ptrdiff_t nWeight = 0x2; // uint16
            }
            // Parent: None
            // Field count: 0
            namespace IPhysicsPlayerController {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace RnVertex_t {
                constexpr std::ptrdiff_t m_nEdge = 0x0; // uint8
            }
            // Parent: None
            // Field count: 0
            namespace IPhysicsMotionController {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace FeDynKinLink_t {
                constexpr std::ptrdiff_t m_nParent = 0x0; // uint16
                constexpr std::ptrdiff_t m_nChild = 0x2; // uint16
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace RnFace_t {
                constexpr std::ptrdiff_t m_nEdge = 0x0; // uint8
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace FeSimdPrism_t {
                constexpr std::ptrdiff_t nNode = 0x0; // uint16[4][6]
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace FeSourceEdge_t {
                constexpr std::ptrdiff_t nNode = 0x0; // uint16[2]
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace FeNodeWindBase_t {
                constexpr std::ptrdiff_t nNodeX0 = 0x0; // uint16
                constexpr std::ptrdiff_t nNodeX1 = 0x2; // uint16
                constexpr std::ptrdiff_t nNodeY0 = 0x4; // uint16
                constexpr std::ptrdiff_t nNodeY1 = 0x6; // uint16
            }
            // Parent: None
            // Field count: 5
            namespace constraint_breakableparams_t {
                constexpr std::ptrdiff_t strength = 0x0; // float32
                constexpr std::ptrdiff_t forceLimit = 0x4; // float32
                constexpr std::ptrdiff_t torqueLimit = 0x8; // float32
                constexpr std::ptrdiff_t bodyMassScale = 0xC; // float32[2]
                constexpr std::ptrdiff_t isActive = 0x14; // bool
            }
            // Parent: None
            // Field count: 0
            namespace IPhysicsBody {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace FeTreeChildren_t {
                constexpr std::ptrdiff_t nChild = 0x0; // uint16[2]
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace RnHalfEdge_t {
                constexpr std::ptrdiff_t m_nNext = 0x0; // uint8
                constexpr std::ptrdiff_t m_nTwin = 0x1; // uint8
                constexpr std::ptrdiff_t m_nOrigin = 0x2; // uint8
                constexpr std::ptrdiff_t m_nFace = 0x3; // uint8
            }
            // Parent: None
            // Field count: 1
            namespace VertexPositionColor_t {
                constexpr std::ptrdiff_t m_vPosition = 0x0; // Vector
            }
        }
    }
}
