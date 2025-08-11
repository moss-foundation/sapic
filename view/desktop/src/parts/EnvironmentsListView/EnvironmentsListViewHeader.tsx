import { ActionButton, SidebarHeader } from "@/components";
import { CreateEnvironmentModal } from "@/components/Modals/Environment/CreateEnvironmentModal";
import { useModal } from "@/hooks";
import { useStreamEnvironments } from "@/hooks/environment";

export const EnvironmentsListViewHeader = () => {
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
            <ActionButton icon="Add" onClick={openCreateEnvironmentModal} />
            <ActionButton icon="Import" />
            <ActionButton icon="Refresh" onClick={clearEnvironmentsCacheAndRefetch} />
          </>
        }
      />
      {showCreateEnvironmentModal && (
        <CreateEnvironmentModal showModal={showCreateEnvironmentModal} closeModal={closeCreateEnvironmentModal} />
      )}
    </>
  );
};
