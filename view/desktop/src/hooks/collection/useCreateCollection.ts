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
      console.log({ data, variables });
      queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return [
          ...old,
          {
            ...variables,
            ...data,
          },
        ];
      });
    },
  });
};

const InputToEvent = (input: CreateCollectionInput): StreamCollectionsEvent => {
  const {
    name,
    order,
    gitParams: { gitHub, gitLab },
  } = input;
  const repository = gitHub.repository || gitLab.repository;
  const branch = gitHub.branch || gitLab.branch;

  return { name, order, repository, branch };
};
