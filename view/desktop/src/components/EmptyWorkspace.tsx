import { useTranslation } from "react-i18next";
import { useModal } from "@/hooks/useModal";
import { useGlobalSidebarState } from "@/hooks/workspace/useGlobalSidebarState";
import ButtonPrimary from "./ButtonPrimary";
import { NewWorkspaceModal } from "./Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "./Modals/Workspace/OpenWorkspaceModal";
import ErrorNaughtyDog from "../assets/images/ErrorNaughtyDog.svg";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface EmptyWorkspaceProps {
  inSidebar?: boolean;
}

export const EmptyWorkspace = ({ inSidebar = false }: EmptyWorkspaceProps) => {
  const { t } = useTranslation();

  useGlobalSidebarState();

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
          <ButtonPrimary onClick={openNewWorkspaceModal}>{t("New workspace")}</ButtonPrimary>
          <ButtonPrimary onClick={openOpenWorkspaceModal}>{t("Open workspace")}</ButtonPrimary>
        </div>
      </div>
    );
  }

  // Main content area - render TabbedPane with WelcomePage
  return (
    <>
      <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      <TabbedPane theme="dockview-theme-light" mode="welcome" />
    </>
  );
};
