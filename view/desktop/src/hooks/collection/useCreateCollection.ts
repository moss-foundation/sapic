import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateCollectionInput, CreateCollectionOutput, StreamCollectionsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTIONS_QUERY_KEY } from "./useStreamCollections";

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
      queryClient.setQueryData([USE_STREAM_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
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
  const { iconPath } = input;

  return {
    iconPath,
    archived: false,
    ...data,
  };
};
