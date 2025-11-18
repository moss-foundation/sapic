import { projectIpc } from "@/infra/ipc/project";
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

  await projectIpc.streamProjectResources(projectId, onProjectResourceEvent, path);

  return resources;
};
