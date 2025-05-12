import { useRef, useState } from "react";

import { NewWorkspaceModal } from "@/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/components/Modals/Workspace/OpenWorkspaceModal";
import { useWorkspaceContext } from "@/context/WorkspaceContext";
import { useModal } from "@/hooks/useModal";
import { useResponsive } from "@/hooks/useResponsive";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";
import {
  HeadBarActionProps,
  useCollectionActions,
  useGitMenuActions,
  useUserMenuActions,
  useWindowsMenuActions,
  useWorkspaceActions,
} from "./HeadBarActions";
import { HeadBarCenterItems } from "./HeadBarCenterItems";
import { HeadBarLeftItems } from "./HeadBarLeftItems";
import { HeadBarRightItems } from "./HeadBarRightItems";
import { WorkspaceMenuProvider } from "./WorkspaceMenuProvider";

export const HeadBar = () => {
  // TEST: Hardcode OS type for testing
  const os = type();
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const openPanel = useTabbedPaneStore((state) => state.openPanel);
  const { isMedium, isLarge, isXLarge, breakpoint } = useResponsive();
  const [userMenuOpen, setUserMenuOpen] = useState(false);
  const [gitMenuOpen, setGitMenuOpen] = useState(false);
  const [windowsMenuOpen, setWindowsMenuOpen] = useState(false);
  const [collectionActionMenuOpen, setCollectionActionMenuOpen] = useState(false);
  const [workspaceMenuOpen, setWorkspaceMenuOpen] = useState(false);

  // Use the workspace context instead of local state
  const { selectedWorkspace, setSelectedWorkspace } = useWorkspaceContext();

  // TEST: Hardcode default user/branch for testing
  const [selectedUser, setSelectedUser] = useState<string | null>(null);
  const [selectedBranch, setSelectedBranch] = useState<string | null>(null);
  const [collectionName, setCollectionName] = useState("Sapic Test Collection");
  const collectionButtonRef = useRef<HTMLButtonElement>(null);
  const [, setIsRenamingCollection] = useState(false);

  // Modal hooks for workspace dialogs
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

  // User menu actions
  const actionProps: HeadBarActionProps = {
    openPanel,
    showDebugPanels,
    setShowDebugPanels,
    setSelectedWorkspace,
    setSelectedUser,
    setSelectedBranch,
    openNewWorkspaceModal,
    openOpenWorkspaceModal,
  };

  const userActionProps: HeadBarActionProps = { ...actionProps };
  const gitActionProps: HeadBarActionProps = { ...actionProps };
  const windowsActionProps: HeadBarActionProps = { ...actionProps };
  const workspaceActionProps: HeadBarActionProps = { ...actionProps };
  const collectionActionProps: HeadBarActionProps = {
    ...actionProps,
    setCollectionName,
    collectionButtonRef,
    setIsRenamingCollection,
  };

  const handleUserMenuAction = useUserMenuActions(userActionProps);
  const handleGitMenuAction = useGitMenuActions(gitActionProps);
  const handleWindowsMenuAction = useWindowsMenuActions(windowsActionProps);
  const handleWorkspaceMenuAction = useWorkspaceActions(workspaceActionProps);
  const collectionActions = useCollectionActions(collectionActionProps);
  const handleCollectionActionMenuAction = collectionActions.handleCollectionActionMenuAction;

  // Action handlers
  const handleRenameCollection = (newName: string) => {
    if (newName.trim() !== "" && newName !== collectionName) {
      setCollectionName(newName);
      return true;
    }
    return false;
  };

  return (
    <WorkspaceMenuProvider>
      {/* Workspace Modals */}
      <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />

      <header
        data-tauri-drag-region
        className={cn(
          "header background-(--moss-secondary-background) grid h-full w-screen items-center shadow-[inset_0_-1px_0_0_var(--moss-border-color)]",
          {
            "grid-cols-[max-content_minmax(0px,_1fr)]": os === "macos",
            "grid-cols-[minmax(0px,_1fr)_max-content]": os !== "macos",
          }
        )}
      >
        {os === "macos" && <Controls os={os} />}

        <div
          className={cn("relative mb-0.5 flex w-full items-center overflow-clip", {
            "mr-2 pr-[8px]": os === "macos",
            "ml-2 pr-[8px]": os === "windows" || os === "linux",
          })}
          style={{ overflowClipMargin: 4 }}
          data-tauri-drag-region
        >
          {/* Main content container with proper layout */}
          <div className="flex w-full items-center justify-between" data-tauri-drag-region>
            {/*HeadBar Left-side items*/}
            <HeadBarLeftItems
              isLarge={isLarge}
              breakpoint={breakpoint}
              windowsMenuOpen={windowsMenuOpen}
              setWindowsMenuOpen={setWindowsMenuOpen}
              handleWindowsMenuAction={handleWindowsMenuAction}
              workspaceMenuOpen={workspaceMenuOpen}
              setWorkspaceMenuOpen={setWorkspaceMenuOpen}
              handleWorkspaceMenuAction={handleWorkspaceMenuAction}
              os={os}
            />

            {/*HeadBar Center items*/}
            {selectedWorkspace && (
              <HeadBarCenterItems
                isMedium={isMedium}
                isXLarge={isXLarge}
                breakpoint={breakpoint}
                gitMenuOpen={gitMenuOpen}
                setGitMenuOpen={setGitMenuOpen}
                handleGitMenuAction={handleGitMenuAction}
                collectionActionMenuOpen={collectionActionMenuOpen}
                setCollectionActionMenuOpen={setCollectionActionMenuOpen}
                handleCollectionActionMenuAction={handleCollectionActionMenuAction}
                selectedBranch={selectedBranch}
                collectionName={collectionName}
                onRenameCollection={handleRenameCollection}
                collectionButtonRef={collectionButtonRef}
                os={os}
              />
            )}

            {/*HeadBar Right-side items*/}
            <HeadBarRightItems
              isMedium={isMedium}
              isLarge={isLarge}
              breakpoint={breakpoint}
              userMenuOpen={userMenuOpen}
              setUserMenuOpen={setUserMenuOpen}
              handleUserMenuAction={handleUserMenuAction}
              showDebugPanels={showDebugPanels}
              setShowDebugPanels={setShowDebugPanels}
              openPanel={openPanel}
              os={os}
              selectedUser={selectedUser}
            />
          </div>
        </div>

        {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
        {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
      </header>
    </WorkspaceMenuProvider>
  );
};
