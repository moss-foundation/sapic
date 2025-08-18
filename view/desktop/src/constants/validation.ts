/**
 * Validation patterns for form inputs
 */

/**
 * Pattern for valid workspace and collection names.
 * Allows letters, numbers, spaces, dots, underscores, and hyphens.
 * Excludes filesystem-unsafe characters like: / \ : * ? " < > |
 */
//TODO the pattern should come from the backend in the future
export const VALID_NAME_PATTERN = "[a-zA-Zа-яА-Я0-9 \\._\\-]+";

/**
 * Pattern for valid identifiers (more restrictive)
 * Only allows letters, numbers, and underscores
 */
export const VALID_IDENTIFIER_PATTERN = "[a-zA-Z0-9_]+";
