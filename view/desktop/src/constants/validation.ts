/**
 * Validation patterns for form inputs
 */

/**
 * Pattern for valid workspace and project names.
 * Allows letters, numbers, spaces, dots, underscores, and hyphens.
 * Excludes filesystem-unsafe characters like: / \ : * ? " < > |
 */
//TODO the pattern should come from the backend in the future
export const VALID_NAME_PATTERN = "[a-zA-Zа-яА-Я0-9 \\._\\-]+";
