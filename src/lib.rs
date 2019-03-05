use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use slang_sys as sys;
use std::ffi::CStr;
use std::ffi::CString;
use std::marker::PhantomData;
use std::path::Path;
use std::ptr;
use std::slice;

fn result_succeeded(r: i32) -> bool {
    r >= 0
}

fn result_failed(r: i32) -> bool {
    r < 0
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CompileTarget {
    TargetUnknown = sys::SLANG_TARGET_UNKNOWN,
    TargetNone = sys::SLANG_TARGET_NONE,
    Glsl = sys::SLANG_GLSL,
    //GlslVulkan = sys::SLANG_GLSL_VULKAN,          //< deprecated: just use `SLANG_GLSL`
    //GlslVulkanOneDesc = sys::SLANG_GLSL_VULKAN_ONE_DESC, //< deprecated
    Hlsl = sys::SLANG_HLSL,
    Spirv = sys::SLANG_SPIRV,
    SpirvAsm = sys::SLANG_SPIRV_ASM,
    Dxbc = sys::SLANG_DXBC,
    DxbcAsm = sys::SLANG_DXBC_ASM,
    Dxil = sys::SLANG_DXIL,
    DxilAsm = sys::SLANG_DXIL_ASM,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Stage {
    None = sys::SLANG_STAGE_NONE as u32,
    Vertex = sys::SLANG_STAGE_VERTEX as u32,
    Hull = sys::SLANG_STAGE_HULL as u32,
    Domain = sys::SLANG_STAGE_DOMAIN as u32,
    Geometry = sys::SLANG_STAGE_GEOMETRY as u32,
    Fragment = sys::SLANG_STAGE_FRAGMENT as u32,
    Compute = sys::SLANG_STAGE_COMPUTE as u32,
    RayGeneration = sys::SLANG_STAGE_RAY_GENERATION as u32,
    Intersection = sys::SLANG_STAGE_INTERSECTION as u32,
    AnyHit = sys::SLANG_STAGE_ANY_HIT as u32,
    ClosestHit = sys::SLANG_STAGE_CLOSEST_HIT as u32,
    Miss = sys::SLANG_STAGE_MISS as u32,
    Callable = sys::SLANG_STAGE_CALLABLE as u32,
    //Pixel  = sys::SLANG_STAGE_FRAGMENT,
}
/*
impl Stage {
    pub fn from_raw(v: u32) -> Option<Stage> {
        match v as i32 {
            sys::SLANG_STAGE_NONE => Some(Stage::None),
            sys::SLANG_STAGE_VERTEX => Some(Stage::Vertex),
            sys::SLANG_STAGE_HULL => Some(Stage::Hull),
            sys::SLANG_STAGE_DOMAIN => Some(Stage::Domain),
            sys::SLANG_STAGE_GEOMETRY => Some(Stage::Geometry),
            sys::SLANG_STAGE_FRAGMENT => Some(Stage::Fragment),
            sys::SLANG_STAGE_COMPUTE => Some(Stage::Compute),
            sys::SLANG_STAGE_RAY_GENERATION => Some(Stage::RayGeneration),
            sys::SLANG_STAGE_INTERSECTION => Some(Stage::Intersection),
            sys::SLANG_STAGE_ANY_HIT => Some(Stage::AnyHit),
            sys::SLANG_STAGE_CLOSEST_HIT => Some(Stage::ClosestHit),
            sys::SLANG_STAGE_MISS => Some(Stage::Miss),
            sys::SLANG_STAGE_CALLABLE => Some(Stage::Callable),
            _ => None
        }
    }
}*/

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SourceLanguage {
    Unknown = sys::SLANG_SOURCE_LANGUAGE_UNKNOWN,
    Slang = sys::SLANG_SOURCE_LANGUAGE_SLANG,
    Hlsl = sys::SLANG_SOURCE_LANGUAGE_HLSL,
    Glsl = sys::SLANG_SOURCE_LANGUAGE_GLSL,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum TypeKind {
    None = sys::SLANG_TYPE_KIND_NONE as u32,
    Struct = sys::SLANG_TYPE_KIND_STRUCT as u32,
    Array = sys::SLANG_TYPE_KIND_ARRAY as u32,
    Matrix = sys::SLANG_TYPE_KIND_MATRIX as u32,
    Vector = sys::SLANG_TYPE_KIND_VECTOR as u32,
    Scalar = sys::SLANG_TYPE_KIND_SCALAR as u32,
    ConstantBuffer = sys::SLANG_TYPE_KIND_CONSTANT_BUFFER as u32,
    Resource = sys::SLANG_TYPE_KIND_RESOURCE as u32,
    SamplerState = sys::SLANG_TYPE_KIND_SAMPLER_STATE as u32,
    TextureBuffer = sys::SLANG_TYPE_KIND_TEXTURE_BUFFER as u32,
    ShaderStorageBuffer = sys::SLANG_TYPE_KIND_SHADER_STORAGE_BUFFER as u32,
    ParameterBlock = sys::SLANG_TYPE_KIND_PARAMETER_BLOCK as u32,
    GenericTypeParameter = sys::SLANG_TYPE_KIND_GENERIC_TYPE_PARAMETER as u32,
    Interface = sys::SLANG_TYPE_KIND_INTERFACE as u32,
    OutputStream = sys::SLANG_TYPE_KIND_OUTPUT_STREAM as u32,
    Count = sys::SLANG_TYPE_KIND_COUNT as u32,
}
/*
impl TypeKind {
    fn from_raw(v: u32) -> Option<TypeKind> {
        match v {
            TypeKind::None as u32 => Some(TypeKind::None),
            TypeKind::Struct as u32 => Some(TypeKind::Struct),
            TypeKind::Array as u32 => Some(TypeKind::Array),
            TypeKind::Matrix as u32 => Some(TypeKind::Matrix),
            TypeKind::Vector as u32 => Some(TypeKind::Vector),
            TypeKind::Scalar as u32 => Some(TypeKind::Scalar),
            TypeKind::ConstantBuffer as u32 => Some(TypeKind::ConstantBuffer),
            TypeKind::Resource as u32 => Some(TypeKind::Resource),
            TypeKind::SamplerState as u32 => Some(TypeKind::SamplerState),
            TypeKind::TextureBuffer as u32 => Some(TypeKind::TextureBuffer),
            TypeKind::ShaderStorageBuffer as u32 => Some(TypeKind::ShaderStorageBuffer),
            TypeKind::ParameterBlock as u32 => Some(TypeKind::ParameterBlock),
            TypeKind::GenericTypeParameter as u32 => Some(TypeKind::GenericTypeParameter),
            TypeKind::Interface as u32 => Some(TypeKind::Interface),
            TypeKind::OutputStream as u32 => Some(TypeKind::OutputStream),
            TypeKind::Count as u32 => Some(TypeKind::Count),
            _ => None
        }
    }
}*/

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum ScalarType {
    None = sys::SLANG_SCALAR_TYPE_NONE as u32,
    Void = sys::SLANG_SCALAR_TYPE_VOID as u32,
    Bool = sys::SLANG_SCALAR_TYPE_BOOL as u32,
    Int32 = sys::SLANG_SCALAR_TYPE_INT32 as u32,
    Uint32 = sys::SLANG_SCALAR_TYPE_UINT32 as u32,
    Int64 = sys::SLANG_SCALAR_TYPE_INT64 as u32,
    Uint64 = sys::SLANG_SCALAR_TYPE_UINT64 as u32,
    Float16 = sys::SLANG_SCALAR_TYPE_FLOAT16 as u32,
    Float32 = sys::SLANG_SCALAR_TYPE_FLOAT32 as u32,
    Float64 = sys::SLANG_SCALAR_TYPE_FLOAT64 as u32,
    Int8 = sys::SLANG_SCALAR_TYPE_INT8 as u32,
    Uint8 = sys::SLANG_SCALAR_TYPE_UINT8 as u32,
    Int16 = sys::SLANG_SCALAR_TYPE_INT16 as u32,
    Uint16 = sys::SLANG_SCALAR_TYPE_UINT16 as u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum ResourceShape {
    ResourceBaseShapeMask = sys::SLANG_RESOURCE_BASE_SHAPE_MASK as u32,
    ResourceNone = sys::SLANG_RESOURCE_NONE as u32,
    Texture1d = sys::SLANG_TEXTURE_1D as u32,
    Texture2d = sys::SLANG_TEXTURE_2D as u32,
    Texture3d = sys::SLANG_TEXTURE_3D as u32,
    TextureCube = sys::SLANG_TEXTURE_CUBE as u32,
    TextureBuffer = sys::SLANG_TEXTURE_BUFFER as u32,
    StructuredBuffer = sys::SLANG_STRUCTURED_BUFFER as u32,
    ByteAddressBuffer = sys::SLANG_BYTE_ADDRESS_BUFFER as u32,
    ResourceUnknown = sys::SLANG_RESOURCE_UNKNOWN as u32,
    ResourceExtShapeMask = sys::SLANG_RESOURCE_EXT_SHAPE_MASK as u32,
    TextureArrayFlag = sys::SLANG_TEXTURE_ARRAY_FLAG as u32,
    TextureMultisampleFlag = sys::SLANG_TEXTURE_MULTISAMPLE_FLAG as u32,
    Texture1dArray = sys::SLANG_TEXTURE_1D_ARRAY as u32,
    Texture2dArray = sys::SLANG_TEXTURE_2D_ARRAY as u32,
    TextureCubeArray = sys::SLANG_TEXTURE_CUBE_ARRAY as u32,
    Texture2dMultisample = sys::SLANG_TEXTURE_2D_MULTISAMPLE as u32,
    Texture2dMultisampleArray = sys::SLANG_TEXTURE_2D_MULTISAMPLE_ARRAY as u32,
}
/*
impl ResourceShape {
    fn from_raw(v: u32) -> Option<ResourceShape> {
        match v as i32 {
            sys::SLANG_RESOURCE_BASE_SHAPE_MASK => Some(ResourceShape::ResourceBaseShapeMask),
            sys::SLANG_RESOURCE_NONE => Some(ResourceShape::ResourceNone),
            sys::SLANG_TEXTURE_1D => Some(ResourceShape::Texture1d),
            sys::SLANG_TEXTURE_2D => Some(ResourceShape::Texture2d),
            sys::SLANG_TEXTURE_3D => Some(ResourceShape::Texture3d),
            sys::SLANG_TEXTURE_CUBE => Some(ResourceShape::TextureCube),
            sys::SLANG_TEXTURE_BUFFER => Some(ResourceShape::TextureBuffer),
            sys::SLANG_STRUCTURED_BUFFER => Some(ResourceShape::StructuredBuffer),
            sys::SLANG_BYTE_ADDRESS_BUFFER => Some(ResourceShape::ByteAddressBuffer),
            sys::SLANG_RESOURCE_UNKNOWN => Some(ResourceShape::ResourceUnknown),
            sys::SLANG_RESOURCE_EXT_SHAPE_MASK => Some(ResourceShape::ResourceExtShapeMask),
            sys::SLANG_TEXTURE_ARRAY_FLAG => Some(ResourceShape::TextureArrayFlag),
            sys::SLANG_TEXTURE_MULTISAMPLE_FLAG => Some(ResourceShape::TextureMultisampleFlag),
            sys::SLANG_TEXTURE_1D_ARRAY => Some(ResourceShape::Texture1dArray),
            sys::SLANG_TEXTURE_2D_ARRAY => Some(ResourceShape::Texture2dArray),
            sys::SLANG_TEXTURE_CUBE_ARRAY => Some(ResourceShape::TextureCubeArray),
            sys::SLANG_TEXTURE_2D_MULTISAMPLE => Some(ResourceShape::Texture2dMultisample),
            sys::SLANG_TEXTURE_2D_MULTISAMPLE_ARRAY => Some(ResourceShape::Texture2dMultisampleArray),
            _ => None
        }
    }
}*/

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum ResourceAccess {
    None = sys::SLANG_RESOURCE_ACCESS_NONE as u32,
    Read = sys::SLANG_RESOURCE_ACCESS_READ as u32,
    ReadWrite = sys::SLANG_RESOURCE_ACCESS_READ_WRITE as u32,
    RasterOrdered = sys::SLANG_RESOURCE_ACCESS_RASTER_ORDERED as u32,
    Append = sys::SLANG_RESOURCE_ACCESS_APPEND as u32,
    Consume = sys::SLANG_RESOURCE_ACCESS_CONSUME as u32,
}
/*
impl ResourceAccess {
    fn from_raw(v: u32) -> Option<ResourceAccess> {
        match v as i32 {
            sys::SLANG_RESOURCE_ACCESS_NONE => Some(ResourceAccess::None),
            sys::SLANG_RESOURCE_ACCESS_READ => Some(ResourceAccess::Read),
            sys::SLANG_RESOURCE_ACCESS_READ_WRITE => Some(ResourceAccess::ReadWrite),
            sys::SLANG_RESOURCE_ACCESS_RASTER_ORDERED => Some(ResourceAccess::RasterOrdered),
            sys::SLANG_RESOURCE_ACCESS_APPEND => Some(ResourceAccess::Append),
            sys::SLANG_RESOURCE_ACCESS_CONSUME => Some(ResourceAccess::Consume),
            _ => None
        }
    }
}*/

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum ParameterCategory {
    None = sys::SLANG_PARAMETER_CATEGORY_NONE as u32,
    Mixed = sys::SLANG_PARAMETER_CATEGORY_MIXED as u32,
    ConstantBuffer = sys::SLANG_PARAMETER_CATEGORY_CONSTANT_BUFFER as u32,
    ShaderResource = sys::SLANG_PARAMETER_CATEGORY_SHADER_RESOURCE as u32,
    UnorderedAccess = sys::SLANG_PARAMETER_CATEGORY_UNORDERED_ACCESS as u32,
    VaryingInput = sys::SLANG_PARAMETER_CATEGORY_VARYING_INPUT as u32,
    VaryingOutput = sys::SLANG_PARAMETER_CATEGORY_VARYING_OUTPUT as u32,
    SamplerState = sys::SLANG_PARAMETER_CATEGORY_SAMPLER_STATE as u32,
    Uniform = sys::SLANG_PARAMETER_CATEGORY_UNIFORM as u32,
    DescriptorTableSlot = sys::SLANG_PARAMETER_CATEGORY_DESCRIPTOR_TABLE_SLOT as u32,
    SpecializationConstant = sys::SLANG_PARAMETER_CATEGORY_SPECIALIZATION_CONSTANT as u32,
    PushConstantBuffer = sys::SLANG_PARAMETER_CATEGORY_PUSH_CONSTANT_BUFFER as u32,
    RegisterSpace = sys::SLANG_PARAMETER_CATEGORY_REGISTER_SPACE as u32,
    Generic = sys::SLANG_PARAMETER_CATEGORY_GENERIC as u32,
    RayPayload = sys::SLANG_PARAMETER_CATEGORY_RAY_PAYLOAD as u32,
    HitAttributes = sys::SLANG_PARAMETER_CATEGORY_HIT_ATTRIBUTES as u32,
    CallablePayload = sys::SLANG_PARAMETER_CATEGORY_CALLABLE_PAYLOAD as u32,
    ShaderRecord = sys::SLANG_PARAMETER_CATEGORY_SHADER_RECORD as u32,
    Count = sys::SLANG_PARAMETER_CATEGORY_COUNT as u32,
}
/*
impl ParameterCategory {
    fn from_raw(v: u32) -> Option<ParameterCategory> {
        match v as i32 {
            sys::SLANG_PARAMETER_CATEGORY_NONE => Some(ParameterCategory::None),
            sys::SLANG_PARAMETER_CATEGORY_MIXED => Some(ParameterCategory::Mixed),
            sys::SLANG_PARAMETER_CATEGORY_CONSTANT_BUFFER => Some(ParameterCategory::ConstantBuffer),
            sys::SLANG_PARAMETER_CATEGORY_SHADER_RESOURCE => Some(ParameterCategory::ShaderResource),
            sys::SLANG_PARAMETER_CATEGORY_UNORDERED_ACCESS => Some(ParameterCategory::UnorderedAccess),
            sys::SLANG_PARAMETER_CATEGORY_VARYING_INPUT => Some(ParameterCategory::VaryingInput),
            sys::SLANG_PARAMETER_CATEGORY_VARYING_OUTPUT => Some(ParameterCategory::VaryingOutput),
            sys::SLANG_PARAMETER_CATEGORY_SAMPLER_STATE => Some(ParameterCategory::SamplerState),
            sys::SLANG_PARAMETER_CATEGORY_UNIFORM => Some(ParameterCategory::Uniform),
            sys::SLANG_PARAMETER_CATEGORY_DESCRIPTOR_TABLE_SLOT => Some(ParameterCategory::DescriptorTableSlot),
            sys::SLANG_PARAMETER_CATEGORY_SPECIALIZATION_CONSTANT => Some(ParameterCategory::SpecializationConstant),
            sys::SLANG_PARAMETER_CATEGORY_PUSH_CONSTANT_BUFFER => Some(ParameterCategory::PushConstantBuffer),
            sys::SLANG_PARAMETER_CATEGORY_REGISTER_SPACE => Some(ParameterCategory::RegisterSpace),
            sys::SLANG_PARAMETER_CATEGORY_GENERIC => Some(ParameterCategory::Generic),
            sys::SLANG_PARAMETER_CATEGORY_RAY_PAYLOAD => Some(ParameterCategory::RayPayload),
            sys::SLANG_PARAMETER_CATEGORY_HIT_ATTRIBUTES => Some(ParameterCategory::HitAttributes),
            sys::SLANG_PARAMETER_CATEGORY_CALLABLE_PAYLOAD => Some(ParameterCategory::CallablePayload),
            sys::SLANG_PARAMETER_CATEGORY_SHADER_RECORD => Some(ParameterCategory::ShaderRecord),
            sys::SLANG_PARAMETER_CATEGORY_COUNT => Some(ParameterCategory::Count),
            _ => None
        }
    }
}
*/
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum MatrixLayoutMode {
    Unknown = sys::SLANG_MATRIX_LAYOUT_MODE_UNKNOWN as u32,
    RowMajor = sys::SLANG_MATRIX_LAYOUT_ROW_MAJOR as u32,
    ColumnMajor = sys::SLANG_MATRIX_LAYOUT_COLUMN_MAJOR as u32,
}
/*
impl MatrixLayoutMode {
    fn from_raw(v: u32) -> Option<MatrixLayoutMode> {
        match v as i32 {
            sys::SLANG_MATRIX_LAYOUT_MODE_UNKNOWN => Some(MatrixLayoutMode::Unknown),
            sys::SLANG_MATRIX_LAYOUT_ROW_MAJOR => Some(MatrixLayoutMode::RowMajor),
            sys::SLANG_MATRIX_LAYOUT_COLUMN_MAJOR => Some(MatrixLayoutMode::ColumnMajor),
            _ => None
        }

    }
}*/

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum LayoutRules {
    Default = sys::SLANG_LAYOUT_RULES_DEFAULT as u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Modifier {
    Shared = sys::SLANG_MODIFIER_SHARED as u32,
}

//--------------------------------------------------------------------------------------------------

pub struct Session(*mut sys::SlangSession);

impl Session {
    pub fn new() -> Session {
        unsafe { Session(sys::spCreateSession(ptr::null())) }
    }

    pub fn create_compile_request(&self) -> CompileRequest {
        unsafe { CompileRequest(sys::spCreateCompileRequest(self.0)) }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { sys::spDestroySession(self.0) }
    }
}

pub struct CompileRequest(*mut sys::SlangCompileRequest);

impl CompileRequest {
    /// Sets the target for code generation.
    ///
    /// Possible values are:
    ///  - `SlangCompileTarget::Glsl`. Generates GLSL code.
    ///  - `SlangCompileTarget::Hlsl`. Generates HLSL code.
    ///  - `SlangCompileTarget::SpirV`. Generates SPIR-V code.
    pub fn set_codegen_target(&mut self, target: CompileTarget) -> &mut Self {
        unsafe { sys::spSetCodeGenTarget(self.0, target as sys::SlangCompileTarget) }
        self
    }

    /// (slang) Adds a path to use when searching for referenced files.
    ///
    /// This will be used for both `#include` directives and also for explicit `__import` declarations.
    pub fn add_include_search_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        let cstr = CString::new(path.as_ref().to_str().expect("invalid UTF-8")).unwrap();

        unsafe {
            sys::spAddSearchPath(self.0, cstr.as_ptr());
        }

        self
    }

    /// (slang) Adds a macro definition to be used during preprocessing.
    ///
    /// This will be used for both `#include` directives and also for explicit `__import` declarations.
    pub fn add_preprocessor_define(&mut self, key: &str, value: &str) -> &mut Self {
        let key = CString::new(key).unwrap();
        let val = CString::new(value).unwrap();

        unsafe {
            sys::spAddPreprocessorDefine(self.0, key.as_ptr(), val.as_ptr());
        }

        self
    }

    /// (slang) Adds a distinct translation unit to the compilation request.
    pub fn add_translation_unit(
        &mut self,
        source_language: SourceLanguage,
        name: Option<&str>,
    ) -> TranslationUnit {
        let name = CString::new(name.unwrap_or("")).unwrap();

        let index = unsafe {
            sys::spAddTranslationUnit(
                self.0,
                source_language as sys::SlangSourceLanguage,
                name.as_ptr(),
            )
        };

        TranslationUnit {
            request: self.0,
            index,
            _phantom: PhantomData,
        }
    }

    /// (slang) Executes the compilation request.
    pub fn compile(self) -> Result<CompiledRequest, CompilationErrors> {
        let r = unsafe { sys::spCompile(self.0) };

        if result_failed(r) {
            let diag = unsafe { sys::spGetDiagnosticOutput(self.0) };

            let diag = unsafe {
                CStr::from_ptr(diag)
                    .to_str()
                    .expect("invalid UTF-8 in diagnostic")
                    .to_string()
            };

            Err(CompilationErrors { errors: diag })
        } else {
            Ok(CompiledRequest { request: self })
        }
    }
}

impl Drop for CompileRequest {
    fn drop(&mut self) {
        unsafe { sys::spDestroyCompileRequest(self.0) }
    }
}

pub struct TranslationUnit<'a> {
    request: *mut sys::SlangCompileRequest,
    index: i32,
    _phantom: PhantomData<&'a CompileRequest>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntryPointIndex(pub i32);

impl<'a> TranslationUnit<'a> {
    /// (slang) Adds a source file to the translation unit.
    pub fn add_source_file<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        let path = CString::new(path.as_ref().to_str().expect("invalid UTF-8")).unwrap();
        unsafe {
            sys::spAddTranslationUnitSourceFile(self.request, self.index, path.as_ptr());
        }
        self
    }

    /// (slang) Adds a source string to the given translation unit.
    ///
    /// The `path` will be used in any diagnostic output, as well
    /// as to determine the base path when resolving relative
    /// `#include`s.
    pub fn add_source_string<P: AsRef<Path>>(&mut self, path: P, source: &str) -> &mut Self {
        let path = CString::new(path.as_ref().to_str().expect("invalid UTF-8")).unwrap();
        let source = CString::new(source).unwrap();
        unsafe {
            sys::spAddTranslationUnitSourceString(
                self.request,
                self.index,
                path.as_ptr(),
                source.as_ptr(),
            );
        }
        self
    }

    pub fn add_entry_point(&mut self, name: &str, stage: Stage) -> EntryPointIndex {
        let name = CString::new(name).unwrap();
        let index = unsafe {
            sys::spAddEntryPoint(
                self.request,
                self.index,
                name.as_ptr(),
                stage as sys::SlangStage,
            )
        };
        EntryPointIndex(index)
    }
}

#[derive(Debug)]
pub struct CompilationErrors {
    errors: String,
}

pub struct CompiledRequest {
    request: CompileRequest,
}

impl CompiledRequest {
    pub fn get_entry_point_code(&self, index: EntryPointIndex) -> &[u8] {
        unsafe {
            let mut data_size = 0;
            let ptr = sys::spGetEntryPointCode(self.request.0, index.0, &mut data_size);
            slice::from_raw_parts(ptr as *const u8, data_size)
        }
    }

    pub fn reflection(&self) -> &Reflection {
        unsafe {
            let r = sys::spGetReflection(self.request.0);
            &*(r as *const Reflection)
        }
    }
}

//--------------------------------------------------------------------------------------------------

// AFAIK, the reflection API does not make any guarantee of being thread-safe

pub struct Reflection(sys::SlangReflection, PhantomData<*mut sys::SlangReflection>);
pub struct ReflectionVariableLayout(
    sys::SlangReflectionVariableLayout,
    PhantomData<*mut sys::SlangReflection>,
);
pub struct ReflectionVariable(
    sys::SlangReflectionVariable,
    PhantomData<*mut sys::SlangReflection>,
);
pub struct ReflectionTypeLayout(
    sys::SlangReflectionTypeLayout,
    PhantomData<*mut sys::SlangReflection>,
);
pub struct ReflectionType(
    sys::SlangReflectionType,
    PhantomData<*mut sys::SlangReflection>,
);
pub struct ReflectionEntryPoint(
    sys::SlangReflectionEntryPoint,
    PhantomData<*mut sys::SlangReflection>,
);
pub struct ReflectionTypeParameter(
    sys::SlangReflectionTypeParameter,
    PhantomData<*mut sys::SlangReflection>,
);
pub struct ReflectionUserAttribute(
    sys::SlangReflectionUserAttribute,
    PhantomData<*mut sys::SlangReflection>,
);

macro_rules! call {
    ($s:ident, $f:ident, $($arg:tt)*) => {
         sys::$f($s as *const _ as *mut _, $($arg)*)
    };
    ($s:ident, $f:ident as str, $($arg:tt)*) => {
        {
            let ptr = sys::$f($s as *const _ as *mut _, $($arg)*);
            CStr::from_ptr(ptr).to_str().expect("invalid UTF-8")
        }
    };
    ($s:ident, $f:ident as option str, $($arg:tt)*) => {
        {
            let ptr = sys::$f($s as *const _ as *mut _, $($arg)*);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().expect("invalid UTF-8"))
            }
        }
    };
    ($s:ident, $f:ident as $cast:ty, $($arg:tt)*) => {
         &*(sys::$f($s as *const _ as *mut _, $($arg)*) as *const $cast)
    };
    ($s:ident, $f:ident as option $cast:ty, $($arg:tt)*) => {
        {
            let ptr = sys::$f($s as *const _ as *mut _, $($arg)*);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *const $cast))
            }
        }
    };
}

