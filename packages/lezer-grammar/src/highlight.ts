import { styleTags, tags as t } from "@lezer/highlight";

export const highlight = styleTags({
  "VarStart": t.punctuation, // {{
  "VarEnd": t.punctuation, // }}
  "Identifier": t.variableName, // env, test1
  "Slash": t.punctuation, // /
  "PathMarker": t.punctuation, // :
  "Raw": t.content, // api-
});
