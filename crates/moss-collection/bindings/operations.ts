// This file was generated by misc/importer.py. Do not edit this file manually.
//
// The necessary import statements have been automatically added by a Python script.
// This ensures that all required dependencies are correctly referenced and available
// within this module.
//
// If you need to add or modify imports, please update the imports.json and
// re-run `make gen-models` it to regenerate the file accordingly.

import type { HeaderParamItem } from "./types";
import type { HttpMethod } from "./types";
import type { PathParamItem } from "./types";
import type { QueryParamItem } from "./types";
import type { RequestBody } from "./types";
import type { RequestInfo } from "./types";
import type { ResourceKey } from "@repo/bindings-utils";

export type CreateRequestInput = {
  name: string;
  relativePath?: string;
  url?: string;
  payload?: CreateRequestProtocolSpecificPayload;
};

export type CreateRequestOutput = { key: ResourceKey };

export type CreateRequestProtocolSpecificPayload = {
  "http": {
    method: HttpMethod;
    query_params: Array<QueryParamItem>;
    path_params: Array<PathParamItem>;
    headers: Array<HeaderParamItem>;
    body: RequestBody | null;
  };
};

export type DeleteRequestInput = { key: ResourceKey };

export type ListRequestsOutput = Array<RequestInfo>;

export type RenameRequestInput = { key: ResourceKey; newName: string };