impl Reflection {
    pub fn parameter_count(&self) -> u32 {
        unsafe { call!(self, spReflection_GetParameterCount,) }
    }

    pub fn parameter_by_index(&self, index: u32) -> &ReflectionVariableLayout {
        assert!(index < self.parameter_count());
        unsafe {
            call!(
                self,
                spReflection_GetParameterByIndex as ReflectionVariableLayout,
                index,
            )
        }
    }

    pub fn parameters(&self) -> impl Iterator<Item = &ReflectionVariableLayout> {
        let n = self.parameter_count();
        (0..n).map(move |i| unsafe {
            call!(
                self,
                spReflection_GetParameterByIndex as ReflectionVariableLayout,
                i,
            )
        })
    }

    pub fn type_parameter_count(&self) -> u32 {
        unsafe { call!(self, spReflection_GetTypeParameterCount,) }
    }

    pub fn type_parameter_by_index(&self, index: u32) -> &ReflectionTypeParameter {
        assert!(index < self.type_parameter_count());
        unsafe {
            call!(
                self,
                spReflection_GetTypeParameterByIndex as ReflectionTypeParameter,
                index,
            )
        }
    }

    pub fn type_parameters(&self) -> impl Iterator<Item = &ReflectionTypeParameter> {
        let n = self.type_parameter_count();
        (0..n).map(move |i| unsafe {
            call!(
                self,
                spReflection_GetTypeParameterByIndex as ReflectionTypeParameter,
                i,
            )
        })
    }

