import { useRef, useState } from "react";

import { NewWorkspaceModal } from "@/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/components/Modals/Workspace/OpenWorkspaceModal";
import { RenameWorkspaceModal } from "@/components/Modals/Workspace/RenameWorkspaceModal";
import { ConfirmationModal } from "@/components";
import { useActiveWorkspace } from "@/hooks";
import { useModal } from "@/hooks/useModal";
import { useResponsive } from "@/hooks/useResponsive";
import { useDeleteWorkspace } from "@/hooks/workbench";
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
  // TEST: Hardoce OS type for testing
  const os = type();
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const openPanel = useTabbedPaneStore((state) => state.openPanel);
  const { isMedium, isLarge, isXLarge, breakpoint } = useResponsive();

  const workspace = useActiveWorkspace();
  const selectedWorkspace = workspace?.displayName || null;

  // TEST: Hardoce default user/branch for testing
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

  // Delete confirmation modal state
  const [showDeleteConfirmModal, setShowDeleteConfirmModal] = useState(false);
  const [workspaceToDelete, setWorkspaceToDelete] = useState<{ id: string; name: string } | null>(null);

  // Rename workspace modal state
  const [showRenameWorkspaceModal, setShowRenameWorkspaceModal] = useState(false);
  const [workspaceToRename, setWorkspaceToRename] = useState<{ id: string; name: string } | null>(null);

  // Delete workspace hook
  const { mutate: deleteWorkspace } = useDeleteWorkspace();

  // User menu actions
  const actionProps: HeadBarActionProps = {
    openPanel,
    showDebugPanels,
    setShowDebugPanels,
    setSelectedUser,
    setSelectedBranch,
    openNewWorkspaceModal,
    openOpenWorkspaceModal,
    showDeleteConfirmModal,
    setShowDeleteConfirmModal,
    workspaceToDelete,
    setWorkspaceToDelete,
    setShowRenameWorkspaceModal,
    setWorkspaceToRename,
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

  // Delete workspace confirmation handler
  const handleDeleteWorkspace = () => {
    if (workspaceToDelete) {
      deleteWorkspace({ id: workspaceToDelete.id });
      setWorkspaceToDelete(null);
    }
  };

  // Close delete confirmation modal
  const closeDeleteConfirmModal = () => {
    setShowDeleteConfirmModal(false);
    setWorkspaceToDelete(null);
  };

  // Close rename workspace modal
  const closeRenameWorkspaceModal = () => {
    setShowRenameWorkspaceModal(false);
    setWorkspaceToRename(null);
  };

  return (
    <WorkspaceMenuProvider>
      {/* Workspace Modals */}
      <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      
      {/* Rename Workspace Modal */}
      {workspaceToRename && (
        <RenameWorkspaceModal
          showModal={showRenameWorkspaceModal}
          closeModal={closeRenameWorkspaceModal}
          workspaceId={workspaceToRename.id}
          currentName={workspaceToRename.name}
        />
      )}

      {/* Delete Confirmation Modal */}
      <ConfirmationModal
        showModal={showDeleteConfirmModal}
        closeModal={closeDeleteConfirmModal}
        title="Delete Workspace"
        message={`Are you sure you want to delete the workspace "${workspaceToDelete?.name}"? This action cannot be undone.`}
        confirmLabel="Delete"
        cancelLabel="Cancel"
        onConfirm={handleDeleteWorkspace}
        variant="danger"
        icon="Delete"
      />

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
          <div
            className={cn("grid w-full gap-1", {
              "grid-cols-[minmax(1px,673px)_1fr_1fr]": selectedWorkspace,
              "grid-cols-[1fr_1fr]": !selectedWorkspace,
            })}
            data-tauri-drag-region
          >
            {/*HeadBar Left-side items*/}
            <HeadBarLeftItems
              isLarge={isLarge}
              breakpoint={breakpoint}
              handleWindowsMenuAction={handleWindowsMenuAction}
              handleWorkspaceMenuAction={handleWorkspaceMenuAction}
              os={os}
              selectedWorkspace={selectedWorkspace}
            />

            {/*HeadBar Center items*/}
            {selectedWorkspace && (
              <HeadBarCenterItems
                isMedium={isMedium}
                isXLarge={isXLarge}
                breakpoint={breakpoint}
                handleGitMenuAction={handleGitMenuAction}
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
              handleUserMenuAction={handleUserMenuAction}
              showDebugPanels={showDebugPanels}
              setShowDebugPanels={setShowDebugPanels}
              openPanel={openPanel}
              os={os}
              selectedWorkspace={selectedWorkspace}
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
