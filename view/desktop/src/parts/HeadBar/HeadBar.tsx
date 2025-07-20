import { useRef, useState } from "react";

import { ConfirmationModal } from "@/components";
import { NewWorkspaceModal } from "@/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/components/Modals/Workspace/OpenWorkspaceModal";
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
  const selectedWorkspace = workspace?.name || null;

  // TEST: Hardoce default user/branch for testing
  const [, setSelectedUser] = useState<string | null>(null);
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

  // Delete workspace hook
  const { mutate: deleteWorkspace, isPending: isDeleting } = useDeleteWorkspace();

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
  };

  const gitActionProps: HeadBarActionProps = { ...actionProps };
  const workspaceActionProps: HeadBarActionProps = { ...actionProps };
  const collectionActionProps: HeadBarActionProps = {
    ...actionProps,
    setCollectionName,
    collectionButtonRef,
    setIsRenamingCollection,
  };

  const handleGitMenuAction = useGitMenuActions(gitActionProps);
  const handleWindowsMenuAction = useWindowsMenuActions();
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
      deleteWorkspace(
        { id: workspaceToDelete.id },
        {
          onSuccess: () => {
            setWorkspaceToDelete(null);
            setShowDeleteConfirmModal(false);
          },
          onError: (error) => {
            console.error("Failed to delete workspace:", error.message);
            setWorkspaceToDelete(null);
            setShowDeleteConfirmModal(false);
          },
        }
      );
    }
  };

  // Close delete confirmation modal
  const closeDeleteConfirmModal = () => {
    setShowDeleteConfirmModal(false);
    setWorkspaceToDelete(null);
  };

  return (
    <WorkspaceMenuProvider>
      {/* Workspace Modals */}
      {showNewWorkspaceModal && (
        <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      )}
      {showOpenWorkspaceModal && (
        <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      )}

      {/* Delete Confirmation Modal */}
      {showDeleteConfirmModal && (
        <ConfirmationModal
          showModal={showDeleteConfirmModal}
          closeModal={closeDeleteConfirmModal}
          title="Delete"
          message={`Delete "${workspaceToDelete?.name}"?`}
          description="This will delete the monitors, scheduled runs and integrations and deactivate the mock servers associated with collections in the workspace."
          confirmLabel={isDeleting ? "Deleting..." : "Delete"}
          cancelLabel="Close"
          onConfirm={handleDeleteWorkspace}
          variant="danger"
          loading={isDeleting}
        />
      )}

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
              "grid-cols-[max-content_1fr_max-content]": selectedWorkspace,
              "grid-cols-[max-content_1fr]": !selectedWorkspace,
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
              <div
                className={cn("flex", {
                  "justify-center": os === "macos",
                  "-mr-[138px] justify-center": os === "windows" || os === "linux",
                })}
                data-tauri-drag-region
              >
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
                  onNavigateBack={() => console.log("Navigate back")}
                  onNavigateForward={() => console.log("Navigate forward")}
                  canGoBack={true}
                  canGoForward={true}
                  onZoomIn={() => console.log("Zoom in")}
                  onZoomOut={() => console.log("Zoom out")}
                  canZoomIn={true}
                  canZoomOut={true}
                  currentZoom={100}
                />
              </div>
            )}

            {/*HeadBar Right-side items*/}
            <div className="flex justify-end" data-tauri-drag-region>
              <HeadBarRightItems
                isMedium={isMedium}
                isLarge={isLarge}
                breakpoint={breakpoint}
                showDebugPanels={showDebugPanels}
                setShowDebugPanels={setShowDebugPanels}
                openPanel={openPanel}
                os={os}
                selectedWorkspace={selectedWorkspace}
              />
            </div>
          </div>
        </div>

        {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
        {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
      </header>
    </WorkspaceMenuProvider>
  );
};