    pub fn find_type_parameter_by_name(&self, name: &str) -> Option<&ReflectionTypeParameter> {
        let name = CString::new(name).unwrap();
        unsafe {
            call!(self, spReflection_FindTypeParameter as option ReflectionTypeParameter, name.as_ptr())
        }
    }

    pub fn find_type_by_name(&self, name: &str) -> Option<&ReflectionType> {
        let name = CString::new(name).unwrap();
        unsafe { call!(self, spReflection_FindTypeByName as option ReflectionType, name.as_ptr()) }
    }

    pub fn type_layout(&self, ty: &ReflectionType, rules: LayoutRules) -> &ReflectionTypeLayout {
        unsafe {
            call!(
                self,
                spReflection_GetTypeLayout as ReflectionTypeLayout,
                &ty.0 as *const _ as *mut sys::SlangReflectionType,
                rules as sys::SlangLayoutRules
            )
        }
    }

    pub fn entry_point_count(&self) -> u32 {
        unsafe { call!(self, spReflection_getEntryPointCount,) as u32 } // why uintptr ?
    }

    pub fn entry_point_by_index(&self, index: u32) -> &ReflectionEntryPoint {
        assert!(index < self.entry_point_count());
        unsafe {
            call!(
                self,
                spReflection_getEntryPointByIndex as ReflectionEntryPoint,
                index as usize, // why uintptr?
            )
        }
    }

