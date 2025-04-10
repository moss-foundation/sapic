// This file was generated by misc/importer.py. Do not edit this file manually.
//
// The necessary import statements have been automatically added by a Python script.
// This ensures that all required dependencies are correctly referenced and available
// within this module.
//
// If you need to add or modify imports, please update the imports.json and
// re-run `make gen-models` it to regenerate the file accordingly.

import type { ResourceKey } from "@repo/bindings-utils";

export type FormDataItem = {
  key: string;
  value: FormDataValue;
  order?: number;
  desc?: string;
  disabled: boolean;
  options: FormDataOptions;
};

export type FormDataOptions = { propagate: boolean };

export type FormDataValue = { "text": string } | { "file": string };

export type HeaderParamItem = {
  key: string;
  value: string;
  order?: number;
  desc?: string;
  disabled: boolean;
  options: HeaderParamOptions;
};

export type HeaderParamOptions = { propagate: boolean };

export type HttpMethod = "post" | "get" | "put" | "delete";

export type PathParamItem = {
  key: string;
  value: string;
  order?: number;
  desc?: string;
  disabled: boolean;
  options: PathParamOptions;
};

export type PathParamOptions = { propagate: boolean };

export type QueryParamItem = {
  key: string;
  value: string;
  order?: number;
  desc?: string;
  disabled: boolean;
  options: QueryParamOptions;
};

export type QueryParamOptions = { propagate: boolean };

export type RawBodyType = { "text": string } | { "json": string } | { "html": string } | { "xml": string };

export type RequestBody =
  | { "raw": RawBodyType }
  | { "formData": Array<FormDataItem> }
  | { "urlEncoded": Array<UrlEncodedItem> }
  | { "binary": string };

export type RequestInfo = {
  key: ResourceKey;
  name: string;
  relativePathFromRequestsDir: string;
  order: number | null;
  typ: RequestProtocol;
};

export type RequestProtocol = { "http": HttpMethod } | "webSocket" | "graphQL" | "grpc";

export type UrlEncodedItem = {
  key: string;
  value: string;
  order?: number;
  desc?: string;
  disabled: boolean;
  options: UrlEncodedOptions;
};

export type UrlEncodedOptions = { propagate: boolean };
