// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { EntryClass, EntryKind, EntryProtocol } from "./primitives";

export type ComponentDirConfigurationModel = never;

export type ComponentItemConfigurationModel = never;

export type ConfigurationMetadata = { id: string };

export type DirConfigurationModel =
  | { "request": RequestDirConfigurationModel }
  | { "endpoint": EndpointDirConfigurationModel }
  | { "component": ComponentDirConfigurationModel }
  | { "schema": SchemaDirConfigurationModel };

export type EndpointDirConfigurationModel = never;

export type EndpointItemConfigurationModel = never;

export type EntryInfo = {
  id: string;
  name: string;
  path: string;
  class: EntryClass;
  kind: EntryKind;
  protocol?: EntryProtocol;
  order?: number;
  expanded: boolean;
};

export type EnvironmentInfo = { id: string; name: string; order?: number };

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

export type HttpDirConfigurationModel = Record<string, never>;

export type ItemConfigurationModel =
  | { "request": RequestItemConfigurationModel }
  | { "endpoint": EndpointItemConfigurationModel }
  | { "component": ComponentItemConfigurationModel }
  | { "schema": SchemaItemConfigurationModel };

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

export type RequestDirConfigurationModel = { "http": HttpDirConfigurationModel };

export type RequestItemConfigurationModel = never;

export type SchemaDirConfigurationModel = never;

export type SchemaItemConfigurationModel = never;

export type UrlEncodedItem = {
  key: string;
  value: string;
  order?: number;
  desc?: string;
  disabled: boolean;
  options: UrlEncodedOptions;
};

export type UrlEncodedOptions = { propagate: boolean };