    pub fn entry_points(&self) -> impl Iterator<Item = &ReflectionEntryPoint> {
        let n = self.entry_point_count();
        (0..n).map(move |i| unsafe {
            call!(
                self,
                spReflection_getEntryPointByIndex as ReflectionEntryPoint,
                i as usize, // why uintptr?
            )
        })
    }

    pub fn find_entry_point_by_name(&self, name: &str) -> Option<&ReflectionEntryPoint> {
        let name = CString::new(name).unwrap();
        unsafe {
            call!(self, spReflection_findEntryPointByName as option ReflectionEntryPoint, name.as_ptr())
        }
    }

    pub fn global_constant_buffer_binding(&self) -> u32 {
        unsafe { call!(self, spReflection_getGlobalConstantBufferBinding,) as u32 } // why usize?
    }

    pub fn global_constant_buffer_size(&self) -> usize {
        unsafe { call!(self, spReflection_getGlobalConstantBufferSize,) }
    }
}

impl ReflectionVariableLayout {
    pub fn variable(&self) -> &ReflectionVariable {
        unsafe {
            call!(
                self,
                spReflectionVariableLayout_GetVariable as ReflectionVariable,
            )
        }
    }

    pub fn type_layout(&self) -> &ReflectionTypeLayout {
        unsafe {
            call!(
                self,
                spReflectionVariableLayout_GetTypeLayout as ReflectionTypeLayout,
            )
        }
    }

