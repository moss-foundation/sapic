import { JsonValue } from "@repo/moss-bindingutils";
import {
  BatchGetItemByPrefixOutput,
  BatchGetItemOutput,
  BatchPutItemOutput,
  BatchRemoveItemByPrefixOutput,
  BatchRemoveItemOutput,
  GetItemOutput,
  PutItemOutput,
  RemoveItemOutput,
  StorageScope,
} from "@repo/shared-storage";

export interface ISharedStorageIpc {
  getItem: (key: string, scope: StorageScope) => Promise<GetItemOutput>;
  putItem: (key: string, value: JsonValue, scope: StorageScope) => Promise<PutItemOutput>;
  removeItem: (key: string, scope: StorageScope) => Promise<RemoveItemOutput>;

  batchPutItem: (items: Record<string, JsonValue>, scope: StorageScope) => Promise<BatchPutItemOutput>;
  batchRemoveItem: (keys: string[], scope: StorageScope) => Promise<BatchRemoveItemOutput>;
  batchGetItem: (keys: string[], scope: StorageScope) => Promise<BatchGetItemOutput>;
  batchGetItemByPrefix: (prefix: string, scope: StorageScope) => Promise<BatchGetItemByPrefixOutput>;
  batchRemoveItemByPrefix: (prefix: string, scope: StorageScope) => Promise<BatchRemoveItemByPrefixOutput>;
}
