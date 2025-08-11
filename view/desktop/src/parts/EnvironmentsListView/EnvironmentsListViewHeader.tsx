import { ActionButton, SidebarHeader } from "@/components";
import { CreateEnvironmentModal } from "@/components/Modals/Environment/CreateEnvironmentModal";
import { useModal } from "@/hooks";

export const EnvironmentsListViewHeader = () => {
  const {
    showModal: showCreateEnvironmentModal,
    closeModal: closeCreateEnvironmentModal,
    openModal: openCreateEnvironmentModal,
  } = useModal();

  return (
    <>
      <SidebarHeader
        title="Environments"
        actionsContent={
          <>
            <ActionButton icon="Add" onClick={openCreateEnvironmentModal} />
            <ActionButton icon="Import" />
            <ActionButton icon="Refresh" />
          </>
        }
      />
      {showCreateEnvironmentModal && (
        <CreateEnvironmentModal showModal={showCreateEnvironmentModal} closeModal={closeCreateEnvironmentModal} />
      )}
    </>
  );
};