    pub fn offset(&self, param_category: ParameterCategory) -> usize {
        unsafe {
            call!(
                self,
                spReflectionVariableLayout_GetOffset,
                param_category as sys::SlangParameterCategory,
            )
        }
    }

    pub fn space(&self, param_category: ParameterCategory) -> usize {
        unsafe {
            call!(
                self,
                spReflectionVariableLayout_GetSpace,
                param_category as sys::SlangParameterCategory,
            )
        }
    }

    pub fn semantic_name(&self) -> Option<&str> {
        unsafe { call!(self, spReflectionVariableLayout_GetSemanticName as option str,) }
    }

    pub fn semantic_index(&self) -> u32 {
        unsafe { call!(self, spReflectionVariableLayout_GetSemanticIndex,) as u32 } // why usize?
    }

    pub fn stage(&self) -> Stage {
        unsafe { Stage::from_u32(call!(self, spReflectionVariableLayout_getStage,)).unwrap() }
    }

    pub fn binding_index(&self) -> u32 {
        unsafe { call!(self, spReflectionParameter_GetBindingIndex,) }
    }

    pub fn binding_space(&self) -> u32 {
        unsafe { call!(self, spReflectionParameter_GetBindingSpace,) }
    }
}

impl ReflectionVariable {
    pub fn name(&self) -> &str {
        unsafe { call!(self, spReflectionVariable_GetName as str,) }
    }

