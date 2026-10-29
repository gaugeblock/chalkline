//! Reverse-direction spec audit: what the specification claims, the AST must still define.

use miter_engine::WorkspaceInventory;

/// Audits contracts the specification declares against the workspace AST.
///
/// This is the reverse of [`super::rules::audit_spec_contracts`]: that rule catches a type the code has
/// but the spec never documents; this one catches a contract the spec still documents but the AST no
/// longer defines, which is what makes a specification trustworthy as an input.
pub fn audit_spec_declared_contracts(
    content: &str,
    inv: &WorkspaceInventory,
    violations: &mut Vec<String>,
) {
    let declared = crate::spec_contracts::parse_contracts_from_markdown(content);
    for contract in declared {
        let present = inv
            .public_types
            .iter()
            .any(|t| t.type_name == contract.name);
        if !present {
            violations.push(format!(
                "SPEC declares shared contract '{}' with no matching type in the workspace AST",
                contract.name
            ));
        }
    }
}
