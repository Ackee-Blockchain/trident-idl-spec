use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Idl {
    #[serde(default, skip_serializing_if = "is_default")]
    pub address: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata: IdlMetadata,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlMetadata {
    pub name: String,
    pub version: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlAccount {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub discriminator: IdlDiscriminator,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlTypeDefGeneric {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlGenericArg {
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
    Pubkey,
    Option(Box<IdlType>),
    Vec(Box<IdlType>),
    Array(Box<IdlType>, IdlArrayLen),
    Generic(String),
    Defined(DefinedType),
    PublicKey,
}

// For backwards compatibility with anchor IDL 28/29
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DefinedType {
    Simple(String),
    Complex {
        name: String,
        #[serde(default, skip_serializing_if = "is_default")]
        generics: Vec<IdlGenericArg>,
    },
}

pub type IdlDiscriminator = Vec<u8>;

/// Get whether the given data is the default of its type.
fn is_default<T: Default + PartialEq>(it: &T) -> bool {
    *it == T::default()
}
