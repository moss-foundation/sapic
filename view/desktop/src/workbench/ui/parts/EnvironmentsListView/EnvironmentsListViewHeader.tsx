import { useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { useModal } from "@/hooks";
import { ActionButton } from "@/workbench/ui/components";
import { NewEnvironmentModal } from "@/workbench/ui/components/Modals/Environment/NewEnvironmentModal";

import { useGroupedEnvironments } from "../../components/EnvironmentsLists/hooks/useGroupedEnvironments";
import { SidebarHeader } from "../Sidebar";

export const EnvironmentsListViewHeader = () => {
  const {
    showModal: showCreateEnvironmentModal,
    closeModal: closeCreateEnvironmentModal,
    openModal: openCreateEnvironmentModal,
  } = useModal();

  const { refetch: refetchWorkspaceEnvironments } = useStreamEnvironments();
  const { fetchAllProjectEnvironments } = useGroupedEnvironments();

  const handleRefresh = () => {
    refetchWorkspaceEnvironments();
    fetchAllProjectEnvironments();
  };

  return (
    <>
      <SidebarHeader
        title="Environments"
        actionsContent={
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
