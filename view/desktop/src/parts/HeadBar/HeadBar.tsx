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
  //FIXME: Hardoce OS type for testing
  const os = type();
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const openPanel = useTabbedPaneStore((state) => state.openPanel);
  const { isMedium, isLarge, isXLarge, breakpoint } = useResponsive();

  const { hasActiveWorkspace } = useActiveWorkspace();

  //FIXME: Hardoce default user/branch for testing
  const [, setSelectedUser] = useState<string | null>(null);
  const [selectedBranch, setSelectedBranch] = useState<string | null>(null);
  const [collectionName, setCollectionName] = useState("Sapic Test Collection");
  const collectionButtonRef = useRef<HTMLButtonElement>(null);
  const [, setIsRenamingCollection] = useState(false);

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

  const [showDeleteConfirmModal, setShowDeleteConfirmModal] = useState(false);
  const [workspaceToDelete, setWorkspaceToDelete] = useState<{ id: string; name: string } | null>(null);

  const { mutate: deleteWorkspace, isPending: isDeleting } = useDeleteWorkspace();

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

  const handleRenameCollection = (newName: string) => {
    if (newName.trim() !== "" && newName !== collectionName) {
      setCollectionName(newName);
      return true;
    }
    return false;
  };

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
          "header background-(--moss-secondary-background) grid h-full w-screen items-center border-b border-(--moss-border-color)",
          {
            "grid-cols-[max-content_1fr]": os === "macos",
            "grid-cols-[1fr]": os !== "macos",
          }
        )}
      >
        {os === "macos" && <Controls os={os} />}

        <div
          className={cn("relative flex h-full w-full items-center overflow-clip", {
            "mr-2 pr-[8px]": os === "macos",
            "ml-[7px] pr-[8px]": os === "windows" || os === "linux",
          })}
          style={{ overflowClipMargin: 4 }}
          data-tauri-drag-region
        >
          {/* Main content container with proper layout */}
          <div
            className={cn("relative grid h-full w-full items-center justify-between gap-1", {
              "grid-cols-[1fr_max-content_1fr]": hasActiveWorkspace,
              "grid-cols-[max-content_max-content]": !hasActiveWorkspace,
            })}
            data-tauri-drag-region
          >
            <HeadBarLeftItems
              isLarge={isLarge}
              breakpoint={breakpoint}
              handleWindowsMenuAction={handleWindowsMenuAction}
              handleWorkspaceMenuAction={handleWorkspaceMenuAction}
              os={os}
            />

            {/*HeadBar Center items - absolutely positioned to be truly centered */}
            {hasActiveWorkspace && (
              <div
                className={cn("w-full", {
                  "pointer-events-none": os === "macos",
                })}
                data-tauri-drag-region
              >
                <div className="pointer-events-auto">
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
              </div>
            )}

            <HeadBarRightItems
              isMedium={isMedium}
              isLarge={isLarge}
              breakpoint={breakpoint}
              showDebugPanels={showDebugPanels}
              setShowDebugPanels={setShowDebugPanels}
              openPanel={openPanel}
              os={os}
            />
          </div>
        </div>
      </header>
    </WorkspaceMenuProvider>
  );
};
