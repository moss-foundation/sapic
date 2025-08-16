import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateCollectionInput, CreateCollectionOutput, StreamCollectionsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

const createCollection = async (input: CreateCollectionInput) => {
  const result = await invokeTauriIpc<CreateCollectionOutput>("create_collection", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateCollection = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createCollection,
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return [
          ...old,
          {
            ...inputToEvent(variables, data),
          },
        ];
      });
    },
  });
};

const inputToEvent = (input: CreateCollectionInput, data: CreateCollectionOutput): StreamCollectionsEvent => {
  const { gitParams, iconPath } = input;

  let repository: string | undefined;
  if (gitParams) {
    if ("gitHub" in gitParams) {
      repository = gitParams.gitHub.repository;
    } else if ("gitLab" in gitParams) {
      repository = gitParams.gitLab.repository;
    }
  }

  return {
    repository,
    contributors: [], // Empty array as default
    iconPath,
    ...data,
  };
};
