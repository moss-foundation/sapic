import ErrorNaughtyDog from "@/assets/images/ErrorNaughtyDog.svg";
import { useModal } from "@/hooks/useModal";
import { Button } from "@/lib/ui";
import { TabbedPane } from "@/workbench/ui/parts";

import { NewWorkspaceModal } from "./Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "./Modals/Workspace/OpenWorkspaceModal";

interface EmptyWorkspaceProps {
  inSidebar?: boolean;
}

export const EmptyWorkspace = ({ inSidebar = false }: EmptyWorkspaceProps) => {
  const {
    showModal: showNewWorkspaceModal,
    closeModal: closeNewWorkspaceModal,
    openModal: openNewWorkspaceModal,
  } = useModal();

  const {
    showModal: showOpenWorkspaceModal,
    closeModal: closeOpenWorkspaceModal,
    openModal: openOpenWorkspaceModal,
  } = useModal();

  if (inSidebar) {
    return (
      <div className="gap-4.25 flex h-full flex-col px-2">
        {showNewWorkspaceModal && (
          <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
        )}
        {showOpenWorkspaceModal && (
          <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
        )}

        <div>
          <img src={ErrorNaughtyDog} className="pointer-events-none mx-auto h-auto w-full max-w-[200px]" />
          <p className="text-(--moss-secondary-foreground)">
            You need to open a workspace before accessing projects, environments, or sending requests. Please open or
            create a workspace to proceed.
          </p>
        </div>

        <div className="flex flex-col gap-3.5">
          <Button intent="primary" onClick={openNewWorkspaceModal}>
            New workspace
          </Button>
          <Button intent="primary" onClick={openOpenWorkspaceModal}>
            Open workspace
          </Button>
        </div>
      </div>
    );
  }

  // Main content area - render TabbedPane with WelcomePage
  return (
    <>
      {showNewWorkspaceModal && (
        <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      )}
      {showOpenWorkspaceModal && (
        <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      )}
      <TabbedPane />
    </>
  );
};
