import { useEffect } from "react";

import { useStreamProjects } from "@/adapters";
import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchGetTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchGetTreeItemState";

export const useSyncProjectSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projects, isLoading } = useStreamProjects();
  const { data: treeItemStates } = useBatchGetTreeItemState(
    projects?.map((project) => project.id) ?? [],
    currentWorkspaceId
  );

  useEffect(() => {
    const updateLocalProjects = async () => {
      for (const project of projects ?? []) {
        const treeItemState = treeItemStates?.find((treeItemState) => treeItemState.id === project.id);

        if (projectSummariesCollection.has(project.id)) {
          projectSummariesCollection.update(project.id, (draft) => {
            Object.assign(draft, {
              ...draft,
              ...project,
              order: treeItemState?.order ?? 0,
              expanded: treeItemState?.expanded ?? true,
            });
          });
        } else {
          projectSummariesCollection.insert({
            ...project,
            order: treeItemState?.order ?? 0,
            expanded: treeItemState?.expanded ?? true,
          });
        }
      }
    };

    if (projects && projects.length > 0) {
      updateLocalProjects();
    }
  }, [currentWorkspaceId, projects, treeItemStates]);

  return { isLoading };
};
