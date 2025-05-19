import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ListCollectionsOutput } from "@repo/moss-workspace";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_COLLECTIONS_QUERY_KEY = "listCollections";

const listCollectionsFn = async (workspaceId: string | null): Promise<ListCollectionsOutput> => {
  if (!workspaceId) return [];

  const result = await invokeTauriIpc<ListCollectionsOutput>("list_collections", { workspaceId });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useListCollections = (workspaceId: string | null) => {
  return useQuery<ListCollectionsOutput, Error>({
    queryKey: [USE_LIST_COLLECTIONS_QUERY_KEY, workspaceId],
    queryFn: () => listCollectionsFn(workspaceId),
    enabled: !!workspaceId,
  });
};
