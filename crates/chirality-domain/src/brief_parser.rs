//! Session brief parser for INIT-TASK format.
//!
//! From chirality-app: Task agents receive briefs in a structured format
//! that defines the task, scope, outputs, constraints, and success criteria.

use crate::entities::SessionBrief;
use crate::error::DomainError;

/// Parser for session briefs.
pub struct BriefParser;

impl BriefParser {
    /// Parse a session brief from structured input.
    pub fn parse(input: &serde_json::Value) -> Result<SessionBrief, DomainError> {
        let task_definition = input
            .get("task_definition")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::InvalidBrief {
                reason: "Missing task_definition".to_string(),
            })?
            .to_string();

        let scope_description = input
            .get("scope_description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let output_contract = Self::parse_string_array(input.get("output_contract"));
        let constraints = Self::parse_string_array(input.get("constraints"));
        let success_criteria = Self::parse_string_array(input.get("success_criteria"));

        let inputs = input
            .get("inputs")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        Ok(SessionBrief {
            task_definition,
            scope_description,
            output_contract,
            constraints,
            success_criteria,
            inputs,
        })
    }

    /// Validate a brief against an agent's expected format.
    pub fn validate(brief: &SessionBrief, agent_name: &str) -> Result<(), DomainError> {
        // Basic validation
        if brief.task_definition.is_empty() {
            return Err(DomainError::InvalidBrief {
                reason: "task_definition cannot be empty".to_string(),
            });
        }

        // Agent-specific validation
        match agent_name {
            "4_DOCUMENTS" => Self::validate_4_documents_brief(brief),
            "PREPARATION" => Self::validate_preparation_brief(brief),
            "CHIRALITY_FRAMEWORK" => Self::validate_chirality_framework_brief(brief),
            "DEPENDENCIES" => Self::validate_dependencies_brief(brief),
            "AGGREGATION" => Self::validate_aggregation_brief(brief),
            _ => Ok(()), // Unknown agents get no additional validation
        }
    }

    fn parse_string_array(value: Option<&serde_json::Value>) -> Vec<String> {
        value
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn validate_4_documents_brief(brief: &SessionBrief) -> Result<(), DomainError> {
        // 4_DOCUMENTS needs deliverable context
        if brief.inputs.get("deliverable_id").is_none() {
            return Err(DomainError::InvalidBrief {
                reason: "4_DOCUMENTS requires deliverable_id in inputs".to_string(),
            });
        }
        Ok(())
    }

    fn validate_preparation_brief(brief: &SessionBrief) -> Result<(), DomainError> {
        // PREPARATION needs package or project context
        if brief.inputs.get("package_id").is_none()
            && brief.inputs.get("project_id").is_none()
        {
            return Err(DomainError::InvalidBrief {
                reason: "PREPARATION requires package_id or project_id in inputs".to_string(),
            });
        }
        Ok(())
    }

    fn validate_chirality_framework_brief(brief: &SessionBrief) -> Result<(), DomainError> {
        // CHIRALITY_FRAMEWORK needs deliverable context
        if brief.inputs.get("deliverable_id").is_none() {
            return Err(DomainError::InvalidBrief {
                reason: "CHIRALITY_FRAMEWORK requires deliverable_id in inputs".to_string(),
            });
        }
        Ok(())
    }

    fn validate_dependencies_brief(brief: &SessionBrief) -> Result<(), DomainError> {
        // DEPENDENCIES needs scope (deliverable, package, or project)
        if brief.inputs.get("deliverable_id").is_none()
            && brief.inputs.get("package_id").is_none()
            && brief.inputs.get("project_id").is_none()
        {
            return Err(DomainError::InvalidBrief {
                reason: "DEPENDENCIES requires a scope (deliverable_id, package_id, or project_id)"
                    .to_string(),
            });
        }
        Ok(())
    }

    fn validate_aggregation_brief(brief: &SessionBrief) -> Result<(), DomainError> {
        // AGGREGATION needs project context and output path
        if brief.inputs.get("project_id").is_none() {
            return Err(DomainError::InvalidBrief {
                reason: "AGGREGATION requires project_id in inputs".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_valid_brief() {
        let input = json!({
            "task_definition": "Generate initial drafts for deliverable",
            "scope_description": "Single deliverable DEL-01.01",
            "output_contract": ["Datasheet.md", "Specification.md", "Guidance.md", "Procedure.md"],
            "constraints": ["Use existing references only", "Mark unknowns as TBD"],
            "success_criteria": ["All four documents exist", "No placeholder content"],
            "inputs": {
                "deliverable_id": "del:01234567890123456789012345"
            }
        });

        let brief = BriefParser::parse(&input).unwrap();
        assert_eq!(brief.task_definition, "Generate initial drafts for deliverable");
        assert_eq!(brief.output_contract.len(), 4);
    }

    #[test]
    fn validate_4_documents_requires_deliverable() {
        let brief = SessionBrief {
            task_definition: "Generate docs".to_string(),
            scope_description: String::new(),
            output_contract: vec![],
            constraints: vec![],
            success_criteria: vec![],
            inputs: json!({}),
        };

        let result = BriefParser::validate(&brief, "4_DOCUMENTS");
        assert!(result.is_err());
    }
}
