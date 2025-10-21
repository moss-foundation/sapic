import { useState } from "react";

import { ActionButton, ActionMenu, SidebarHeader } from "@/components";
import { CREATE_TAB, IMPORT_TAB } from "@/components/Modals/Project/NewProjectModal/constansts";
import { NewProjectModal } from "@/components/Modals/Project/NewProjectModal/NewProjectModal";
import {
  USE_STREAM_PROJECT_RESOURCES_QUERY_KEY,
  useActiveWorkspace,
  useClearAllProjectResources,
  useModal,
  useStreamedProjectsWithResources,
  useStreamProjects,
} from "@/hooks";
import { useBatchUpdateProject } from "@/hooks/project/useBatchUpdateProject";
import { useBatchUpdateProjectResource } from "@/hooks/project/useBatchUpdateProjectResource";
import { StreamResourcesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

export const ProjectTreeViewHeader = () => {
  const queryClient = useQueryClient();

  const { isLoading: areProjectsLoading, clearProjectsCacheAndRefetch } = useStreamProjects();
  const { clearAllProjectResourcesCache } = useClearAllProjectResources();
  const { data: projectsWithResources } = useStreamedProjectsWithResources();
  const { mutateAsync: batchUpdateProject } = useBatchUpdateProject();
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();
  const { hasActiveWorkspace } = useActiveWorkspace();

  const [initialTab, setInitialTab] = useState<typeof CREATE_TAB | typeof IMPORT_TAB>(CREATE_TAB);

  const {
    showModal: showNewProjectModal,
    closeModal: closeNewProjectModal,
    openModal: openNewProjectModal,
  } = useModal();

  const handleRefreshProjects = () => {
    clearProjectsCacheAndRefetch();
    clearAllProjectResourcesCache();
  };

  const areAllProjectsCollapsed = projectsWithResources.every((p) => !p.expanded);
  const areAllDirNodesCollapsed = projectsWithResources.every((p) => {
    return p.resources.filter((resource) => resource.kind === "Dir").every((resource) => !resource.expanded);
  });

  const handleCollapseAll = async () => {
    await collapseExpandedProjects();
    await collapseExpandedDirResources();
  };

  const collapseExpandedProjects = async () => {
    const openedProjects = projectsWithResources.filter((p) => p.expanded);

    if (openedProjects.length === 0) return;

    await batchUpdateProject({
      items: openedProjects.map((p) => ({
        id: p.id,
        expanded: false,
      })),
    });
  };

  const collapseExpandedDirResources = async () => {
    const projectsWithExpandedDirs = projectsWithResources
      .map((p) => ({
        projectId: p.id,
        resources: p.resources.filter((resource) => resource.kind === "Dir" && resource.expanded),
      }))
      .filter((p) => p.resources.length > 0);

    if (projectsWithExpandedDirs.length === 0) return;

    const promises = projectsWithExpandedDirs.map(async (p) => {
      const preparedResources = p.resources.map((resource) => ({
        DIR: {
          id: resource.id,
          expanded: false,
        },
      }));

      const res = await batchUpdateProjectResource({
        projectId: p.projectId,
        resources: {
          resources: preparedResources,
        },
      });

      if (res.status === "ok") {
        queryClient.setQueryData(
          [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, p.projectId],
          (old: StreamResourcesEvent[]) => {
            return old.map((resource) => {
              const shouldCollapse = preparedResources.some(
                (preparedResource) => preparedResource.DIR.id === resource.id
              );
              return shouldCollapse ? { ...resource, expanded: false } : resource;
            });
          }
        );
      }
    });

    await Promise.all(promises);
  };

  return (
    <>
      <SidebarHeader
        title="Projects"
        actionsContent={
          <>
            <ActionButton
              title="Add Project"
              disabled={!hasActiveWorkspace}
              icon="Add"
              onClick={() => {
                setInitialTab(CREATE_TAB);
                openNewProjectModal();
              }}
            />
            <ActionButton
              title="Collapse all Projects"
              disabled={!hasActiveWorkspace || (areAllDirNodesCollapsed && areAllProjectsCollapsed)}
              icon="CollapseAll"
              onClick={handleCollapseAll}
            />
            <ActionButton
              title="Import Project"
              disabled={!hasActiveWorkspace}
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
              disabled={areProjectsLoading || !hasActiveWorkspace}
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
