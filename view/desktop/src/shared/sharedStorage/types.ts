import type { StorageScope } from "@repo/shared-storage";

export const SharedStorageScopeEnum = {
  APPLICATION: "application" as const satisfies StorageScope,
  Workspace: (id: string) => ({ workspace: id }) as const satisfies StorageScope,
} as const;

const _ensureUserForSharedStorageScope: "application" extends StorageScope
  ? typeof SharedStorageScopeEnum.APPLICATION
  : never = SharedStorageScopeEnum.APPLICATION;
const _ensureWorkspaceForSharedStorageScope: { workspace: string } extends StorageScope
  ? ReturnType<typeof SharedStorageScopeEnum.Workspace>
  : never = SharedStorageScopeEnum.Workspace("test");
void _ensureUserForSharedStorageScope;
void _ensureWorkspaceForSharedStorageScope;
