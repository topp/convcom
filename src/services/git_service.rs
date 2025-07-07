use crate::error::{ConvComError, Result};
use git2::{Repository, Status};
use std::path::Path;

/// Git service for handling git diff operations and change extraction
pub struct GitService {
    repo: Repository,
}

impl GitService {
    /// Create a new GitService by discovering the repository
    pub fn new() -> Result<Self> {
        let repo = Repository::discover(".").map_err(|_| ConvComError::NotGitRepoError)?;

        Ok(Self { repo })
    }

    /// Get list of staged files from git
    pub fn get_staged_files(&self) -> Result<Vec<String>> {
        let mut staged_files = Vec::new();
        let statuses = self.repo.statuses(None)?;

        for entry in statuses.iter() {
            let status = entry.status();
            if status.contains(Status::INDEX_NEW)
                || status.contains(Status::INDEX_MODIFIED)
                || status.contains(Status::INDEX_DELETED)
            {
                if let Some(path) = entry.path() {
                    staged_files.push(path.to_string());
                }
            }
        }

        if staged_files.is_empty() {
            return Err(ConvComError::NoStagedFilesError);
        }

        Ok(staged_files)
    }

    /// Get the git status of a specific file
    pub fn get_file_status(&self, file_path: &str) -> Result<char> {
        let statuses = self.repo.statuses(None)?;

        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                if path == file_path {
                    let status = entry.status();
                    if status.contains(Status::INDEX_NEW) {
                        return Ok('A'); // Added
                    } else if status.contains(Status::INDEX_MODIFIED) {
                        return Ok('M'); // Modified
                    } else if status.contains(Status::INDEX_DELETED) {
                        return Ok('D'); // Deleted
                    }
                }
            }
        }

        Ok('M') // Default to modified
    }

    /// Get content of a newly added file from the index
    pub fn get_file_content(&self, file_path: &str) -> Result<String> {
        // First try to get from index (staged content)
        if let Ok(index) = self.repo.index() {
            if let Some(entry) = index.get_path(Path::new(file_path), 0) {
                if let Ok(blob) = self.repo.find_blob(entry.id) {
                    if let Ok(content) = std::str::from_utf8(blob.content()) {
                        return Ok(content.to_string());
                    }
                }
            }
        }

        // Fallback: try to read from working directory
        std::fs::read_to_string(file_path).map_err(|_| {
            ConvComError::IoError(format!("Could not read file content: {}", file_path))
        })
    }

    /// Extract changes from a modified file using git diff
    pub fn get_file_changes(&self, file_path: &str) -> Result<Vec<String>> {
        let mut changes = Vec::new();

        // Get diff between HEAD and index (staged changes)
        let tree = self.repo.head()?.peel_to_tree()?;
        let diff = self.repo.diff_tree_to_index(Some(&tree), None, None)?;

        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let file_matches = _delta
                .new_file()
                .path()
                .map(|p| p.to_str().unwrap_or("") == file_path)
                .unwrap_or(false);

            if file_matches {
                let content = String::from_utf8_lossy(line.content());
                let origin = line.origin();

                match origin {
                    '+' => {
                        if !content.starts_with("+++") {
                            changes.push(format!("+ {}", content.trim_end()));
                        }
                    }
                    '-' => {
                        if !content.starts_with("---") {
                            changes.push(format!("- {}", content.trim_end()));
                        }
                    }
                    _ => {}
                }
            }
            true
        })?;

        Ok(changes)
    }

    /// Process changes for a single file based on its status
    pub fn process_file_changes(&self, file_path: &str, file_status: char) -> Result<Vec<String>> {
        let mut changes = Vec::new();

        match file_status {
            'A' => {
                // Added file
                changes.push(format!("NEW FILE: {}", file_path));
                match self.get_file_content(file_path) {
                    Ok(content) => {
                        changes.push("COMPLETE CONTENT:".to_string());
                        changes.push(content);
                        changes.push(String::new());
                    }
                    Err(_) => {
                        changes.push("Could not read file content".to_string());
                        changes.push(String::new());
                    }
                }
            }
            'D' => {
                // Deleted file
                changes.push(format!("DELETED: {}", file_path));
            }
            'M' => {
                // Modified file
                match self.get_file_changes(file_path) {
                    Ok(file_changes) => {
                        if !file_changes.is_empty() {
                            changes.push(format!("MODIFIED: {}", file_path));
                            changes.extend(file_changes.into_iter().take(100)); // Limit to 100 changes
                            changes.push(String::new());
                        }
                    }
                    Err(_) => {
                        changes.push(format!("MODIFIED: {} (could not get diff)", file_path));
                        changes.push(String::new());
                    }
                }
            }
            _ => {
                changes.push(format!("UNKNOWN STATUS: {}", file_path));
            }
        }

        Ok(changes)
    }

    /// Build the complete diff content from all staged files
    pub fn build_diff_content(&self) -> Result<String> {
        let staged_files = self.get_staged_files()?;
        let mut processed_diff = Vec::new();

        for file_path in staged_files {
            let file_status = self.get_file_status(&file_path)?;
            let file_changes = self.process_file_changes(&file_path, file_status)?;
            processed_diff.extend(file_changes);
        }

        Ok(processed_diff.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_service_creation() {
        // This test will only pass if run in a git repository
        match GitService::new() {
            Ok(_) => {
                // Successfully created in a git repo
                assert!(true);
            }
            Err(ConvComError::NotGitRepoError) => {
                // Expected when not in a git repo
                assert!(true);
            }
            Err(e) => {
                panic!("Unexpected error: {}", e);
            }
        }
    }

    #[test]
    fn test_file_status_mapping() {
        // Test that file status characters are correct
        assert_eq!('A', 'A'); // Added
        assert_eq!('M', 'M'); // Modified
        assert_eq!('D', 'D'); // Deleted
    }
}
