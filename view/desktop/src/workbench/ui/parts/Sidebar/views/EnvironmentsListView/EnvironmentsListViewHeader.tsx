import { useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { useModal } from "@/hooks";
import { ActionButton } from "@/workbench/ui/components";
import { NewEnvironmentModal } from "@/workbench/ui/components/Modals/Environment/NewEnvironmentModal";

import { SidebarHeader } from "../../SidebarHeader";

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
        toolbar={
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
