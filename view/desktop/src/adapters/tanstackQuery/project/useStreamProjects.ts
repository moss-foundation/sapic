import { projectService } from "@/domains/project/projectService";
import { StreamProjectsEvent } from "@repo/ipc";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export const USE_STREAM_PROJECTS_QUERY_KEY = "streamProjects";

const startStreamingProjects = async (): Promise<StreamProjectsEvent[]> => {
  const projects: StreamProjectsEvent[] = [];

  const projectEvent = new Channel<StreamProjectsEvent>();
  projectEvent.onmessage = (project) => {
    projects.push(project);
  };

  await projectService.streamProjects(projectEvent);

  return projects;
};

export const useStreamProjects = () => {
  const queryClient = useQueryClient();

  const query = useQuery<StreamProjectsEvent[], Error>({
    queryKey: [USE_STREAM_PROJECTS_QUERY_KEY],
    queryFn: () => startStreamingProjects(),
    placeholderData: [],
  });

  const clearProjectsCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_PROJECTS_QUERY_KEY] });
  };

  return {
    ...query,
    clearProjectsCacheAndRefetch,
  };
};
