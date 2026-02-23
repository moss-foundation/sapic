import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";
import {
  BatchGetItemOutput,
  BatchPutItemOutput,
  BatchRemoveItemOutput,
  GetItemOutput,
  PutItemOutput,
  RemoveItemOutput,
  StorageScope,
} from "@repo/shared-storage";

const toScope = (workspaceId?: string): StorageScope => (workspaceId ? { workspace: workspaceId } : "application");

export async function getItemOrder(key: string, workspaceId?: string): Promise<GetItemOutput> {
  const orderKey = `${key}.order`;
  const output = await sharedStorageIpc.getItem(orderKey, toScope(workspaceId));
  return {
    key: key,
    value: output.value,
    scope: output.scope,
  };
}

export async function batchGetItemOrder(keys: string[], workspaceId?: string): Promise<BatchGetItemOutput> {
  const orderKeys = keys.map((key) => `${key}.order`);
  return await sharedStorageIpc.batchGetItem(orderKeys, toScope(workspaceId));
}

export async function putItemOrder(key: string, order: number, workspaceId?: string): Promise<PutItemOutput> {
  const orderKey = `${key}.order`;
  return await sharedStorageIpc.putItem(orderKey, order, toScope(workspaceId));
}

export async function batchPutItemOrder(
  items: Record<string, number>,
  workspaceId?: string
): Promise<BatchPutItemOutput> {
  const orderItems = Object.fromEntries(Object.entries(items).map(([key, value]) => [`${key}.order`, value]));
  return await sharedStorageIpc.batchPutItem(orderItems, toScope(workspaceId));
}

export async function removeItemOrder(key: string, workspaceId?: string): Promise<RemoveItemOutput> {
  const orderKey = `${key}.order`;
  return await sharedStorageIpc.removeItem(orderKey, toScope(workspaceId));
}

export async function batchRemoveItemOrder(keys: string[], workspaceId?: string): Promise<BatchRemoveItemOutput> {
  const orderKeys = keys.map((key) => `${key}.order`);
  return await sharedStorageIpc.batchRemoveItem(orderKeys, toScope(workspaceId));
}
