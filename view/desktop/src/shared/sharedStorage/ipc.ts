import { JsonValue } from "@repo/moss-bindingutils";
import { GetItemOutput, PutItemOutput, RemoveItemOutput, StorageScope } from "@repo/shared-storage";

export interface ISharedStorageIpc {
  getItem: (key: string, scope: StorageScope) => Promise<GetItemOutput>;
  putItem: (key: string, value: JsonValue, scope: StorageScope) => Promise<PutItemOutput>;
  removeItem: (key: string, scope: StorageScope) => Promise<RemoveItemOutput>;
}
