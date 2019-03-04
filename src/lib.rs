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

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Stage {
    None = sys::SLANG_STAGE_NONE,
    Vertex = sys::SLANG_STAGE_VERTEX,
    Hull = sys::SLANG_STAGE_HULL,
    Domain = sys::SLANG_STAGE_DOMAIN,
    Geometry = sys::SLANG_STAGE_GEOMETRY,
    Fragment = sys::SLANG_STAGE_FRAGMENT,
    Compute = sys::SLANG_STAGE_COMPUTE,
    RayGeneration = sys::SLANG_STAGE_RAY_GENERATION,
    Intersection = sys::SLANG_STAGE_INTERSECTION,
    AnyHit = sys::SLANG_STAGE_ANY_HIT,
    ClosestHit = sys::SLANG_STAGE_CLOSEST_HIT,
    Miss = sys::SLANG_STAGE_MISS,
    Callable = sys::SLANG_STAGE_CALLABLE,
    //Pixel  = sys::SLANG_STAGE_FRAGMENT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SourceLanguage {
    Unknown = sys::SLANG_SOURCE_LANGUAGE_UNKNOWN,
    Slang = sys::SLANG_SOURCE_LANGUAGE_SLANG,
    Hlsl = sys::SLANG_SOURCE_LANGUAGE_HLSL,
    Glsl = sys::SLANG_SOURCE_LANGUAGE_GLSL,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TypeKind {
    None = sys::SLANG_TYPE_KIND_NONE,
    Struct = sys::SLANG_TYPE_KIND_STRUCT,
    Array = sys::SLANG_TYPE_KIND_ARRAY,
    Matrix = sys::SLANG_TYPE_KIND_MATRIX,
    Vector = sys::SLANG_TYPE_KIND_VECTOR,
    Scalar = sys::SLANG_TYPE_KIND_SCALAR,
    ConstantBuffer = sys::SLANG_TYPE_KIND_CONSTANT_BUFFER,
    Resource = sys::SLANG_TYPE_KIND_RESOURCE,
    SamplerState = sys::SLANG_TYPE_KIND_SAMPLER_STATE,
    TextureBuffer = sys::SLANG_TYPE_KIND_TEXTURE_BUFFER,
    ShaderStorageBuffer = sys::SLANG_TYPE_KIND_SHADER_STORAGE_BUFFER,
    ParameterBlock = sys::SLANG_TYPE_KIND_PARAMETER_BLOCK,
    GenericTypeParameter = sys::SLANG_TYPE_KIND_GENERIC_TYPE_PARAMETER,
    Interface = sys::SLANG_TYPE_KIND_INTERFACE,
    OutputStream = sys::SLANG_TYPE_KIND_OUTPUT_STREAM,
    Count = sys::SLANG_TYPE_KIND_COUNT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ScalarType {
    None = sys::SLANG_SCALAR_TYPE_NONE,
    Void = sys::SLANG_SCALAR_TYPE_VOID,
    Bool = sys::SLANG_SCALAR_TYPE_BOOL,
    Int32 = sys::SLANG_SCALAR_TYPE_INT32,
    Uint32 = sys::SLANG_SCALAR_TYPE_UINT32,
    Int64 = sys::SLANG_SCALAR_TYPE_INT64,
    Uint64 = sys::SLANG_SCALAR_TYPE_UINT64,
    Float16 = sys::SLANG_SCALAR_TYPE_FLOAT16,
    Float32 = sys::SLANG_SCALAR_TYPE_FLOAT32,
    Float64 = sys::SLANG_SCALAR_TYPE_FLOAT64,
    Int8 = sys::SLANG_SCALAR_TYPE_INT8,
    Uint8 = sys::SLANG_SCALAR_TYPE_UINT8,
    Int16 = sys::SLANG_SCALAR_TYPE_INT16,
    Uint16 = sys::SLANG_SCALAR_TYPE_UINT16,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResourceShape {
    ResourceBaseShapeMask = sys::SLANG_RESOURCE_BASE_SHAPE_MASK,
    ResourceNone = sys::SLANG_RESOURCE_NONE,
    Texture1d = sys::SLANG_TEXTURE_1D,
    Texture2d = sys::SLANG_TEXTURE_2D,
    Texture3d = sys::SLANG_TEXTURE_3D,
    TextureCube = sys::SLANG_TEXTURE_CUBE,
    TextureBuffer = sys::SLANG_TEXTURE_BUFFER,
    StructuredBuffer = sys::SLANG_STRUCTURED_BUFFER,
    ByteAddressBuffer = sys::SLANG_BYTE_ADDRESS_BUFFER,
    ResourceUnknown = sys::SLANG_RESOURCE_UNKNOWN,
    ResourceExtShapeMask = sys::SLANG_RESOURCE_EXT_SHAPE_MASK,
    TextureArrayFlag = sys::SLANG_TEXTURE_ARRAY_FLAG,
    TextureMultisampleFlag = sys::SLANG_TEXTURE_MULTISAMPLE_FLAG,
    Texture1dArray = sys::SLANG_TEXTURE_1D_ARRAY,
    Texture2dArray = sys::SLANG_TEXTURE_2D_ARRAY,
    TextureCubeArray = sys::SLANG_TEXTURE_CUBE_ARRAY,
    Texture2dMultisample = sys::SLANG_TEXTURE_2D_MULTISAMPLE,
    Texture2dMultisampleArray = sys::SLANG_TEXTURE_2D_MULTISAMPLE_ARRAY,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResourceAccess {
    None = sys::SLANG_RESOURCE_ACCESS_NONE,
    Read = sys::SLANG_RESOURCE_ACCESS_READ,
    ReadWrite = sys::SLANG_RESOURCE_ACCESS_READ_WRITE,
    RasterOrdered = sys::SLANG_RESOURCE_ACCESS_RASTER_ORDERED,
    Append = sys::SLANG_RESOURCE_ACCESS_APPEND,
    Consume = sys::SLANG_RESOURCE_ACCESS_CONSUME,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParameterCategory {
    None = sys::SLANG_PARAMETER_CATEGORY_NONE,
    Mixed = sys::SLANG_PARAMETER_CATEGORY_MIXED,
    ConstantBuffer = sys::SLANG_PARAMETER_CATEGORY_CONSTANT_BUFFER,
    ShaderResource = sys::SLANG_PARAMETER_CATEGORY_SHADER_RESOURCE,
    UnorderedAccess = sys::SLANG_PARAMETER_CATEGORY_UNORDERED_ACCESS,
    VaryingInput = sys::SLANG_PARAMETER_CATEGORY_VARYING_INPUT,
    VaryingOutput = sys::SLANG_PARAMETER_CATEGORY_VARYING_OUTPUT,
    SamplerState = sys::SLANG_PARAMETER_CATEGORY_SAMPLER_STATE,
    Uniform = sys::SLANG_PARAMETER_CATEGORY_UNIFORM,
    DescriptorTableSlot = sys::SLANG_PARAMETER_CATEGORY_DESCRIPTOR_TABLE_SLOT,
    SpecializationConstant = sys::SLANG_PARAMETER_CATEGORY_SPECIALIZATION_CONSTANT,
    PushConstantBuffer = sys::SLANG_PARAMETER_CATEGORY_PUSH_CONSTANT_BUFFER,
    RegisterSpace = sys::SLANG_PARAMETER_CATEGORY_REGISTER_SPACE,
    Generic = sys::SLANG_PARAMETER_CATEGORY_GENERIC,
    RayPayload = sys::SLANG_PARAMETER_CATEGORY_RAY_PAYLOAD,
    HitAttributes = sys::SLANG_PARAMETER_CATEGORY_HIT_ATTRIBUTES,
    CallablePayload = sys::SLANG_PARAMETER_CATEGORY_CALLABLE_PAYLOAD,
    ShaderRecord = sys::SLANG_PARAMETER_CATEGORY_SHADER_RECORD,
    Count = sys::SLANG_PARAMETER_CATEGORY_COUNT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MatrixLayoutMode {
    Unknown = sys::SLANG_MATRIX_LAYOUT_MODE_UNKNOWN,
    RowMajor = sys::SLANG_MATRIX_LAYOUT_ROW_MAJOR,
    ColumnMajor = sys::SLANG_MATRIX_LAYOUT_COLUMN_MAJOR,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum LayoutRules {
    Default = sys::SLANG_LAYOUT_RULES_DEFAULT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Modifier {
    Shared = sys::SLANG_MODIFIER_SHARED,
}

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
    pub fn codegen_target(&self, target: CompileTarget) -> &Self {
        unsafe { sys::spSetCodeGenTarget(self.0, target as sys::SlangCompileTarget) }
        self
    }

    /// (slang) Adds a path to use when searching for referenced files.
    ///
    /// This will be used for both `#include` directives and also for explicit `__import` declarations.
    pub fn add_include_search_path<P: AsRef<Path>>(&self, path: P) -> &Self {
        let cstr = CString::new(path.as_ref().to_str().expect("invalid UTF-8")).unwrap();

        unsafe {
            sys::spAddSearchPath(self.0, cstr.as_ptr());
        }

        self
    }

    /// (slang) Adds a macro definition to be used during preprocessing.
    ///
    /// This will be used for both `#include` directives and also for explicit `__import` declarations.
    pub fn add_preprocessor_define(&self, key: &str, value: &str) -> &Self {
        let key = CString::new(key).unwrap();
        let val = CString::new(value).unwrap();

        unsafe {
            sys::spAddPreprocessorDefine(self.0, key.as_ptr(), val.as_ptr());
        }

        self
    }

    /// (slang) Adds a distinct translation unit to the compilation request.
    pub fn add_translation_unit(
        &self,
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
    pub fn add_source_file<P: AsRef<Path>>(&self, path: P) {
        let path = CString::new(path.as_ref().to_str().expect("invalid UTF-8")).unwrap();
        unsafe {
            sys::spAddTranslationUnitSourceFile(self.request, self.index, path.as_ptr());
        }
    }

    /// (slang) Adds a source string to the given translation unit.
    ///
    /// The `path` will be used in any diagnostic output, as well
    /// as to determine the base path when resolving relative
    /// `#include`s.
    pub fn add_source_string<P: AsRef<Path>>(&self, path: P, source: &str) {
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
    }

    pub fn add_entry_point(&self, name: &str, stage: Stage) -> EntryPointIndex {
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
pub struct Reflection(sys::SlangReflection);
pub struct ReflectionVariableLayout(sys::SlangReflectionVariableLayout);
pub struct ReflectionVariable(sys::SlangReflectionVariable);
pub struct ReflectionTypeLayout(sys::SlangReflectionTypeLayout);
pub struct ReflectionType(sys::SlangReflectionType);
pub struct ReflectionEntryPoint(sys::SlangReflectionEntryPoint);
pub struct ReflectionTypeParameter(sys::SlangReflectionTypeParameter);
pub struct ReflectionUserAttribute(sys::SlangReflectionUserAttribute);

macro_rules! call_return_str {
    ($s:ident, $f:ident, $($arg:tt)*) => {
        {
            let ptr = sys::$f($s as *const _ as *mut _, $($arg)*);
            CStr::from_ptr(ptr).to_str().expect("invalid UTF-8")
        }
    };
}

macro_rules! call {
    ($s:ident, $f:ident, $($arg:tt)*) => {
         sys::$f($s as *const _ as *mut _, $($arg)*)
    };
}

macro_rules! call_ret_cast {
    ($s:ident, $f:ident as $cast:ty, $($arg:tt)*) => {
         &*(sys::$f($s as *const _ as *mut _, $($arg)*) as *const $cast)
    };
}
macro_rules! call_ret_cast_opt {
    ($s:ident, $f:ident as $cast:ty, $($arg:tt)*) => {
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

impl ReflectionVariableLayout {
    pub fn variable(&self) -> &ReflectionVariable {
        unimplemented!()
    }
    pub fn type_layout(&self) -> &ReflectionTypeLayout {
        unimplemented!()
    }
    pub fn offset(&self, param_category: ParameterCategory) -> usize {
        unimplemented!()
    }
    pub fn space(&self, param_category: ParameterCategory) -> usize {
        unimplemented!()
    }
    pub fn semantic_name(&self) -> Option<&str> {
        unimplemented!()
    }
    pub fn semantic_index(&self) -> Option<usize> {
        unimplemented!()
    }
    pub fn stage(&self) -> Option<Stage> {
        unimplemented!()
    }
    pub fn binding_index(&self) -> Option<u32> {
        unimplemented!()
    }
    pub fn binding_space(&self) -> Option<u32> {
        unimplemented!()
    }
}

impl ReflectionVariable {
    pub fn name(&self) -> &str {
        unsafe { call_return_str!(self, spReflectionVariable_GetName,) }
    }
    pub fn ty(&self) -> &ReflectionType {
        unsafe { call_ret_cast!(self, spReflectionVariable_GetType as ReflectionType,) }
    }
    pub fn user_attribute_count(&self) -> u32 {
        unsafe { call!(self, spReflectionVariable_GetUserAttributeCount,) }
    }
    pub fn user_attribute_by_index(&self, index: u32) -> Option<&ReflectionUserAttribute> {
        unsafe { call_ret_cast_opt!(self, spReflectionVariable_GetUserAttribute as ReflectionUserAttribute, index) }
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
    pub fn size(&self, category: ParameterCategory) -> usize {
        unimplemented!()
    }
    pub fn kind(&self) -> TypeKind {
        unimplemented!()
    }
    pub fn field_by_index(&self, index: u32) -> Option<&ReflectionVariableLayout> {
        unimplemented!()
    }
    pub fn element_stride(&self, category: ParameterCategory) -> usize {
        unimplemented!()
    }
    pub fn element_type_layout(&self) -> &ReflectionTypeLayout {
        unimplemented!()
    }
    pub fn element_var_layout(&self) -> &ReflectionVariableLayout {
        unimplemented!()
    }
    pub fn parameter_category(&self) -> ParameterCategory {
        unimplemented!()
    }
    pub fn parameter_category_count(&self) -> u32 {
        unimplemented!()
    }
    pub fn parameter_category_by_index(&self, index: u32) -> Option<ParameterCategory> {
        unimplemented!()
    }
    pub fn matrix_layout_mode(&self) -> MatrixLayoutMode {
        unimplemented!()
    }
    pub fn generic_param_index(&self) -> u32 {
        unimplemented!()
    }
}

impl ReflectionType {
    pub fn name(&self) -> &str {
        unimplemented!()
    }
    pub fn kind(&self) -> TypeKind {
        unimplemented!()
    }
    pub fn user_attribute_count(&self) -> u32 {
        unimplemented!()
    }
    pub fn user_attribute_by_index(&self, index: u32) -> Option<&ReflectionUserAttribute> {
        unimplemented!()
    }
    pub fn find_user_attribute_by_name(&self, name: &str) -> Option<&ReflectionUserAttribute> {
        unimplemented!()
    }
    pub fn field_count(&self) -> u32 {
        unimplemented!()
    }
    pub fn field_by_index(&self, index: u32) -> Option<&ReflectionVariable> {
        unimplemented!()
    }
    pub fn element_count(&self) -> Option<usize> {
        unimplemented!()
    } // None if unbounded
    pub fn element_type(&self) -> Option<&ReflectionType> {
        unimplemented!()
    }
    pub fn row_count(&self) -> u32 {
        unimplemented!()
    }
    pub fn column_count(&self) -> u32 {
        unimplemented!()
    }
    pub fn scalar_type(&self) -> Option<ScalarType> {
        unimplemented!()
    }
    pub fn resource_shape(&self) -> ResourceShape {
        unimplemented!()
    }
    pub fn resource_access(&self) -> ResourceAccess {
        unimplemented!()
    }
    pub fn resource_result_type(&self) -> ReflectionType {
        unimplemented!()
    }
}

impl ReflectionEntryPoint {}
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
