pub mod spec;
pub mod utils;

pub use spec::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_v29_idl() {
        let json = include_str!("../idl-examples/29/idl_test.json");
        let idl: Idl = serde_json::from_str(json).expect("Failed to parse v29 IDL");
        assert_eq!(idl.get_name(), "idl_test");
        assert_eq!(idl.get_version(), "0.1.0");
        assert!(!idl.instructions.is_empty());
        assert!(!idl.accounts.is_empty());
        // v29 has embedded type in accounts
        assert!(idl.accounts[0].ty.is_some());
    }

    #[test]
    fn test_parse_v30_idl() {
        let json = include_str!("../idl-examples/30/idl_test.json");
        let idl: Idl = serde_json::from_str(json).expect("Failed to parse v30 IDL");
        assert_eq!(idl.get_name(), "idl_test");
        assert_eq!(idl.get_version(), "0.1.0");
        assert_eq!(idl.address, "HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN");
        assert!(!idl.instructions.is_empty());
        // v30+ has discriminators in accounts
        assert!(!idl.accounts[0].discriminator.is_empty());
    }

    #[test]
    fn test_parse_v31_idl() {
        let json = include_str!("../idl-examples/31/idl_test.json");
        let idl: Idl = serde_json::from_str(json).expect("Failed to parse v31 IDL");
        assert_eq!(idl.get_name(), "idl_test");
        assert!(!idl.instructions.is_empty());
    }

    #[test]
    fn test_parse_v32_idl() {
        let json = include_str!("../idl-examples/32/idl_test.json");
        let idl: Idl = serde_json::from_str(json).expect("Failed to parse v32 IDL");
        assert_eq!(idl.get_name(), "idl_test");
        assert!(!idl.instructions.is_empty());
    }

    #[test]
    fn test_parse_additional_program_v29() {
        let json = include_str!("../idl-examples/29/additional_program.json");
        let idl: Idl =
            serde_json::from_str(json).expect("Failed to parse v29 additional_program IDL");
        assert_eq!(idl.get_name(), "additional_program");
    }

    #[test]
    fn test_parse_additional_program_v30() {
        let json = include_str!("../idl-examples/30/additional_program.json");
        let idl: Idl =
            serde_json::from_str(json).expect("Failed to parse v30 additional_program IDL");
        assert_eq!(idl.get_name(), "additional_program");
    }

    #[test]
    fn test_parse_additional_program_v31() {
        let json = include_str!("../idl-examples/31/additional_program.json");
        let idl: Idl =
            serde_json::from_str(json).expect("Failed to parse v31 additional_program IDL");
        assert_eq!(idl.get_name(), "additional_program");
    }

    #[test]
    fn test_parse_additional_program_v32() {
        let json = include_str!("../idl-examples/32/additional_program.json");
        let idl: Idl =
            serde_json::from_str(json).expect("Failed to parse v32 additional_program IDL");
        assert_eq!(idl.get_name(), "additional_program");
    }
}
