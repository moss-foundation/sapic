import { ActionButton, SidebarHeader } from "@/components";
import { NewEnvironmentModal } from "@/components/Modals/Environment/NewEnvironmentModal";
import { useActiveWorkspace, useModal } from "@/hooks";
import { useStreamEnvironments } from "@/hooks/environment";

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
            <ActionButton disabled={!hasActiveWorkspace} icon="Add" onClick={openCreateEnvironmentModal} />
            <ActionButton disabled={!hasActiveWorkspace} icon="Refresh" onClick={clearEnvironmentsCacheAndRefetch} />
          </>
        }
      />
      {showCreateEnvironmentModal && (
        <NewEnvironmentModal showModal={showCreateEnvironmentModal} closeModal={closeCreateEnvironmentModal} />
      )}
    </>
  );
};
