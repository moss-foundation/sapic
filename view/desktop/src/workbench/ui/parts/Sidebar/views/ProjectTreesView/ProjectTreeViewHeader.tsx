import { useState } from "react";

import { useClearAllProjectResources } from "@/adapters/tanstackQuery/project";
import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useGetAllLocalResourceSummaries } from "@/db/resourceSummaries/hooks/useGetAllLocalResourceSummaries";
import { useCurrentWorkspace, useModal } from "@/hooks";
import { useBatchPutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchPutTreeItemState";
import { usePutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/usePutTreeItemState";
import { ActionButton, ActionMenu } from "@/workbench/ui/components";
import { CREATE_TAB, IMPORT_TAB } from "@/workbench/ui/components/Modals/Project/NewProjectModal/constants";
import { NewProjectModal } from "@/workbench/ui/components/Modals/Project/NewProjectModal/NewProjectModal";

import { SidebarHeader } from "../../SidebarHeader";

export const ProjectTreeViewHeader = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { isLoading: areProjectsLoading, clearProjectsCacheAndRefetch } = useListProjects();

  const { clearAllProjectResourcesCache } = useClearAllProjectResources();

  const { mutateAsync: putTreeItemState } = usePutTreeItemState();
  const { mutateAsync: batchPutTreeItemState } = useBatchPutTreeItemState();

  const projectSummaries = useGetAllLocalProjectSummaries();
  const resourceSummaries = useGetAllLocalResourceSummaries();

  const [initialTab, setInitialTab] = useState<typeof CREATE_TAB | typeof IMPORT_TAB>(CREATE_TAB);

  //TODO project and resource summaries that is linked to manipulating all states is broken for now
  //until all the resources and projects summaries start using state from shared storage
  const areAllProjectsCollapsed = resourceSummaries?.every((p) => !p.expanded);
  const areAllDirNodesCollapsed = resourceSummaries?.every(() => {
    return resourceSummaries?.filter((resource) => resource.kind === "Dir").every((resource) => !resource.expanded);
  });

  const {
    showModal: showNewProjectModal,
    closeModal: closeNewProjectModal,
    openModal: openNewProjectModal,
  } = useModal();

  const handleRefreshProjects = () => {
    clearProjectsCacheAndRefetch();
    clearAllProjectResourcesCache();
  };

  const handleCollapseAll = async () => {
    await collapseExpandedProjects();
    await collapseExpandedDirResources();
  };

  const collapseExpandedProjects = async () => {
    const openedProjectSummaries = projectSummaries?.filter((p) => p.expanded);

    if (openedProjectSummaries.length === 0) return;

    if (openedProjectSummaries.length === 1) {
      await putTreeItemState({
        treeItemState: {
          id: openedProjectSummaries[0].id,
          expanded: false,
          order: openedProjectSummaries[0].order,
        },
        workspaceId: currentWorkspaceId,
      });
    } else {
      await batchPutTreeItemState({
        treeItemStates: openedProjectSummaries.map((p) => ({
          id: p.id,
          expanded: false,
          order: p.order,
        })),
        workspaceId: currentWorkspaceId,
      });
    }
  };

  const collapseExpandedDirResources = async () => {
    const expandedDirResources = resourceSummaries?.filter((resource) => resource.kind === "Dir" && resource.expanded);

    if (expandedDirResources.length === 0) return;

    if (expandedDirResources.length === 1) {
      await putTreeItemState({
        treeItemState: {
          id: expandedDirResources[0].id,
          expanded: false,
          order: expandedDirResources[0].order ?? undefined,
        },
        workspaceId: currentWorkspaceId,
      });
    } else {
      await batchPutTreeItemState({
        treeItemStates: expandedDirResources.map((resource) => ({
          id: resource.id,
          expanded: false,
          order: resource.order ?? undefined,
        })),
        workspaceId: currentWorkspaceId,
      });
    }
  };

  return (
    <>
      <SidebarHeader
        toolbar={
          <>
            <ActionButton
              title="Add Project"
              icon="Add"
              onClick={() => {
                setInitialTab(CREATE_TAB);
                openNewProjectModal();
              }}
            />
            <ActionButton
              title="Collapse all Projects"
              disabled={areAllDirNodesCollapsed && areAllProjectsCollapsed}
              icon="CollapseAll"
              onClick={handleCollapseAll}
            />
            <ActionButton
              title="Import Project"
              icon="Import"
              onClick={() => {
                setInitialTab(IMPORT_TAB);
                openNewProjectModal();
              }}
            />
            <ActionButton
              icon="Refresh"
              onClick={handleRefreshProjects}
              title="Refresh Projects"
              disabled={areProjectsLoading}
            />

            <PlaceholderDropdownMenu />
          </>
        }
      />
      {showNewProjectModal && (
        <NewProjectModal initialTab={initialTab} showModal={showNewProjectModal} closeModal={closeNewProjectModal} />
      )}
    </>
  );
};

const PlaceholderDropdownMenu = () => {
  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <ActionButton icon="MoreHorizontal" />
      </ActionMenu.Trigger>

      <ActionMenu.Portal>
        <ActionMenu.Content align="center">
          <ActionMenu.Item onSelect={() => console.log("Item 1 selected")}>Placeholder Item 1</ActionMenu.Item>
          <ActionMenu.Item onSelect={() => console.log("Item 2 selected")}>Placeholder Item 2</ActionMenu.Item>
          <ActionMenu.Item onSelect={() => console.log("Item 3 selected")}>Placeholder Item 3</ActionMenu.Item>
        </ActionMenu.Content>
      </ActionMenu.Portal>
    </ActionMenu.Root>
  );
};
