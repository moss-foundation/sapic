import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { StreamProjectsEvent } from "@repo/moss-workspace";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { useActiveWorkspace } from "../workspace";

export const USE_STREAM_PROJECTS_QUERY_KEY = "streamProjects";

const startStreamProjects = async (): Promise<StreamProjectsEvent[]> => {
  const projects: StreamProjectsEvent[] = [];

  const onProjectEvent = new Channel<StreamProjectsEvent>();

  onProjectEvent.onmessage = (project) => {
    projects.push(project);
  };

  await invokeTauriIpc("stream_projects", {
    channel: onProjectEvent,
  });

  return projects;
};

export const useStreamProjects = () => {
  const queryClient = useQueryClient();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamProjectsEvent[], Error>({
    queryKey: [USE_STREAM_PROJECTS_QUERY_KEY],
    queryFn: async (): Promise<StreamProjectsEvent[]> => {
      const projects = await startStreamProjects();
      return projects;
    },
    placeholderData: [],
    enabled: hasActiveWorkspace,
  });

  const clearProjectsCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_PROJECTS_QUERY_KEY] });
  };

  return {
    ...query,
    clearProjectsCacheAndRefetch,
  };
};
