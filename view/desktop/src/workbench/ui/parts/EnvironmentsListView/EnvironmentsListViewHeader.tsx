import { useModal } from "@/hooks";
import { useStreamEnvironments } from "@/workbench/adapters";
import { ActionButton } from "@/workbench/ui/components";
import { NewEnvironmentModal } from "@/workbench/ui/components/Modals/Environment/NewEnvironmentModal";

import { SidebarHeader } from "../Sidebar";

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
            <ActionButton icon="Add" onClick={openCreateEnvironmentModal} hoverVariant="list" />
            <ActionButton icon="Refresh" onClick={clearEnvironmentsCacheAndRefetch} hoverVariant="list" />
          </>
        }
      />
      {showCreateEnvironmentModal && (
        <NewEnvironmentModal showModal={showCreateEnvironmentModal} closeModal={closeCreateEnvironmentModal} />
      )}
    </>
  );
};
