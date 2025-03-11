// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { HeaderParamItem } from "../types/request";
import type { HttpMethod } from "../types/request";
import type { PathParamItem } from "../types/request";
import type { QueryParamItem } from "../types/request";
import type { RequestBody } from "../types/request";

export type CreateCollectionInput = { name: string; path: string; repo?: string };

export type CreateRequestInput = { name: string; url?: string; payload: CreateRequestProtocolSpecificPayload | null };

export type CreateRequestProtocolSpecificPayload = {
  "http": {
    method: HttpMethod;
    query_params: Array<QueryParamItem>;
    path_params: Array<PathParamItem>;
    headers: Array<HeaderParamItem>;
    body: RequestBody | null;
  };
};

export type OverviewCollectionOutput = { name: string; path: string; order?: number };
