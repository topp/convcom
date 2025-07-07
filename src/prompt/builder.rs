use crate::error::{ConvComError, Result};
use std::collections::HashMap;

/// Professional prompt building system for conventional commit generation using templates
pub struct PromptBuilder {
    template: String,
}

impl PromptBuilder {
    /// Initialize the prompt builder with the commit template
    pub fn new() -> Result<Self> {
        let template = Self::load_template()?;
        Ok(Self { template })
    }

    /// Load the commit template from embedded resource or file
    fn load_template() -> Result<String> {
        // First try to load from embedded template (for releases)
        let embedded_template = include_str!("../../templates/commit_template.txt");

        // Validate that the template contains required placeholders
        if embedded_template.contains("$diff_content")
            && embedded_template.contains("$focus_section")
        {
            Ok(embedded_template.to_string())
        } else {
            Err(ConvComError::TemplateError(
                "Template missing required placeholders".to_string(),
            ))
        }
    }

    /// Build a complete prompt for conventional commit generation
    pub fn build_prompt(&self, diff_content: &str, focus_message: Option<&str>) -> Result<String> {
        let focus_section = self.build_focus_section(focus_message);
        let focus_reminder = self.build_focus_reminder(focus_message);

        // Create substitution map
        let mut substitutions = HashMap::new();
        substitutions.insert("$diff_content", diff_content);
        substitutions.insert("$focus_section", &focus_section);
        substitutions.insert("$focus_reminder", &focus_reminder);

        // Perform template substitution
        let mut result = self.template.clone();
        for (placeholder, value) in substitutions {
            result = result.replace(placeholder, value);
        }

        Ok(result)
    }

    /// Build the focus section for the prompt header
    fn build_focus_section(&self, focus_message: Option<&str>) -> String {
        match focus_message {
            Some(message) => format!(
                "\n🚨 CRITICAL USER REQUIREMENT 🚨\n{message}\n🚨 THIS MUST BE APPLIED TO YOUR OUTPUT 🚨\n\n"
            ),
            None => String::new(),
        }
    }

    /// Build the focus reminder for the prompt instructions
    fn build_focus_reminder(&self, focus_message: Option<&str>) -> String {
        match focus_message {
            Some(message) => format!(
                "\n\n🚨 REMINDER: APPLY THIS REQUIREMENT TO YOUR COMMIT MESSAGE 🚨\n{message}\n🚨 THIS IS MANDATORY - DO NOT IGNORE 🚨"
            ),
            None => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder_creation() {
        let builder = PromptBuilder::new();
        assert!(builder.is_ok());
    }

    #[test]
    fn test_build_prompt_without_focus() {
        let builder = PromptBuilder::new().unwrap();
        let diff_content = "MODIFIED: test.py\n+ print('hello')";

        let result = builder.build_prompt(diff_content, None).unwrap();

        assert!(result.contains("<<CONVENTIONAL COMMITS v1.0.0"));
        assert!(result.contains(diff_content));
        assert!(!result.contains("🚨 CRITICAL USER REQUIREMENT 🚨"));
    }

    #[test]
    fn test_build_prompt_with_focus() {
        let builder = PromptBuilder::new().unwrap();
        let diff_content = "MODIFIED: test.py\n+ print('hello')";
        let focus_message = "keep it concise";

        let result = builder
            .build_prompt(diff_content, Some(focus_message))
            .unwrap();

        assert!(result.contains("<<CONVENTIONAL COMMITS v1.0.0"));
        assert!(result.contains(diff_content));
        assert!(result.contains("🚨 CRITICAL USER REQUIREMENT 🚨"));
        assert!(result.contains(focus_message));
        assert!(result.contains("🚨 REMINDER: APPLY THIS REQUIREMENT"));
    }

    #[test]
    fn test_focus_section_generation() {
        let builder = PromptBuilder::new().unwrap();

        // Test without focus
        let result = builder.build_focus_section(None);
        assert_eq!(result, "");

        // Test with focus
        let focus_message = "test focus";
        let result = builder.build_focus_section(Some(focus_message));
        assert!(result.contains(focus_message));
        assert!(result.contains("🚨 CRITICAL USER REQUIREMENT 🚨"));
    }

    #[test]
    fn test_focus_reminder_generation() {
        let builder = PromptBuilder::new().unwrap();

        // Test without focus
        let result = builder.build_focus_reminder(None);
        assert_eq!(result, "");

        // Test with focus
        let focus_message = "test focus";
        let result = builder.build_focus_reminder(Some(focus_message));
        assert!(result.contains(focus_message));
        assert!(result.contains("🚨 REMINDER: APPLY THIS REQUIREMENT"));
    }

    #[test]
    fn test_template_contains_required_elements() {
        let builder = PromptBuilder::new().unwrap();
        let template = &builder.template;

        // Check for key template elements
        assert!(template.contains("CONVENTIONAL COMMITS v1.0.0"));
        assert!(template.contains("GIT DIFF FORMAT"));
        assert!(template.contains("OUTPUT FORMAT REQUIREMENTS"));
        assert!(template.contains("$diff_content"));
        assert!(template.contains("$focus_section"));
        assert!(template.contains("$focus_reminder"));
    }
}
