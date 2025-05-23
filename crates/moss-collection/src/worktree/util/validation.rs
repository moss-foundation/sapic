use std::sync::Arc;
use validator::ValidationError;

use crate::worktree::physical_snapshot::PhysicalEntry;

pub(crate) type Rule = Box<dyn Fn(&Arc<PhysicalEntry>) -> Result<(), ValidationError>>;

pub(crate) fn is_dir() -> Rule {
    Box::new(move |entry: &Arc<PhysicalEntry>| {
        if !entry.is_dir() {
            return Err(ValidationError::new("Entry is not a directory"));
        }
        Ok(())
    })
}

pub(crate) fn path_ends_with_extension(value: &'static str) -> Rule {
    Box::new(move |entry: &Arc<PhysicalEntry>| {
        let extension = entry.path.extension();
        if extension.unwrap_or_default() != value {
            return Err(ValidationError::new(
                "Entry path does not end with expected value",
            ));
        }
        Ok(())
    })
}

pub(crate) fn path_not_ends_with_extension(value: &'static str) -> Rule {
    Box::new(move |entry: &Arc<PhysicalEntry>| {
        let extension = entry.path.extension();
        if extension.unwrap_or_default() == value {
            return Err(ValidationError::new("Entry path ends with forbidden value"));
        }
        Ok(())
    })
}

pub(crate) fn path_starts_with(value: &'static str) -> Rule {
    Box::new(move |entry: &Arc<PhysicalEntry>| {
        if !entry.path.starts_with(value) {
            return Err(ValidationError::new(
                "Entry path starts with forbidden value",
            ));
        }
        Ok(())
    })
}

pub(crate) fn validate_entry(
    entry: &Arc<PhysicalEntry>,
    rules: &[Rule],
) -> Result<(), ValidationError> {
    if rules.is_empty() {
        return Ok(());
    }

    for rule in rules {
        rule(entry)?;
    }

    Ok(())
}
