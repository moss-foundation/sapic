import { ExternalTokenizer } from "@lezer/lr";
// These are the IDs of the tokens we will define in the grammar
import { Raw } from "./variables.grammar.terms";

const brace = 123,
  colon = 58,
  slash = 47;

// Identifier must start with letters
function isAlpha(ch: number) {
  return (
    (ch >= 65 && ch <= 90) || // A-Z
    (ch >= 97 && ch <= 122)
  ); // a-z
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
        // Check for beginning of identifier
        // It must start with a letter
        if (isAlpha(input.peek(2))) {
          // Matched, break from raw tokenizing so that the lezer grammar can parse "{{"
          break;
        }
        // Not a valid variable, continue to treat it as raw string
      }
    }

    // Check for Path Param "/:path"
    // It should only match colon immediately following a slash
    else if (input.next === slash) {
      // Match `:`
      if (input.peek(1) == colon) {
        // Check for beginning of identifier
        // It must start with a letter
        if (isAlpha(input.peek(2))) {
          // Matched, break from raw tokenizing so that the lezer grammar can parse slash
          break;
        }
        // Not a valid variable, continue to treat it as raw string
      }
    }

    input.advance();
    matched = true;
  }

  if (matched) {
    input.acceptToken(Raw);
  }
});
