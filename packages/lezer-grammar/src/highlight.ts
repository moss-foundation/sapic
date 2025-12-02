import { styleTags, tags as t } from "@lezer/highlight";

export const highlight = styleTags({
  "VarStart": t.variableName, // {{
  "VarEnd": t.variableName, // }}
  "Identifier": t.variableName, // env, test1
  "Slash": t.punctuation, // /
  "PathMarker": t.variableName, // :
  "Raw": t.content, // api-
});
