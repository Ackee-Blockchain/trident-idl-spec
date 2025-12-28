pub mod spec;
pub mod utils;

pub use spec::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Basic Parsing Tests ====================

    mod v29_parsing {
        use super::*;

        #[test]
        fn test_parse_idl_test() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            // v29 has name/version at root level
            assert_eq!(idl.name, Some("idl_test".to_string()));
            assert_eq!(idl.version, Some("0.1.0".to_string()));
            assert_eq!(idl.get_name(), "idl_test");
            assert_eq!(idl.get_version(), "0.1.0");

            // v29 should not have address
            assert!(idl.address.is_empty());

            assert!(!idl.instructions.is_empty());
            assert!(!idl.accounts.is_empty());
            assert!(!idl.types.is_empty());
            assert!(!idl.errors.is_empty());
        }

        #[test]
        fn test_accounts_have_embedded_type() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            // v29 has embedded type in accounts
            for account in &idl.accounts {
                assert!(
                    account.ty.is_some(),
                    "v29 account '{}' should have embedded type",
                    account.name
                );
                assert!(
                    account.discriminator.is_empty(),
                    "v29 account '{}' should not have discriminator",
                    account.name
                );
            }
        }

        #[test]
        fn test_instruction_accounts_with_is_mut_is_signer() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            let instruction = &idl.instructions[0];

            // Find the signer account - should have isSigner/isMut mapped to signer/writable
            let signer_account = instruction.accounts.iter().find_map(|item| match item {
                IdlInstructionAccountItem::Single(acc) if acc.name == "signer" => Some(acc),
                _ => None,
            });

            assert!(signer_account.is_some(), "Should find signer account");
            let signer = signer_account.unwrap();
            assert!(
                signer.signer,
                "signer should have signer=true (from isSigner)"
            );
            assert!(
                signer.writable,
                "signer should have writable=true (from isMut)"
            );
        }

        #[test]
        fn test_defined_type_simple_format() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            // Find an instruction arg with a simple defined type
            let instruction = idl
                .instructions
                .iter()
                .find(|i| i.name == "processCustomTypes")
                .expect("Should find processCustomTypes instruction");

            let classic_arg = instruction
                .args
                .iter()
                .find(|a| a.name == "inputClassic")
                .expect("Should find inputClassic arg");

            match &classic_arg.ty {
                IdlType::Defined(DefinedType::Simple(name)) => {
                    assert_eq!(name, "ClassicStruct");
                }
                _ => panic!("Expected Defined(Simple) type, got {:?}", classic_arg.ty),
            }
        }

        #[test]
        fn test_defined_with_type_args() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            let instruction = idl
                .instructions
                .iter()
                .find(|i| i.name == "processCustomTypes")
                .expect("Should find processCustomTypes instruction");

            let generic_arg = instruction
                .args
                .iter()
                .find(|a| a.name == "inputGenericStruct")
                .expect("Should find inputGenericStruct arg");

            match &generic_arg.ty {
                IdlType::DefinedWithTypeArgs(def) => {
                    assert_eq!(def.name, "GenericStruct");
                    assert!(!def.args.is_empty(), "Should have type args");
                }
                _ => panic!(
                    "Expected DefinedWithTypeArgs type, got {:?}",
                    generic_arg.ty
                ),
            }
        }

        #[test]
        fn test_generics_simple_format() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            let generic_struct = idl
                .types
                .iter()
                .find(|t| t.name == "GenericStruct")
                .expect("Should find GenericStruct type");

            assert!(!generic_struct.generics.is_empty());
            match &generic_struct.generics[0] {
                IdlTypeDefGeneric::Simple(name) => {
                    assert_eq!(name, "T");
                }
                _ => panic!(
                    "Expected Simple generic, got {:?}",
                    generic_struct.generics[0]
                ),
            }
            assert_eq!(generic_struct.generics[0].get_name(), "T");
        }

        #[test]
        fn test_public_key_type() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            let classic_struct = idl
                .types
                .iter()
                .find(|t| t.name == "ClassicStruct")
                .expect("Should find ClassicStruct");

            if let IdlTypeDefTy::Struct {
                fields: Some(IdlDefinedFields::Named(fields)),
            } = &classic_struct.ty
            {
                let field3 = fields.iter().find(|f| f.name == "field3").unwrap();
                assert!(
                    matches!(field3.ty, IdlType::PublicKey),
                    "field3 should be publicKey type"
                );
                assert!(field3.ty.is_pubkey());
            } else {
                panic!("Expected struct with named fields");
            }
        }

        #[test]
        fn test_parse_additional_program() {
            let json = include_str!("../idl-examples/29/additional_program.json");
            let idl: Idl =
                serde_json::from_str(json).expect("Failed to parse v29 additional_program IDL");
            assert_eq!(idl.get_name(), "additional_program");
        }
    }

    mod v30_parsing {
        use super::*;

        #[test]
        fn test_parse_idl_test() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            // v30+ has metadata object
            assert_eq!(idl.metadata.name, "idl_test");
            assert_eq!(idl.metadata.version, "0.1.0");
            assert_eq!(idl.metadata.spec, "0.1.0");
            assert_eq!(
                idl.metadata.description,
                Some("Created with Anchor".to_string())
            );

            // Helper methods should work
            assert_eq!(idl.get_name(), "idl_test");
            assert_eq!(idl.get_version(), "0.1.0");

            // v30+ has address
            assert_eq!(idl.address, "HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN");

            assert!(!idl.instructions.is_empty());
        }

        #[test]
        fn test_accounts_have_discriminator() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            // v30+ has discriminators in accounts
            for account in &idl.accounts {
                assert!(
                    !account.discriminator.is_empty(),
                    "v30+ account '{}' should have discriminator",
                    account.name
                );
                assert!(
                    account.ty.is_none(),
                    "v30+ account '{}' should not have embedded type",
                    account.name
                );
            }
        }

        #[test]
        fn test_instruction_discriminator() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            for instruction in &idl.instructions {
                assert!(
                    !instruction.discriminator.is_empty(),
                    "v30+ instruction '{}' should have discriminator",
                    instruction.name
                );
            }
        }

        #[test]
        fn test_defined_type_complex_format() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            let instruction = idl
                .instructions
                .iter()
                .find(|i| i.name == "process_custom_types")
                .expect("Should find process_custom_types instruction");

            let classic_arg = instruction
                .args
                .iter()
                .find(|a| a.name == "_input_classic")
                .expect("Should find _input_classic arg");

            match &classic_arg.ty {
                IdlType::Defined(DefinedType::Complex { name, generics }) => {
                    assert_eq!(name, "ClassicStruct");
                    assert!(generics.is_empty());
                }
                _ => panic!("Expected Defined(Complex) type, got {:?}", classic_arg.ty),
            }
        }

        #[test]
        fn test_defined_type_with_generics() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            let instruction = idl
                .instructions
                .iter()
                .find(|i| i.name == "process_custom_types")
                .expect("Should find process_custom_types instruction");

            let generic_arg = instruction
                .args
                .iter()
                .find(|a| a.name == "_input_generic_struct")
                .expect("Should find _input_generic_struct arg");

            match &generic_arg.ty {
                IdlType::Defined(DefinedType::Complex { name, generics }) => {
                    assert_eq!(name, "GenericStruct");
                    assert!(!generics.is_empty(), "Should have generics");
                }
                _ => panic!(
                    "Expected Defined(Complex) with generics, got {:?}",
                    generic_arg.ty
                ),
            }
        }

        #[test]
        fn test_generics_complex_format() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            let generic_struct = idl
                .types
                .iter()
                .find(|t| t.name == "GenericStruct")
                .expect("Should find GenericStruct type");

            assert!(!generic_struct.generics.is_empty());
            match &generic_struct.generics[0] {
                IdlTypeDefGeneric::Complex(IdlTypeDefGenericComplex::Type { name }) => {
                    assert_eq!(name, "T");
                }
                _ => panic!(
                    "Expected Complex generic, got {:?}",
                    generic_struct.generics[0]
                ),
            }
            assert_eq!(generic_struct.generics[0].get_name(), "T");
        }

        #[test]
        fn test_pubkey_type() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            let classic_struct = idl
                .types
                .iter()
                .find(|t| t.name == "ClassicStruct")
                .expect("Should find ClassicStruct");

            if let IdlTypeDefTy::Struct {
                fields: Some(IdlDefinedFields::Named(fields)),
            } = &classic_struct.ty
            {
                let field3 = fields.iter().find(|f| f.name == "field3").unwrap();
                assert!(
                    matches!(field3.ty, IdlType::Pubkey),
                    "field3 should be pubkey type"
                );
                assert!(field3.ty.is_pubkey());
            } else {
                panic!("Expected struct with named fields");
            }
        }

        #[test]
        fn test_parse_additional_program() {
            let json = include_str!("../idl-examples/30/additional_program.json");
            let idl: Idl =
                serde_json::from_str(json).expect("Failed to parse v30 additional_program IDL");
            assert_eq!(idl.get_name(), "additional_program");
        }
    }

    mod v31_parsing {
        use super::*;

        #[test]
        fn test_parse_idl_test() {
            let json = include_str!("../idl-examples/31/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v31 IDL");
            assert_eq!(idl.get_name(), "idl_test");
            assert!(!idl.instructions.is_empty());
            assert!(!idl.accounts.is_empty());
            assert!(!idl.types.is_empty());
        }

        #[test]
        fn test_parse_additional_program() {
            let json = include_str!("../idl-examples/31/additional_program.json");
            let idl: Idl =
                serde_json::from_str(json).expect("Failed to parse v31 additional_program IDL");
            assert_eq!(idl.get_name(), "additional_program");
        }
    }

    mod v32_parsing {
        use super::*;

        #[test]
        fn test_parse_idl_test() {
            let json = include_str!("../idl-examples/32/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v32 IDL");
            assert_eq!(idl.get_name(), "idl_test");
            assert!(!idl.instructions.is_empty());
            assert!(!idl.accounts.is_empty());
            assert!(!idl.types.is_empty());
        }

        #[test]
        fn test_parse_additional_program() {
            let json = include_str!("../idl-examples/32/additional_program.json");
            let idl: Idl =
                serde_json::from_str(json).expect("Failed to parse v32 additional_program IDL");
            assert_eq!(idl.get_name(), "additional_program");
        }
    }

    // ==================== Composite Account Tests ====================

    mod composite_accounts {
        use super::*;

        #[test]
        fn test_composite_accounts_v29() {
            let json = include_str!("../idl-examples/29/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");

            let instruction = &idl.instructions[0];

            // Find composite account
            let composite = instruction.accounts.iter().find_map(|item| match item {
                IdlInstructionAccountItem::Composite(acc)
                    if acc.name == "compositeAccountNested" =>
                {
                    Some(acc)
                }
                _ => None,
            });

            assert!(composite.is_some(), "Should find compositeAccountNested");
            let composite = composite.unwrap();
            assert!(!composite.accounts.is_empty());

            // Check for nested composite
            let nested_inner = composite.accounts.iter().find_map(|item| match item {
                IdlInstructionAccountItem::Composite(acc) if acc.name == "nestedInner" => Some(acc),
                _ => None,
            });
            assert!(nested_inner.is_some(), "Should find nestedInner");
        }

        #[test]
        fn test_composite_accounts_v30() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");

            let instruction = &idl.instructions[0];

            // Find composite account (snake_case in v30+)
            let composite = instruction.accounts.iter().find_map(|item| match item {
                IdlInstructionAccountItem::Composite(acc)
                    if acc.name == "composite_account_nested" =>
                {
                    Some(acc)
                }
                _ => None,
            });

            assert!(composite.is_some(), "Should find composite_account_nested");
        }
    }

    // ==================== Type Definition Tests ====================

    mod type_definitions {
        use super::*;

        #[test]
        fn test_struct_with_named_fields() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let classic = idl
                .types
                .iter()
                .find(|t| t.name == "ClassicStruct")
                .unwrap();

            match &classic.ty {
                IdlTypeDefTy::Struct {
                    fields: Some(IdlDefinedFields::Named(fields)),
                } => {
                    assert_eq!(fields.len(), 3);
                    assert_eq!(fields[0].name, "field1");
                    assert!(matches!(fields[0].ty, IdlType::U8));
                }
                _ => panic!("Expected struct with named fields"),
            }
        }

        #[test]
        fn test_tuple_struct() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let tuple_struct = idl.types.iter().find(|t| t.name == "TupleStruct").unwrap();

            match &tuple_struct.ty {
                IdlTypeDefTy::Struct {
                    fields: Some(IdlDefinedFields::Tuple(types)),
                } => {
                    assert_eq!(types.len(), 3);
                    assert!(matches!(types[0], IdlType::U8));
                    assert!(matches!(types[1], IdlType::U16));
                    assert!(types[2].is_pubkey());
                }
                _ => panic!("Expected tuple struct"),
            }
        }

        #[test]
        fn test_unit_struct() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let unit_struct = idl.types.iter().find(|t| t.name == "UnitStruct").unwrap();

            match &unit_struct.ty {
                IdlTypeDefTy::Struct { fields: None } => {}
                _ => panic!("Expected unit struct with no fields"),
            }
        }

        #[test]
        fn test_simple_enum() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let simple_enum = idl.types.iter().find(|t| t.name == "SimpleEnum").unwrap();

            match &simple_enum.ty {
                IdlTypeDefTy::Enum { variants } => {
                    assert_eq!(variants.len(), 3);
                    assert_eq!(variants[0].name, "Variant1");
                    assert!(variants[0].fields.is_none());
                }
                _ => panic!("Expected enum"),
            }
        }

        #[test]
        fn test_enum_with_tuple_fields() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let data_enum = idl.types.iter().find(|t| t.name == "DataEnum").unwrap();

            match &data_enum.ty {
                IdlTypeDefTy::Enum { variants } => {
                    let integer_variant = variants.iter().find(|v| v.name == "Integer").unwrap();
                    match &integer_variant.fields {
                        Some(IdlDefinedFields::Tuple(types)) => {
                            assert_eq!(types.len(), 1);
                            assert!(matches!(types[0], IdlType::I32));
                        }
                        _ => panic!("Expected tuple fields"),
                    }
                }
                _ => panic!("Expected enum"),
            }
        }

        #[test]
        fn test_enum_with_named_fields() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let named_enum = idl
                .types
                .iter()
                .find(|t| t.name == "NamedFieldsEnum")
                .unwrap();

            match &named_enum.ty {
                IdlTypeDefTy::Enum { variants } => {
                    let point_variant = variants.iter().find(|v| v.name == "Point").unwrap();
                    match &point_variant.fields {
                        Some(IdlDefinedFields::Named(fields)) => {
                            assert_eq!(fields.len(), 2);
                            assert_eq!(fields[0].name, "x");
                            assert!(matches!(fields[0].ty, IdlType::F64));
                        }
                        _ => panic!("Expected named fields"),
                    }
                }
                _ => panic!("Expected enum"),
            }
        }
    }

    // ==================== Primitive Types Tests ====================

    mod primitive_types {
        use super::*;

        #[test]
        fn test_all_primitive_types() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let instruction = idl
                .instructions
                .iter()
                .find(|i| i.name == "process_rust_types")
                .unwrap();

            let types_to_check = [
                ("_input_u8", IdlType::U8),
                ("_input_u16", IdlType::U16),
                ("_input_u32", IdlType::U32),
                ("_input_u64", IdlType::U64),
                ("_input_i8", IdlType::I8),
                ("_input_i16", IdlType::I16),
                ("_input_i32", IdlType::I32),
                ("_input_i64", IdlType::I64),
                ("_input_i128", IdlType::I128),
                ("_input_f32", IdlType::F32),
                ("_input_f64", IdlType::F64),
                ("_input_string", IdlType::String),
                ("_input_bool", IdlType::Bool),
                ("_input_vec", IdlType::Bytes),
            ];

            for (name, expected_type) in types_to_check {
                let arg = instruction.args.iter().find(|a| a.name == name);
                assert!(arg.is_some(), "Should find arg '{}'", name);
                assert_eq!(
                    arg.unwrap().ty,
                    expected_type,
                    "Arg '{}' should have correct type",
                    name
                );
            }
        }

        #[test]
        fn test_vec_type() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let instruction = idl
                .instructions
                .iter()
                .find(|i| i.name == "process_rust_types")
                .unwrap();

            let vec_arg = instruction
                .args
                .iter()
                .find(|a| a.name == "_input_vec_string")
                .unwrap();

            match &vec_arg.ty {
                IdlType::Vec(inner) => {
                    assert!(matches!(**inner, IdlType::String));
                }
                _ => panic!("Expected Vec type"),
            }
        }

        #[test]
        fn test_option_type() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let optional_fields = idl
                .types
                .iter()
                .find(|t| t.name == "OptionalFields")
                .unwrap();

            if let IdlTypeDefTy::Struct {
                fields: Some(IdlDefinedFields::Named(fields)),
            } = &optional_fields.ty
            {
                let field1 = fields.iter().find(|f| f.name == "field1").unwrap();
                match &field1.ty {
                    IdlType::Option(inner) => {
                        assert!(matches!(**inner, IdlType::U8));
                    }
                    _ => panic!("Expected Option type"),
                }
            }
        }
    }

    // ==================== Error Tests ====================

    mod errors {
        use super::*;

        #[test]
        fn test_error_codes() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            assert_eq!(idl.errors.len(), 5);

            let input_too_long = idl
                .errors
                .iter()
                .find(|e| e.name == "InputTooLong")
                .unwrap();
            assert_eq!(input_too_long.code, 6000);
            assert_eq!(
                input_too_long.msg,
                Some("The input is too long".to_string())
            );

            let input_too_short = idl
                .errors
                .iter()
                .find(|e| e.name == "InputTooShort")
                .unwrap();
            assert_eq!(input_too_short.code, 6001);
        }
    }

    // ==================== Helper Method Tests ====================

    mod helper_methods {
        use super::*;

        #[test]
        fn test_defined_type_get_name() {
            let simple = DefinedType::Simple("MyType".to_string());
            assert_eq!(simple.get_name(), "MyType");

            let complex = DefinedType::Complex {
                name: "MyGenericType".to_string(),
                generics: vec![],
            };
            assert_eq!(complex.get_name(), "MyGenericType");
        }

        #[test]
        fn test_defined_type_get_generics() {
            let simple = DefinedType::Simple("MyType".to_string());
            assert!(simple.get_generics().is_empty());

            let complex = DefinedType::Complex {
                name: "MyGenericType".to_string(),
                generics: vec![IdlGenericArg::Type { ty: IdlType::U8 }],
            };
            assert_eq!(complex.get_generics().len(), 1);
        }

        #[test]
        fn test_idl_type_is_pubkey() {
            assert!(IdlType::Pubkey.is_pubkey());
            assert!(IdlType::PublicKey.is_pubkey());
            assert!(!IdlType::U8.is_pubkey());
            assert!(!IdlType::String.is_pubkey());
        }

        #[test]
        fn test_idl_type_def_generic_get_name() {
            let simple = IdlTypeDefGeneric::Simple("T".to_string());
            assert_eq!(simple.get_name(), "T");

            let complex_type = IdlTypeDefGeneric::Complex(IdlTypeDefGenericComplex::Type {
                name: "U".to_string(),
            });
            assert_eq!(complex_type.get_name(), "U");

            let complex_const = IdlTypeDefGeneric::Complex(IdlTypeDefGenericComplex::Const {
                name: "N".to_string(),
                ty: "usize".to_string(),
            });
            assert_eq!(complex_const.get_name(), "N");
        }
    }

    // ==================== Utils Tests ====================

    mod utils_tests {
        use super::*;

        #[test]
        fn test_idl_type_to_syn_type_primitives() {
            let test_cases = [
                (IdlType::Bool, "bool"),
                (IdlType::U8, "u8"),
                (IdlType::I8, "i8"),
                (IdlType::U16, "u16"),
                (IdlType::I16, "i16"),
                (IdlType::U32, "u32"),
                (IdlType::I32, "i32"),
                (IdlType::U64, "u64"),
                (IdlType::I64, "i64"),
                (IdlType::U128, "u128"),
                (IdlType::I128, "i128"),
                (IdlType::F32, "f32"),
                (IdlType::F64, "f64"),
                (IdlType::String, "String"),
                (IdlType::Bytes, "Vec < u8 >"),
            ];

            for (idl_type, expected) in test_cases {
                let (syn_type, is_custom) = idl_type_to_syn_type(&idl_type, 0, false);
                assert!(!is_custom, "{:?} should not be custom", idl_type);
                let type_str = quote::quote!(#syn_type).to_string();
                assert_eq!(type_str, expected, "Type mismatch for {:?}", idl_type);
            }
        }

        #[test]
        fn test_idl_type_to_syn_type_pubkey() {
            let (syn_type, _) = idl_type_to_syn_type(&IdlType::Pubkey, 0, false);
            assert_eq!(quote::quote!(#syn_type).to_string(), "Pubkey");

            let (syn_type, _) = idl_type_to_syn_type(&IdlType::PublicKey, 0, false);
            assert_eq!(quote::quote!(#syn_type).to_string(), "Pubkey");

            // With convert_pkey = true
            let (syn_type, _) = idl_type_to_syn_type(&IdlType::Pubkey, 0, true);
            assert_eq!(quote::quote!(#syn_type).to_string(), "AccountId");
        }

        #[test]
        fn test_idl_type_to_syn_type_option() {
            let opt_type = IdlType::Option(Box::new(IdlType::U32));
            let (syn_type, _) = idl_type_to_syn_type(&opt_type, 0, false);
            assert_eq!(quote::quote!(#syn_type).to_string(), "Option < u32 >");
        }

        #[test]
        fn test_idl_type_to_syn_type_vec() {
            let vec_type = IdlType::Vec(Box::new(IdlType::String));
            let (syn_type, _) = idl_type_to_syn_type(&vec_type, 0, false);
            assert_eq!(quote::quote!(#syn_type).to_string(), "Vec < String >");
        }

        #[test]
        fn test_idl_type_to_syn_type_array() {
            let arr_type = IdlType::Array(Box::new(IdlType::U8), IdlArrayLen::Value(32));
            let (syn_type, _) = idl_type_to_syn_type(&arr_type, 0, false);
            assert_eq!(quote::quote!(#syn_type).to_string(), "[u8 ; 32usize]");
        }

        #[test]
        fn test_idl_type_to_syn_type_defined() {
            let defined = IdlType::Defined(DefinedType::Simple("MyCustomType".to_string()));
            let (syn_type, is_custom) = idl_type_to_syn_type(&defined, 0, false);
            assert!(is_custom);
            assert_eq!(quote::quote!(#syn_type).to_string(), "MyCustomType");
        }

        #[test]
        fn test_idl_type_to_syn_type_defined_with_type_args() {
            let defined = IdlType::DefinedWithTypeArgs(DefinedWithTypeArgs {
                name: "GenericType".to_string(),
                args: vec![],
            });
            let (syn_type, is_custom) = idl_type_to_syn_type(&defined, 0, false);
            assert!(is_custom);
            assert_eq!(quote::quote!(#syn_type).to_string(), "GenericType");
        }

        #[test]
        fn test_idl_type_to_syn_type_generic() {
            let generic = IdlType::Generic("T".to_string());
            let (syn_type, is_custom) = idl_type_to_syn_type(&generic, 0, false);
            assert!(is_custom);
            assert_eq!(quote::quote!(#syn_type).to_string(), "T");
        }
    }

    // ==================== Instruction Account Address Tests ====================

    mod instruction_account_address {
        use super::*;

        #[test]
        fn test_account_with_address() {
            let json = include_str!("../idl-examples/30/idl_test.json");
            let idl: Idl = serde_json::from_str(json).expect("Failed to parse IDL");

            let instruction = &idl.instructions[0];

            // Find nested account with address (system_program)
            fn find_account_with_address(
                accounts: &[IdlInstructionAccountItem],
            ) -> Option<&IdlInstructionAccount> {
                for item in accounts {
                    match item {
                        IdlInstructionAccountItem::Single(acc) if acc.address.is_some() => {
                            return Some(acc);
                        }
                        IdlInstructionAccountItem::Composite(comp) => {
                            if let Some(found) = find_account_with_address(&comp.accounts) {
                                return Some(found);
                            }
                        }
                        _ => {}
                    }
                }
                None
            }

            let account_with_address = find_account_with_address(&instruction.accounts);
            assert!(
                account_with_address.is_some(),
                "Should find account with address"
            );

            let acc = account_with_address.unwrap();
            assert_eq!(
                acc.address,
                Some("11111111111111111111111111111111".to_string())
            );
        }
    }
}
