import { useEffect } from "react";

import { useWorkspaceState } from "@/hooks/appState/useWorkspaceState";
import { useModal } from "@/hooks/useModal";
import { useGetViewGroup } from "@/hooks/viewGroups/useGetViewGroup";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import ErrorNaughtyDog from "../assets/images/ErrorNaughtyDog.svg";
import ButtonPrimary from "./ButtonPrimary";
import CollectionTreeView from "./CollectionTreeView";
import { NewWorkspaceModal } from "./Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "./Modals/Workspace/OpenWorkspaceModal";

export const ViewContainer = ({ groupId }: { groupId: string }) => {
  const { data: viewGroup } = useGetViewGroup(groupId);
  const { state: workspaceState } = useWorkspaceState();
  const { api } = useTabbedPaneStore();

  useEffect(() => {
    // Only close the welcome panel when we have a workspace
    if (workspaceState === "inWorkspace") {
      const WelcomePanel = api?.getPanel("WelcomePage");
      if (WelcomePanel) {
        WelcomePanel.api.close();
      }
    }
  }, [workspaceState, api]);

  // Always show the NoWorkspaceComponent in sidebars if we're in empty state
  if (workspaceState === "empty") {
    return (
      <div className="flex h-full flex-col">
        <NoWorkspaceComponent />
      </div>
    );
  }

  if (!viewGroup) {
    return <div>No view group found</div>;
  }

  switch (groupId) {
    case "collections.groupId":
      return <CollectionTreeView />;
    case "environments.groupId":
      return <div>No view group found</div>;
    case "mock.groupId":
      return <div>No view group found</div>;
    default:
      return <div>No view group found</div>;
  }
};

const NoWorkspaceComponent = () => {
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

  return (
    <div className="flex flex-col gap-4.25 px-2">
      <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />

      <div>
        <img src={ErrorNaughtyDog} className="mx-auto h-auto w-full max-w-[200px]" />
        <p className="text-(--moss-secondary-text)">
          You need to open a workspace before accessing collections, environments, or sending requests. Please open or
          create a workspace to proceed.
        </p>
      </div>

      <div className="flex flex-col gap-3.5">
        <ButtonPrimary onClick={openNewWorkspaceModal}>New workspace</ButtonPrimary>
        <ButtonPrimary onClick={openOpenWorkspaceModal}>Open workspace</ButtonPrimary>
      </div>
    </div>
  );
};
