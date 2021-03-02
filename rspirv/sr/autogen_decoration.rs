// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use spirv;
#[doc = r" SPIR-V decorations."]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId(u32),
    Block,
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride(u32),
    MatrixStride(u32),
    GLSLShared,
    GLSLPacked,
    CPacked,
    BuiltIn(spirv::BuiltIn),
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    UniformId(spirv::Word),
    SaturatedConversion,
    Stream(u32),
    Location(u32),
    Component(u32),
    Index(u32),
    Binding(u32),
    DescriptorSet(u32),
    Offset(u32),
    XfbBuffer(u32),
    XfbStride(u32),
    FuncParamAttr(spirv::FunctionParameterAttribute),
    FPRoundingMode(spirv::FPRoundingMode),
    FPFastMathMode(spirv::FPFastMathMode),
    LinkageAttributes(String, spirv::LinkageType),
    NoContraction,
    InputAttachmentIndex(u32),
    Alignment(u32),
    MaxByteOffset(u32),
    AlignmentId(spirv::Word),
    MaxByteOffsetId(spirv::Word),
    NoSignedWrap,
    NoUnsignedWrap,
    ExplicitInterpAMD,
    OverrideCoverageNV,
    PassthroughNV,
    ViewportRelativeNV,
    SecondaryViewportRelativeNV(u32),
    PerPrimitiveNV,
    PerViewNV,
    PerTaskNV,
    PerVertexNV,
    NonUniform,
    NonUniformEXT,
    RestrictPointer,
    RestrictPointerEXT,
    AliasedPointer,
    AliasedPointerEXT,
    SIMTCallINTEL(u32),
    ReferencedIndirectlyINTEL,
    ClobberINTEL(String),
    SideEffectsINTEL,
    VectorComputeVariableINTEL,
    FuncParamIOKindINTEL(u32),
    VectorComputeFunctionINTEL,
    StackCallINTEL,
    GlobalVariableOffsetINTEL(u32),
    CounterBuffer(spirv::Word),
    HlslCounterBufferGOOGLE(spirv::Word),
    UserSemantic(String),
    HlslSemanticGOOGLE(String),
    UserTypeGOOGLE(String),
    FunctionRoundingModeINTEL(u32, spirv::FPRoundingMode),
    FunctionDenormModeINTEL(u32, spirv::FPDenormMode),
    RegisterINTEL,
    MemoryINTEL(String),
    NumbanksINTEL(u32),
    BankwidthINTEL(u32),
    MaxPrivateCopiesINTEL(u32),
    SinglepumpINTEL,
    DoublepumpINTEL,
    MaxReplicatesINTEL(u32),
    SimpleDualPortINTEL,
    MergeINTEL(String, String),
    BankBitsINTEL(Vec<u32>),
    ForcePow2DepthINTEL(u32),
    BurstCoalesceINTEL,
    CacheSizeINTEL(u32),
    DontStaticallyCoalesceINTEL,
    PrefetchINTEL(u32),
    StallEnableINTEL,
    FuseLoopsInFunctionINTEL,
    BufferLocationINTEL(u32),
    IOPipeStorageINTEL(u32),
    FunctionFloatingPointModeINTEL(u32, spirv::FPOperationMode),
    SingleElementVectorINTEL,
    VectorComputeCallableFunctionINTEL,
}
