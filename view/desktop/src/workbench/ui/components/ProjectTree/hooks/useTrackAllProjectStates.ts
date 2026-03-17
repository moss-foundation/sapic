import { useCurrentWorkspace } from "@/hooks";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";

import { checkIfTreeIsFullyCollapsed } from "../TreeRoot/validation/checkIfTreeIsFullyCollapsed";
import { checkIfTreeIsFullyExpanded } from "../TreeRoot/validation/checkIfTreeIsFullyExpanded";
import { ProjectTreeRoot } from "../types";

export const useTrackAllProjectStates = (tree: ProjectTreeRoot) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: resourcesListExpanded = false } = useGetResourcesListItemState(tree.id, currentWorkspaceId);
  const { data: environmentsListExpanded = false } = useGetEnvironmentListItemState(tree.id, currentWorkspaceId);

  const isFullyExpanded = checkIfTreeIsFullyExpanded({
    tree,
    resourcesListExpanded,
    environmentsListExpanded,
  });

  const isFullyCollapsed = checkIfTreeIsFullyCollapsed({
    tree,
    resourcesListExpanded,
    environmentsListExpanded,
  });

  return { isFullyExpanded, isFullyCollapsed };
};
