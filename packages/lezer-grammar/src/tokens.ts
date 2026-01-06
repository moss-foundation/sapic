import { ExternalTokenizer } from "@lezer/lr";

// These are the IDs of the tokens we will define in the grammar
import { Raw } from "./variables.grammar.terms";

const brace = 123,
  colon = 58,
  slash = 47,
  closingBrace = 125;

// Identifier must start with letters
function isAlpha(ch: number) {
  return (
    (ch >= 65 && ch <= 90) || // A-Z
    (ch >= 97 && ch <= 122)
  ); // a-z
}

// Check if character is alphanumeric or underscore
function isAlphaNumOrUnderscore(ch: number) {
  return (
    isAlpha(ch) ||
    (ch >= 48 && ch <= 57) || // 0-9
    ch === 95 // _
  );
}

// Check if we have a complete {{identifier}} pattern starting at current position
function hasCompleteVariable(input: any): boolean {
  let pos = 0;

  // Check for {{
  if (input.peek(pos) !== brace || input.peek(pos + 1) !== brace) {
    return false;
  }

  pos += 2;

  // Check for identifier starting with letter
  if (!isAlpha(input.peek(pos))) {
    return false;
  }

  // Consume rest of identifier (alphanumeric and underscore)
  pos++;
  while (isAlphaNumOrUnderscore(input.peek(pos))) {
    pos++;
  }

  // Check for }}
  if (input.peek(pos) !== closingBrace || input.peek(pos + 1) !== closingBrace) {
    return false;
  }

  return true;
}

// Check if we have a complete /:identifier pattern starting at current position
function hasCompletePathParam(input: any): boolean {
  let pos = 0;

  // Check for /
  if (input.peek(pos) !== slash) {
    return false;
  }

  pos++;

  // Check for :
  if (input.peek(pos) !== colon) {
    return false;
  }

  pos++;

  // Check for identifier starting with letter
  if (!isAlpha(input.peek(pos))) {
    return false;
  }

  // Identifier is valid (we don't need to check the end for path params)
  return true;
}

export const rawTokenizer = new ExternalTokenizer((input, stack) => {
  let matched = false;

  // Loop until end of file
  while (input.next !== -1) {
    // Check for Variable "{{var}}"
    // First "{"
    if (input.next === brace) {
      // The second "{"
      if (input.peek(1) === brace) {
        // Only break if we have a complete {{identifier}} pattern
        if (hasCompleteVariable(input)) {
          // Matched, break from raw tokenizing so that the lezer grammar can parse "{{"
          break;
        }
        // Not a valid complete variable, continue to treat it as raw string
      }
    }

    // Check for Path Param "/:path"
    // It should only match colon immediately following a slash
    else if (input.next === slash) {
      // Match `:`
      if (input.peek(1) == colon) {
        // Only break if we have a complete /:identifier pattern
        if (hasCompletePathParam(input)) {
          // Matched, break from raw tokenizing so that the lezer grammar can parse slash
          break;
        }
        // Not a valid path param, continue to treat it as raw string
      }
    }

    input.advance();
    matched = true;
  }

  if (matched) {
    input.acceptToken(Raw);
  }
});
