// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

#[derive(Clone, Debug)]
pub enum Type {
    Void,
    Bool,
    Int {
        width: u32,
        signedness: u32,
    },
    Float {
        width: u32,
    },
    Vector {
        component_type: Token<Type>,
        component_count: u32,
    },
    Matrix {
        column_type: Token<Type>,
        column_count: u32,
    },
    Image {
        sampled_type: Token<Type>,
        dim: spirv::Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        image_format: spirv::ImageFormat,
        access_qualifier: Option<spirv::AccessQualifier>,
    },
    Sampler,
    SampledImage {
        image_type: Token<Type>,
    },
    Array {
        element_type: Token<Type>,
        length: Token<Constant>,
    },
    RuntimeArray {
        element_type: Token<Type>,
    },
    Struct {
        member_0_type_member_1_type: Vec<StructMember>,
    },
    Opaque {
        type_name: String,
    },
    Pointer {
        storage_class: spirv::StorageClass,
        ty: spirv::Word,
    },
    Function {
        return_type: Token<Type>,
        parameter_0_type_parameter_1_type: Vec<Token<Type>>,
    },
    Event,
    DeviceEvent,
    ReserveId,
    Queue,
    Pipe {
        qualifier: spirv::AccessQualifier,
    },
    ForwardPointer {
        pointer_type: Token<Type>,
        storage_class: spirv::StorageClass,
    },
    PipeStorage,
    NamedBarrier,
    AccelerationStructureNV,
    CooperativeMatrixNV {
        component_type: Token<Type>,
        execution: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
    },
    VmeImageINTEL {
        image_type: Token<Type>,
    },
    AvcImePayloadINTEL,
    AvcRefPayloadINTEL,
    AvcSicPayloadINTEL,
    AvcMcePayloadINTEL,
    AvcMceResultINTEL,
    AvcImeResultINTEL,
    AvcImeResultSingleReferenceStreamoutINTEL,
    AvcImeResultDualReferenceStreamoutINTEL,
    AvcImeSingleReferenceStreaminINTEL,
    AvcImeDualReferenceStreaminINTEL,
    AvcRefResultINTEL,
    AvcSicResultINTEL,
}
