import { useMemo } from "react";

import { USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY, USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY } from "@/adapters";
import { USE_LIST_PROJECTS_QUERY_KEY } from "@/adapters/tanstackQuery/project";
import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "@/adapters/tanstackQuery/resource";
import { flushEnvironmentSummaries } from "@/db/environmentsSummaries/actions/flushEnvironmentSummaries";
import { flushProjectSummaries } from "@/db/projectSummaries/actions/flushProjectSummaries";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { flushResourceSummaries } from "@/db/resourceSummaries/actions/flushResourceSummaries";
import { useGetAllLocalResourceSummaries } from "@/db/resourceSummaries/hooks/useGetAllLocalResourceSummaries";
import { useCurrentWorkspace } from "@/hooks";
import {
  USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY,
  useBatchGetEnvironmentListItemState,
  useBatchPutEnvironmentListItemState,
  useGetEnvironmentListItemState,
  useGetProjectListState,
  usePutEnvironmentListItemState,
  usePutProjectListState,
} from "@/workbench/adapters";
import {
  USE_BATCH_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY,
  useBatchGetResourcesListItemState,
} from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useBatchGetResourcesListItemState";
import { useBatchPutResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useBatchPutResourcesListItemState";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { WORKSPACE_ENVIRONMENTS_LIST_ID } from "@/workbench/ui/components/EnvironmentsLists/constants";
import { useIsFetching, useQueryClient } from "@tanstack/react-query";

export const useProjectsViewOperations = () => {
  const queryClient = useQueryClient();
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { isLoading: isReloadingProjectsView } = useProjectsViewFetchingTracking();

  // 1. DATA FETCHING (Queries)
  const { data: workspaceEnvironmentListExpanded } = useGetEnvironmentListItemState(
    WORKSPACE_ENVIRONMENTS_LIST_ID,
    currentWorkspaceId
  );
  const { data: projectListExpanded } = useGetProjectListState(currentWorkspaceId);
  const { data: projectSummaries } = useGetAllLocalProjectSummaries();
  const { data: resourceSummaries } = useGetAllLocalResourceSummaries();

  // 2. DERIVED DATA & BATCH QUERIES
  const projectIds = useMemo(() => projectSummaries?.map((p) => p.id) ?? [], [projectSummaries]);
  const resourceIds = useMemo(() => resourceSummaries?.map((r) => r.id) ?? [], [resourceSummaries]);

  const { data: projectEnvironmentListExpanded } = useBatchGetEnvironmentListItemState(projectIds, currentWorkspaceId);
  const { data: resourcesListExpanded } = useBatchGetResourcesListItemState(resourceIds, currentWorkspaceId);

  // 3. MUTATIONS
  const { mutate: updateProjectListState } = usePutProjectListState();
  const { mutate: updateEnvironmentListItemState } = usePutEnvironmentListItemState();
  const { mutate: batchPutEnvironmentListItemState } = useBatchPutEnvironmentListItemState();
  const { mutate: batchPutResourcesListItemState } = useBatchPutResourcesListItemState();

  // 4. COMPUTED UI STATES
  const areAllProjectsCollapsed = projectSummaries?.every((p) => !p.expanded) ?? true;
  const areAllDirNodesCollapsed =
    resourceSummaries?.filter((resource) => resource.kind === "Dir").every((resource) => !resource.expanded) ?? true;

  const everythingIsCollapsed = useMemo(() => {
    return (
      workspaceEnvironmentListExpanded === false &&
      projectListExpanded === false &&
      areAllProjectsCollapsed &&
      (projectEnvironmentListExpanded?.every((p) => p === false) ?? true) &&
      (resourcesListExpanded?.every((r) => r === false) ?? true) &&
      areAllDirNodesCollapsed
    );
  }, [
    workspaceEnvironmentListExpanded,
    projectListExpanded,
    areAllProjectsCollapsed,
    projectEnvironmentListExpanded,
    resourcesListExpanded,
    areAllDirNodesCollapsed,
  ]);

  // 5. HELPER ACTIONS
  const collapseTreeItems = async (items) => {
    if (!items || items.length === 0) return;

    if (items.length === 1) {
      treeItemStateService.putExpanded(items[0].id, false, currentWorkspaceId);
    } else {
      const entries = Object.fromEntries(items.map((item) => [item.id, false]));
      treeItemStateService.batchPutExpanded(entries, currentWorkspaceId);
    }
  };

  // 6. EVENT HANDLERS
  const handleRefreshProjectsView = async () => {
    flushEnvironmentSummaries();
    queryClient.resetQueries({ queryKey: [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY] });
    queryClient.resetQueries({ queryKey: [USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY] });

    flushProjectSummaries();
    queryClient.resetQueries({ queryKey: [USE_LIST_PROJECTS_QUERY_KEY] });

    flushResourceSummaries();
    queryClient.resetQueries({ queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY] });
  };

  const handleCollapseAll = async () => {
    // Awaiting in parallel is usually faster if these don't depend on each other
    await Promise.all([
      collapseTreeItems(projectSummaries?.filter((p) => p.expanded)),
      collapseTreeItems(resourceSummaries?.filter((r) => r.kind === "Dir" && r.expanded)),

      batchPutEnvironmentListItemState({
        environmentListItemStates: Object.fromEntries(projectIds.map((id) => [id, false])),
        workspaceId: currentWorkspaceId,
      }),

      updateEnvironmentListItemState({
        id: WORKSPACE_ENVIRONMENTS_LIST_ID,
        expanded: false,
        workspaceId: currentWorkspaceId,
      }),

      updateProjectListState({ expanded: false, workspaceId: currentWorkspaceId }),

      batchPutResourcesListItemState({
        resourcesListItemStates: Object.fromEntries(projectIds.map((id) => [id, false])),
        workspaceId: currentWorkspaceId,
      }),
    ]);
  };

  return {
    handleRefreshProjectsView,
    handleCollapseAll,
    isReloadingProjectsView,
    areAllProjectsCollapsed,
    areAllDirNodesCollapsed,
    everythingIsCollapsed,
  };
};

const useProjectsViewFetchingTracking = () => {
  const projectsFetchingCount = useIsFetching({
    queryKey: [USE_LIST_PROJECTS_QUERY_KEY],
  });
  const workspaceEnvironmentsFetchingCount = useIsFetching({
    queryKey: [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY],
  });
  const projectEnvironmentsFetchingCount = useIsFetching({
    queryKey: [USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY],
  });
  const projectResourcesFetchingCount = useIsFetching({
    queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY],
  });
  const resourcesListExpandedFetchingCount = useIsFetching({
    queryKey: [USE_BATCH_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY],
  });
  const environmentListExpandedFetchingCount = useIsFetching({
    queryKey: [USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY],
  });

  const isLoading =
    projectsFetchingCount > 0 ||
    workspaceEnvironmentsFetchingCount > 0 ||
    projectEnvironmentsFetchingCount > 0 ||
    projectResourcesFetchingCount > 0 ||
    resourcesListExpandedFetchingCount > 0 ||
    environmentListExpandedFetchingCount > 0;

  return {
    isLoading,
  };
};
