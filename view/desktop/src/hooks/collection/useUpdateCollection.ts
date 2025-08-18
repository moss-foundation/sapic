import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamCollectionsEvent, UpdateCollectionInput, UpdateCollectionOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

export const updateCollection = async (input: UpdateCollectionInput) => {
  const result = await invokeTauriIpc<UpdateCollectionOutput>("update_collection", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateCollection = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateCollectionOutput, Error, UpdateCollectionInput>({
    mutationFn: updateCollection,
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return old.map((oldCollection) => {
          if (oldCollection.id !== data.id) return oldCollection;

          const handleChangeValue = <T>(
            changeValue: { "UPDATE": T } | "REMOVE" | undefined,
            currentValue: T | undefined
          ): T | undefined => {
            if (changeValue === undefined) {
              return currentValue;
            }
            if (changeValue === "REMOVE") {
              return undefined;
            }
            if (typeof changeValue === "object" && "UPDATE" in changeValue) {
              return changeValue.UPDATE;
            }
            return currentValue;
          };

          const updatedRepository = handleChangeValue(variables.repository, oldCollection.repository);
          const updatedIconPath = handleChangeValue(variables.iconPath, oldCollection.iconPath);

          return {
            ...oldCollection,
            ...variables,
            repository: updatedRepository,
            iconPath: updatedIconPath,
          };
        });
      });
    },
  });
};
