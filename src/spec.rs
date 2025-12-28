use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Idl {
    // New format (v30+)
    #[serde(default, skip_serializing_if = "is_default")]
    pub address: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata: IdlMetadata,
    // Legacy format (v29) - name and version at root level
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub version: Option<String>,
    // Common fields
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Vec<String>,
    pub instructions: Vec<IdlInstruction>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub accounts: Vec<IdlAccount>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub events: Vec<IdlEvent>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub errors: Vec<IdlErrorCode>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub types: Vec<IdlTypeDef>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub constants: Vec<IdlConst>,
}

impl Idl {
    /// Get the program name, handling both v29 (root level) and v30+ (metadata) formats
    pub fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.metadata.name)
    }

    /// Get the program version, handling both v29 (root level) and v30+ (metadata) formats
    pub fn get_version(&self) -> &str {
        self.version.as_deref().unwrap_or(&self.metadata.version)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlMetadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub spec: String,
    #[serde(skip_serializing_if = "is_default")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub dependencies: Vec<IdlDependency>,
    #[serde(skip_serializing_if = "is_default")]
    pub contact: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub deployments: Option<IdlDeployments>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlDependency {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlDeployments {
    pub mainnet: Option<String>,
    pub testnet: Option<String>,
    pub devnet: Option<String>,
    pub localnet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstruction {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub discriminator: IdlDiscriminator,
    pub accounts: Vec<IdlInstructionAccountItem>,
    pub args: Vec<IdlField>,
    #[serde(skip_serializing_if = "is_default")]
    pub returns: Option<IdlType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlInstructionAccountItem {
    Composite(IdlInstructionAccounts),
    Single(IdlInstructionAccount),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstructionAccount {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default", alias = "isMut")]
    pub writable: bool,
    #[serde(default, skip_serializing_if = "is_default", alias = "isSigner")]
    pub signer: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub optional: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub pda: Option<IdlPda>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub relations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstructionAccounts {
    pub name: String,
    pub accounts: Vec<IdlInstructionAccountItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IdlPda {
    pub seeds: Vec<IdlSeed>,
    #[serde(skip_serializing_if = "is_default")]
    pub program: Option<IdlSeed>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlSeed {
    Const(IdlSeedConst),
    Arg(IdlSeedArg),
    Account(IdlSeedAccount),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IdlSeedConst {
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IdlSeedArg {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IdlSeedAccount {
    pub path: String,
    #[serde(skip_serializing_if = "is_default")]
    pub account: Option<String>,
}

/// Account definition - supports both v29 (with embedded type) and v30+ (name + discriminator only)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlAccount {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub discriminator: IdlDiscriminator,
    /// v29 format has embedded type definition
    #[serde(default, skip_serializing_if = "is_default", rename = "type")]
    pub ty: Option<IdlTypeDefTy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlEvent {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub discriminator: IdlDiscriminator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlConst {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Vec<String>,
    #[serde(rename = "type")]
    pub ty: IdlType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdlErrorCode {
    pub code: u32,
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlField {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Vec<String>,
    #[serde(rename = "type")]
    pub ty: IdlType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlTypeDef {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub docs: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub serialization: IdlSerialization,
    #[serde(skip_serializing_if = "is_default")]
    pub repr: Option<IdlRepr>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub generics: Vec<IdlTypeDefGeneric>,
    #[serde(rename = "type")]
    pub ty: IdlTypeDefTy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IdlSerialization {
    #[default]
    Borsh,
    Bytemuck,
    BytemuckUnsafe,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
#[non_exhaustive]
pub enum IdlRepr {
    Rust(IdlReprModifier),
    C(IdlReprModifier),
    Transparent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlReprModifier {
    #[serde(default, skip_serializing_if = "is_default")]
    pub packed: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub align: Option<usize>,
}

/// Generic type parameter definition - supports both v29 (string) and v30+ (object) formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlTypeDefGeneric {
    /// v29 format: just a string like "T"
    Simple(String),
    /// v30+ format: object with kind and name
    Complex(IdlTypeDefGenericComplex),
}

impl IdlTypeDefGeneric {
    /// Get the name of the generic parameter
    pub fn get_name(&self) -> &str {
        match self {
            IdlTypeDefGeneric::Simple(name) => name,
            IdlTypeDefGeneric::Complex(complex) => match complex {
                IdlTypeDefGenericComplex::Type { name } => name,
                IdlTypeDefGenericComplex::Const { name, .. } => name,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlTypeDefGenericComplex {
    Type {
        name: String,
    },
    Const {
        name: String,
        #[serde(rename = "type")]
        ty: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlTypeDefTy {
    Struct {
        #[serde(skip_serializing_if = "is_default")]
        fields: Option<IdlDefinedFields>,
    },
    Enum {
        variants: Vec<IdlEnumVariant>,
    },
    Type {
        alias: IdlType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlEnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub fields: Option<IdlDefinedFields>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlDefinedFields {
    Named(Vec<IdlField>),
    Tuple(Vec<IdlType>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdlArrayLen {
    Generic(String),
    #[serde(untagged)]
    Value(usize),
}

/// Generic argument used in defined types - supports both v29 and v30+ formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlGenericArg {
    /// v30+ format with explicit kind
    WithKind(IdlGenericArgWithKind),
    /// v29 format without kind - just type or generic reference
    Type {
        #[serde(rename = "type")]
        ty: IdlType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlGenericArgWithKind {
    Type {
        #[serde(rename = "type")]
        ty: IdlType,
    },
    Const {
        value: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IdlDefinedTypeArg {
    Generic(String),
    Value(String),
    Type(IdlType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum IdlType {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    U128,
    I128,
    U256,
    I256,
    Bytes,
    String,
    /// v30+ format
    Pubkey,
    /// v29 format
    PublicKey,
    Option(Box<IdlType>),
    Vec(Box<IdlType>),
    Array(Box<IdlType>, IdlArrayLen),
    Generic(String),
    /// v30+ format: { "defined": { "name": "...", "generics": [...] } }
    Defined(DefinedType),
    /// v29 format: { "definedWithTypeArgs": { "name": "...", "args": [...] } }
    DefinedWithTypeArgs(DefinedWithTypeArgs),
}

impl IdlType {
    /// Check if this type represents a public key (either pubkey or publicKey)
    pub fn is_pubkey(&self) -> bool {
        matches!(self, IdlType::Pubkey | IdlType::PublicKey)
    }
}

/// v30+ format for defined types: { "defined": { "name": "...", "generics": [...] } }
/// Also supports v29 simple format: { "defined": "TypeName" }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DefinedType {
    /// v29 simple format: just a string
    Simple(String),
    /// v30+ format: object with name and optional generics
    Complex {
        name: String,
        #[serde(default, skip_serializing_if = "is_default")]
        generics: Vec<IdlGenericArg>,
    },
}

impl DefinedType {
    /// Get the type name regardless of format
    pub fn get_name(&self) -> &str {
        match self {
            DefinedType::Simple(name) => name,
            DefinedType::Complex { name, .. } => name,
        }
    }

    /// Get the generics if any
    pub fn get_generics(&self) -> &[IdlGenericArg] {
        match self {
            DefinedType::Simple(_) => &[],
            DefinedType::Complex { generics, .. } => generics,
        }
    }
}

/// v29 format for defined types with generics: { "definedWithTypeArgs": { "name": "...", "args": [...] } }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DefinedWithTypeArgs {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub args: Vec<IdlGenericArg>,
}

pub type IdlDiscriminator = Vec<u8>;

/// Get whether the given data is the default of its type.
fn is_default<T: Default + PartialEq>(it: &T) -> bool {
    *it == T::default()
}
