import { useState } from "react";

import { useModal } from "@/hooks";
import { ActionButton, ActionMenu } from "@/workbench/ui/components";
import { CREATE_TAB, IMPORT_TAB } from "@/workbench/ui/components/Modals/Project/NewProjectModal/constants";
import { NewProjectModal } from "@/workbench/ui/components/Modals/Project/NewProjectModal/NewProjectModal";

import { SidebarHeader } from "../../SidebarHeader";
import { useProjectsViewOperations } from "./hooks/useProjectsViewOperations";

export const ProjectTreeViewHeader = () => {
  const { handleRefreshProjectsView, handleCollapseAll, isReloadingProjectsView, everythingIsCollapsed } =
    useProjectsViewOperations();

  const [initialTab, setInitialTab] = useState<typeof CREATE_TAB | typeof IMPORT_TAB>(CREATE_TAB);

  const {
    showModal: showNewProjectModal,
    closeModal: closeNewProjectModal,
    openModal: openNewProjectModal,
  } = useModal();

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
              disabled={everythingIsCollapsed}
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
              onClick={handleRefreshProjectsView}
              title="Refresh Projects"
              disabled={isReloadingProjectsView}
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
