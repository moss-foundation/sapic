// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type HeaderItem = {
  key: string;
  value: string;
  order?: number;
  desc?: string;
  disabled: boolean;
  options: HeaderOptions;
};

export type HeaderOptions = { propagate: boolean };

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
