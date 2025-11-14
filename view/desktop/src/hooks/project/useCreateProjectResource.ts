import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { CreateResourceInput, CreateResourceOutput, StreamResourcesEvent } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "./useStreamProjectResources";
import { createProjectResourceForCache } from "./utils";

export interface UseCreateProjectResourceInputProps {
  projectId: string;
  input: CreateResourceInput;
}

const createProjectResource = async ({ projectId, input }: UseCreateProjectResourceInputProps) => {
  const result = await invokeTauriIpc<CreateResourceOutput>("create_project_resource", {
    projectId: projectId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateProjectResource = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateResourceOutput, Error, UseCreateProjectResourceInputProps>({
    mutationFn: createProjectResource,
    onSuccess: async (data, variables) => {
      const newResource = await createProjectResourceForCache(data.id, variables.input);

      queryClient.setQueryData(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: StreamResourcesEvent[]) => {
          return [...old, newResource];
        }
      );
    },
  });
};
