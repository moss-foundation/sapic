import { useEffect, useRef } from "react";

import { useStreamProjects } from "@/adapters";
import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchGetTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchGetTreeItemState";

export const useSyncProjectSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const hasSyncedRef = useRef(false);
  const lastWorkspaceIdRef = useRef<string | undefined>(currentWorkspaceId);

  const { data: projects, isLoading } = useStreamProjects();
  const { data: treeItemStates } = useBatchGetTreeItemState(
    projects?.map((project) => project.id) ?? [],
    currentWorkspaceId
  );

  // Reset sync flag when workspace changes
  useEffect(() => {
    if (lastWorkspaceIdRef.current !== currentWorkspaceId) {
      lastWorkspaceIdRef.current = currentWorkspaceId;
      hasSyncedRef.current = false;
    }
  }, [currentWorkspaceId]);

  useEffect(() => {
    // Only sync on initial load when data is available
    if (hasSyncedRef.current || !projects || projects.length === 0 || !treeItemStates) {
      return;
    }

    const updateLocalProjects = async () => {
      for (const project of projects) {
        const treeItemState = treeItemStates.find((treeItemState) => treeItemState.id === project.id);

        if (projectSummariesCollection.has(project.id)) {
          projectSummariesCollection.update(project.id, (draft) => {
            draft.order = treeItemState?.order ?? -1;
            draft.expanded = treeItemState?.expanded ?? true;
          });
        } else {
          projectSummariesCollection.insert({
            ...project,
            order: treeItemState?.order ?? -1,
            expanded: treeItemState?.expanded ?? true,
          });
        }
      }
      hasSyncedRef.current = true;
    };

    updateLocalProjects();
  }, [currentWorkspaceId, projects, treeItemStates]);

  return { isLoading };
};
