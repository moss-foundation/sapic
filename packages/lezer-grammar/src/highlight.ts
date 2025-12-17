import { styleTags, tags as t } from "@lezer/highlight";

export const highlight = styleTags({
  // Make so that variables and path params are highlighted as a single block
  "Variable": t.variableName,
  "PathParam": t.variableName,
  "VarStart": t.variableName, // {{
  "VarEnd": t.variableName, // }}
  "Identifier": t.variableName, // env, test1
  "Slash": t.punctuation, // /
  "PathMarker": t.variableName, // :
  "Raw": t.content, // api-
});