    pub fn ty(&self) -> &ReflectionType {
        unsafe { call!(self, spReflectionVariable_GetType as ReflectionType,) }
    }

    pub fn user_attribute_count(&self) -> u32 {
        unsafe { call!(self, spReflectionVariable_GetUserAttributeCount,) }
    }

    pub fn user_attribute_by_index(&self, index: u32) -> Option<&ReflectionUserAttribute> {
        unsafe {
            call!(
                self,
                spReflectionVariable_GetUserAttribute as option ReflectionUserAttribute,
                index
            )
        }
    }

    pub fn find_user_attribute_by_name(&self, name: &str) -> Option<&ReflectionUserAttribute> {
        unimplemented!()
        /*unsafe {
            let cstr = CString::new(name).unwrap();
            call_ret_cast_opt!(self, spReflectionVariable_FindUserAttributeByName as ReflectionUserAttribute, ptr::null(), cstr.as_ptr())
        }*/
    }
}

impl ReflectionTypeLayout {
    pub fn ty(&self) -> &ReflectionType {
        unsafe { call!(self, spReflectionTypeLayout_GetType as ReflectionType,) }
    }

    pub fn size(&self, category: ParameterCategory) -> usize {
        unsafe {
            call!(
                self,
                spReflectionTypeLayout_GetSize,
                category as sys::SlangParameterCategory
            )
        }
    }

