//FIXME this should be imported from the @repo/template-parser plugin
export type ParsedUrl = {
  schemePart: Array<ParsedValue>;
  hostPart: Array<ParsedValue>;
  pathPart: Array<ParsedValue>;
  queryPart: Array<QueryParam>;
  fragmentPart?: Array<ParsedValue>;
  raw: Array<ParsedValue>;
};

export type ParsedValue = { "string": string } | { "variable": string } | { "pathVariable": string };

export type QueryParam = { key: Array<ParsedValue>; value?: Array<ParsedValue> };
