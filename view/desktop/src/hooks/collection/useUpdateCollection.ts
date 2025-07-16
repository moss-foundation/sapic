import { invokeTauriIpc } from "@/lib/backend/tauri";
import { UpdateEntryOutput } from "@repo/moss-collection";
import { StreamCollectionsEvent, UpdateCollectionInput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

export const updateCollection = async (input: UpdateCollectionInput) => {
  const result = await invokeTauriIpc<UpdateEntryOutput>("update_collection", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateCollection = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateEntryOutput, Error, UpdateCollectionInput>({
    mutationFn: updateCollection,
    onSuccess: (data, variables) => {
      console.log({
        data,
        variables,
      });
      queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return old.map((c) => {
          if (c.id !== variables.id) return c;

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

          const updatedRepository = handleChangeValue(variables.repository, c.repository);
          const updatedIconPath = handleChangeValue(variables.iconPath, c.picturePath);

          return {
            ...c,
            ...variables,
            repository: updatedRepository,
            picturePath: updatedIconPath,
          };
        });
      });
    },
  });
};
