import { useModal } from "@/hooks";
import { ActionButton } from "@/workbench/ui/components";
import { NewEnvironmentModal } from "@/workbench/ui/components/Modals/Environment/NewEnvironmentModal";
import { SidebarHeader } from "@/workbench/ui/parts/Sidebar/SidebarHeader";

export const EnvironmentsListViewHeader = () => {
  const {
    showModal: showCreateEnvironmentModal,
    closeModal: closeCreateEnvironmentModal,
    openModal: openCreateEnvironmentModal,
  } = useModal();

  //const { refetch: refetchWorkspaceEnvironments } = useStreamEnvironments();
  // const { fetchAllProjectEnvironments } = useGroupedEnvironments();

  const handleRefresh = () => {
    // refetchWorkspaceEnvironments();
    // fetchAllProjectEnvironments();
  };

  return (
    <>
      <SidebarHeader
        toolbar={
          <>
            <ActionButton icon="Add" onClick={openCreateEnvironmentModal} hoverVariant="list" />
            <ActionButton icon="Refresh" onClick={handleRefresh} hoverVariant="list" />
          </>
        }
      />
      {showCreateEnvironmentModal && (
        <NewEnvironmentModal showModal={showCreateEnvironmentModal} closeModal={closeCreateEnvironmentModal} />
      )}
    </>
  );
};
