import { JsonValue } from "@repo/moss-bindingutils";
import {
  GetItemInput,
  GetItemOutput,
  PutItemInput,
  PutItemOutput,
  RemoveItemInput,
  RemoveItemOutput,
} from "@repo/shared-storage";

import { invokeTauriServiceIpc } from "../../../infra/ipc/tauri";

export const sharedStorageService = {
  getItem: async (key: string, workspaceId?: string): Promise<GetItemOutput> => {
    const res = await invokeTauriServiceIpc<GetItemInput, GetItemOutput>({
      cmd: "plugin:shared-storage|get_item",
      args: {
        input: {
          key,
          scope: workspaceId ? { workspace: workspaceId } : "application",
        },
      },
    });

    if (res.status === "error") {
      console.error(`Failed to get key "${key}" from workspace "${workspaceId ?? "application"}"`);
      return {
        key,
        value: null,
        scope: workspaceId ? { workspace: workspaceId } : "application",
      };
    }

    return res.data;
  },
  putItem: async (key: string, value: JsonValue, workspaceId?: string) => {
    const res = await invokeTauriServiceIpc<PutItemInput, PutItemOutput>({
      cmd: "plugin:shared-storage|put_item",
      args: {
        input: {
          key,
          value,
          scope: workspaceId ? { workspace: workspaceId } : "application",
        },
      },
    });

    if (res.status === "error") {
      console.error(`Failed to save key "${key}" with value "${value}" in workspace "${workspaceId ?? "application"}"`);
    }
  },
  removeItem: async (key: string, workspaceId?: string): Promise<RemoveItemOutput> => {
    const res = await invokeTauriServiceIpc<RemoveItemInput, RemoveItemOutput>({
      cmd: "plugin:shared-storage|remove_item",
      args: {
        input: {
          key,
          scope: workspaceId ? { workspace: workspaceId } : "application",
        },
      },
    });

    if (res.status === "error") {
      console.error(`Failed to remove key "${key}" from workspace "${workspaceId ?? "application"}"`);
      return {
        value: null,
        scope: workspaceId ? { workspace: workspaceId } : "application",
      };
    }

    return res.data;
  },
};
