import { useState } from "react";

import { useListWorkspaceEnvironments } from "@/adapters";
import { refreshProjectSummaries } from "@/db/projectSummaries/actions/refreshProjectSummaries";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useGetAllLocalResourceSummaries } from "@/db/resourceSummaries/hooks/useGetAllLocalResourceSummaries";
import { useCurrentWorkspace, useModal } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { ActionButton, ActionMenu } from "@/workbench/ui/components";
import { CREATE_TAB, IMPORT_TAB } from "@/workbench/ui/components/Modals/Project/NewProjectModal/constants";
import { NewProjectModal } from "@/workbench/ui/components/Modals/Project/NewProjectModal/NewProjectModal";

import { SidebarHeader } from "../../SidebarHeader";

export const ProjectTreeViewHeader = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projectSummaries, isLoading: areProjectsLoading } = useGetAllLocalProjectSummaries();
  const { data: resourceSummaries } = useGetAllLocalResourceSummaries();
  const { flushWorkspaceEnvironments } = useListWorkspaceEnvironments();
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

  const handleRefreshProjects = async () => {
    flushWorkspaceEnvironments();
    await refreshProjectSummaries({ currentWorkspaceId });
  };

  const handleCollapseAll = async () => {
    await collapseExpandedProjects();
    await collapseExpandedDirResources();
  };

  const collapseExpandedProjects = async () => {
    const openedProjectSummaries = projectSummaries?.filter((p) => p.expanded);

    if (openedProjectSummaries.length === 0) return;

    if (openedProjectSummaries.length === 1) {
      treeItemStateService.putExpanded(openedProjectSummaries[0].id, false, currentWorkspaceId);
    } else {
      treeItemStateService.batchPutExpanded(
        Object.fromEntries(openedProjectSummaries.map((p) => [p.id, false])),
        currentWorkspaceId
      );
    }
  };

  const collapseExpandedDirResources = async () => {
    const expandedDirResources = resourceSummaries?.filter((resource) => resource.kind === "Dir" && resource.expanded);

    if (expandedDirResources.length === 0) return;

    if (expandedDirResources.length === 1) {
      treeItemStateService.putExpanded(expandedDirResources[0].id, false, currentWorkspaceId);
    } else {
      treeItemStateService.batchPutExpanded(
        Object.fromEntries(expandedDirResources.map((resource) => [resource.id, false])),
        currentWorkspaceId
      );
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
