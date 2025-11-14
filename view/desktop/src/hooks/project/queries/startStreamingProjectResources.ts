import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { StreamResourcesEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

export const startStreamingProjectResources = async (
  projectId: string,
  path?: string
): Promise<StreamResourcesEvent[]> => {
  const resources: StreamResourcesEvent[] = [];
  const onProjectResourceEvent = new Channel<StreamResourcesEvent>();

  onProjectResourceEvent.onmessage = (projectResource) => {
    resources.push(projectResource);
  };

  const result = await invokeTauriIpc("stream_project_resources", {
    projectId,
    channel: onProjectResourceEvent,
    input: path ? { "RELOAD_PATH": path } : "LOAD_ROOT",
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return resources;
};
