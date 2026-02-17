import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";
import {
  BatchGetItemOutput,
  BatchPutItemOutput,
  BatchRemoveItemOutput,
  GetItemOutput,
  PutItemOutput,
  RemoveItemOutput,
} from "@repo/shared-storage";

export async function getItemExpanded(key: string, workspaceId: string): Promise<GetItemOutput> {
  const expandedKey = `${key}.expanded`;
  return await sharedStorageIpc.getItem(expandedKey, { workspace: workspaceId });
}

export async function batchGetItemExpanded(keys: string[], workspaceId: string): Promise<BatchGetItemOutput> {
  const expandedKeys = keys.map((key) => `${key}.expanded`);
  return await sharedStorageIpc.batchGetItem(expandedKeys, { workspace: workspaceId });
}

export async function updateItemExpanded(key: string, expanded: boolean, workspaceId: string): Promise<PutItemOutput> {
  const expandedKey = `${key}.expanded`;
  return await sharedStorageIpc.putItem(expandedKey, expanded, { workspace: workspaceId });
}

export async function batchPutItemExpanded(
  items: Record<string, boolean>,
  workspaceId: string
): Promise<BatchPutItemOutput> {
  const expandedItems = Object.fromEntries(Object.entries(items).map(([key, value]) => [`${key}.expanded`, value]));
  return await sharedStorageIpc.batchPutItem(expandedItems, { workspace: workspaceId });
}

export async function removeItemExpanded(key: string, workspaceId: string): Promise<RemoveItemOutput> {
  const expandedKey = `${key}.expanded`;
  return await sharedStorageIpc.removeItem(expandedKey, { workspace: workspaceId });
}

export async function batchRemoveItemExpanded(keys: string[], workspaceId: string): Promise<BatchRemoveItemOutput> {
  const expandedKeys = keys.map((key) => `${key}.expanded`);
  return await sharedStorageIpc.batchRemoveItem(expandedKeys, { workspace: workspaceId });
}