    pub fn field_count(&self) -> u32 {
        self.ty().field_count()
    }

    pub fn field_by_index(&self, index: u32) -> &ReflectionVariableLayout {
        assert!(index < self.field_count());
        unsafe {
            call!(
                self,
                spReflectionTypeLayout_GetFieldByIndex as ReflectionVariableLayout,
                index
            )
        }
    }

    pub fn fields(&self) -> impl Iterator<Item = &ReflectionVariableLayout> {
        let n = self.field_count();
        (0..n).map(move |i| unsafe {
            call!(
                self,
                spReflectionTypeLayout_GetFieldByIndex as ReflectionVariableLayout,
                i
            )
        })
    }

    pub fn element_stride(&self, category: ParameterCategory) -> usize {
        unsafe {
            call!(
                self,
                spReflectionTypeLayout_GetElementStride,
                category as sys::SlangParameterCategory
            )
        }
    }

    pub fn element_type_layout(&self) -> Option<&ReflectionTypeLayout> {
        unsafe {
            call!(self, spReflectionTypeLayout_GetElementTypeLayout as option ReflectionTypeLayout, )
        }
    }

    pub fn element_var_layout(&self) -> Option<&ReflectionVariableLayout> {
        unsafe {
            call!(self, spReflectionTypeLayout_GetElementVarLayout as option ReflectionVariableLayout, )
        }
    }

    pub fn parameter_category(&self) -> ParameterCategory {
        unsafe {
            ParameterCategory::from_u32(call!(self, spReflectionTypeLayout_GetParameterCategory,))
                .unwrap()
        }
    }

    pub fn parameter_category_count(&self) -> u32 {
        unsafe { call!(self, spReflectionTypeLayout_GetCategoryCount,) }
    }

    pub fn parameter_category_by_index(&self, index: u32) -> ParameterCategory {
        assert!(index < self.parameter_category_count());
        unsafe {
            ParameterCategory::from_u32(call!(
                self,
                spReflectionTypeLayout_GetCategoryByIndex,
                index
            ))
            .unwrap()
        }
    }

    pub fn matrix_layout_mode(&self) -> MatrixLayoutMode {
        unsafe {
            MatrixLayoutMode::from_u32(call!(self, spReflectionTypeLayout_GetMatrixLayoutMode,))
                .unwrap()
        }
    }

