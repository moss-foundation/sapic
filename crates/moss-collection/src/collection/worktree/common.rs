use validator::ValidationError;

use super::snapshot::EntryRef;

pub(super) const ROOT_PATH: &str = "";

pub(crate) type Rule = Box<dyn Fn(&EntryRef) -> Result<(), ValidationError>>;

pub(crate) fn is_dir() -> Rule {
    Box::new(move |entry: &EntryRef| {
        if !entry.is_dir() {
            return Err(ValidationError::new("Entry is not a directory"));
        }
        Ok(())
    })
}

pub(crate) fn path_ends_with_extension(value: &'static str) -> Rule {
    Box::new(move |entry: &EntryRef| {
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
    Box::new(move |entry: &EntryRef| {
        let extension = entry.path.extension();
        if extension.unwrap_or_default() == value {
            return Err(ValidationError::new("Entry path ends with forbidden value"));
        }
        Ok(())
    })
}

pub(crate) fn path_starts_with(value: &'static str) -> Rule {
    Box::new(move |entry: &EntryRef| {
        if !entry.path.starts_with(value) {
            return Err(ValidationError::new(
                "Entry path starts with forbidden value",
            ));
        }
        Ok(())
    })
}

pub(crate) fn validate_entry(entry: &EntryRef, rules: &[Rule]) -> Result<(), ValidationError> {
    if rules.is_empty() {
        return Ok(());
    }

    for rule in rules {
        rule(entry)?;
    }

    Ok(())
}
