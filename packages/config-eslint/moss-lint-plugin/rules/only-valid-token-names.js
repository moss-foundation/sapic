import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const cssVariables = JSON.parse(readFileSync(join(__dirname, "../css_variables.json"), "utf8"));

const VALID_TOKENS = new Set(cssVariables);

const ANY_TW_SELECTOR_WITH_ARBITRARY_VALUE =
  /\b[\p{L}\p{N}\-:]+(?:\[(?:(?:var\((--[\p{L}\p{N}\-]+)\))|(--[\p{L}\p{N}\-]+))\]|-\((?:(?:var\((--[\p{L}\p{N}\-]+)\))|(--[\p{L}\p{N}\-]+))\))/gu;

const getTokensWithInvalidArbitraryValues = (str, loc) => {
  const invalidTokens = [];
  let arr;

  while ((arr = ANY_TW_SELECTOR_WITH_ARBITRARY_VALUE.exec(str)) !== null) {
    const className = arr[0];
    const name = arr.slice(1).find((item) => item !== undefined);

    const startColumn = loc.start.column + str.indexOf(className) + 1;
    const endColumn = startColumn + className.length;

    if (!VALID_TOKENS.has(name)) {
      invalidTokens.push({
        name,
        loc: {
          start: {
            line: loc.start.line,
            column: startColumn,
          },
          end: {
            line: loc.end.line,
            column: endColumn,
          },
        },
      });
    }
  }

  return invalidTokens;
};

/**  @type {import('eslint').Rule.RuleModule} **/
export default {
  meta: {
    defaultOptions: [],
    type: "problem",
    docs: {
      description: "Validation of token names",
      category: "Invalid syntax",
      recommended: true,
    },
    messages: {
      invalidTokenName: "Invalid token name: {{tokenName}}",
    },
  },
  create(context) {
    return {
      Literal(node) {
        if (typeof node.value === "string") {
          const invalidTokens = getTokensWithInvalidArbitraryValues(node.value, node.loc);

          invalidTokens.forEach((token) => {
            context.report({
              node,
              messageId: "invalidTokenName",
              data: {
                tokenName: token.name,
              },
              loc: token.loc,
            });
          });
        }
      },
      TemplateElement(node) {
        if (typeof node.value.raw === "string") {
          const invalidTokens = getTokensWithInvalidArbitraryValues(node.value.raw, node.loc);

          invalidTokens.forEach((token) => {
            context.report({
              node,
              messageId: "invalidTokenName",
              data: {
                tokenName: token.name,
              },
              loc: token.loc,
            });
          });
        }
      },
    };
  },
};
