import { useActiveWorkspace, useModal, useStreamEnvironments } from "@/hooks";
import { ActionButton } from "@/workbench/ui/components";
import { NewEnvironmentModal } from "@/workbench/ui/components/Modals/Environment/NewEnvironmentModal";

import { SidebarHeader } from "../Sidebar";

export const EnvironmentsListViewHeader = () => {
  const { hasActiveWorkspace } = useActiveWorkspace();

  const {
    showModal: showCreateEnvironmentModal,
    closeModal: closeCreateEnvironmentModal,
    openModal: openCreateEnvironmentModal,
  } = useModal();

  const { clearEnvironmentsCacheAndRefetch } = useStreamEnvironments();

  return (
    <>
      <SidebarHeader
        title="Environments"
        actionsContent={
          <>
            <ActionButton
              disabled={!hasActiveWorkspace}
              icon="Add"
              onClick={openCreateEnvironmentModal}
              hoverVariant="list"
            />
            <ActionButton
              disabled={!hasActiveWorkspace}
              icon="Refresh"
              onClick={clearEnvironmentsCacheAndRefetch}
              hoverVariant="list"
            />
          </>
        }
      />
      {showCreateEnvironmentModal && (
        <NewEnvironmentModal showModal={showCreateEnvironmentModal} closeModal={closeCreateEnvironmentModal} />
      )}
    </>
  );
};