    pub fn generic_param_index(&self) -> Option<u32> {
        unsafe {
            let i = call!(self, spReflectionTypeLayout_getGenericParamIndex,);
            if i < 0 {
                None
            } else {
                Some(i as u32)
            }
        }
    }
}

impl ReflectionType {
    pub fn name(&self) -> Option<&str> {
        unsafe { call!(self, spReflectionType_GetName as option str,) }
    }

    pub fn kind(&self) -> TypeKind {
        unsafe { TypeKind::from_u32(call!(self, spReflectionType_GetKind,)).unwrap() }
    }

    pub fn user_attribute_count(&self) -> u32 {
        unsafe { call!(self, spReflectionType_GetUserAttributeCount,) }
    }

    pub fn user_attribute_by_index(&self, index: u32) -> &ReflectionUserAttribute {
        assert!(index < self.user_attribute_count());
        unsafe {
            call!(
                self,
                spReflectionType_GetUserAttribute as ReflectionUserAttribute,
                index
            )
        }
    }

    pub fn find_user_attribute_by_name(&self, name: &str) -> Option<&ReflectionUserAttribute> {
        let name = CString::new(name).unwrap();
        unsafe {
            call!(self, spReflectionType_FindUserAttributeByName as option ReflectionUserAttribute, name.as_ptr())
        }
    }

    pub fn field_count(&self) -> u32 {
        unsafe { call!(self, spReflectionType_GetFieldCount,) }
    }

    pub fn field_by_index(&self, index: u32) -> &ReflectionVariable {
        assert!(index < self.field_count());
        unsafe {
            call!(
                self,
                spReflectionType_GetFieldByIndex as ReflectionVariable,
                index
            )
        }
    }

    pub fn fields(&self) -> impl Iterator<Item = &ReflectionVariable> {
        let n = self.field_count();
        (0..n).map(move |i| unsafe {
            call!(
                self,
                spReflectionType_GetFieldByIndex as ReflectionVariable,
                i
            )
        })
    }

    pub fn element_count(&self) -> usize {
        unsafe { call!(self, spReflectionType_GetElementCount,) }
    }
    pub fn element_type(&self) -> Option<&ReflectionType> {
        unsafe { call!(self, spReflectionType_GetElementType as option ReflectionType,) }
    }
    pub fn row_count(&self) -> u32 {
        unsafe { call!(self, spReflectionType_GetRowCount,) }
    }
    pub fn column_count(&self) -> u32 {
        unsafe { call!(self, spReflectionType_GetColumnCount,) }
    }
    pub fn scalar_type(&self) -> ScalarType {
        unsafe { ScalarType::from_u32(call!(self, spReflectionType_GetKind,)).unwrap() }
    }
    pub fn resource_shape(&self) -> ResourceShape {
        unsafe { ResourceShape::from_u32(call!(self, spReflectionType_GetResourceShape,)).unwrap() }
    }
    pub fn resource_access(&self) -> ResourceAccess {
        unsafe {
            ResourceAccess::from_u32(call!(self, spReflectionType_GetResourceAccess,)).unwrap()
        }
    }
    pub fn resource_result_type(&self) -> Option<&ReflectionType> {
        unsafe { call!(self, spReflectionType_GetResourceResultType as option ReflectionType,) }
    }
}

impl ReflectionEntryPoint {
    pub fn name(&self) -> &str {
        unsafe { call!(self, spReflectionEntryPoint_getName as str,) }
    }

    pub fn parameter_count(&self) -> u32 {
        unsafe { call!(self, spReflectionEntryPoint_getParameterCount,) }
    }

    pub fn parameter_by_index(&self, index: u32) -> &ReflectionVariableLayout {
        assert!(index < self.parameter_count());
        unsafe {
            call!(
                self,
                spReflectionEntryPoint_getParameterByIndex as ReflectionVariableLayout,
                index
            )
        }
    }

    pub fn parameters(&self) -> impl Iterator<Item = &ReflectionVariableLayout> {
        let n = self.parameter_count();
        (0..n).map(move |i| unsafe {
            call!(
                self,
                spReflectionEntryPoint_getParameterByIndex as ReflectionVariableLayout,
                i
            )
        })
    }

    pub fn stage(&self) -> Stage {
        unsafe { Stage::from_u32(call!(self, spReflectionEntryPoint_getParameterCount,)).unwrap() }
    }

    pub fn compute_thread_group_size(&self) -> (usize, usize, usize) {
        unsafe {
            let mut axes = [1usize; 3];
            call!(
                self,
                spReflectionEntryPoint_getComputeThreadGroupSize,
                3,
                axes.as_mut_ptr()
            );
            (axes[0], axes[1], axes[2])
        }
    }
}

impl ReflectionTypeParameter {}

impl ReflectionUserAttribute {
    pub fn name(&self) -> &str {
        unimplemented!()
    }
    pub fn argument_count(&self) -> u32 {
        unimplemented!()
    }
    pub fn argument_type_by_index(&self, index: u32) -> Option<&ReflectionType> {
        unimplemented!()
    }
    pub fn argument_value_int_by_index(&self, index: u32) -> Option<i32> {
        unimplemented!()
    }
    pub fn argument_value_float_by_index(&self, index: u32) -> Option<f32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles() {}
}
