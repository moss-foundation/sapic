import { RefObject } from "react";

export interface HeadBarActionProps {
  openPanel: (panel: string) => void;
  setShowDebugPanels: (show: boolean) => void;
  showDebugPanels: boolean;
  setCollectionName: (name: string) => void;
  collectionButtonRef: RefObject<HTMLButtonElement>;
  setIsRenamingCollection: (isRenaming: boolean) => void;
}

/**
 * User menu action handler
 */
export const useUserMenuActions = () => {
  return (action: string) => {
    console.log(`User action: ${action}`);
    // Here you would handle different user actions like profile, settings, logout, etc.
  };
};

/**
 * Git menu action handler
 */
export const useGitMenuActions = () => {
  return (action: string) => {
    console.log(`Git action: ${action}`);
    // Here you would handle different git actions like branch switching, pull, push, etc.
  };
};

/**
 * Windows menu action handler
 */
export const useWindowsMenuActions = () => {
  return (action: string) => {
    console.log(`Windows menu action: ${action}`);
    // Here you would handle different Windows menu actions
  };
};

/**
 * Collection action menu handler
 */
export const useCollectionActions = (props: HeadBarActionProps) => {
  const { setCollectionName, collectionButtonRef, setIsRenamingCollection } = props;

  const startRenameCollection = () => {
    setIsRenamingCollection(true);

    // Use a small timeout to ensure the menu has closed
    setTimeout(() => {
      // Dispatch a double-click event to the collection button to trigger renaming
      if (collectionButtonRef.current) {
        const doubleClickEvent = new MouseEvent("dblclick", {
          bubbles: true,
          cancelable: true,
          view: window,
        });
        collectionButtonRef.current.dispatchEvent(doubleClickEvent);
      }
    }, 50);
  };

  const handleRenameCollection = (newName: string) => {
    if (newName.trim() !== "") {
      setCollectionName(newName);
    }
    setIsRenamingCollection(false);
  };

  const handleCollectionActionMenuAction = (action: string) => {
    console.log(`Collection action: ${action}`);

    // Check if the rename action was selected
    if (action === "rename") {
      startRenameCollection();
    }
  };

  return {
    handleCollectionActionMenuAction,
    handleRenameCollection,
    startRenameCollection,
  };
};

/**
 * Workspace menu action handler
 */
export const useWorkspaceActions = (props: HeadBarActionProps) => {
  const { openPanel, setShowDebugPanels, showDebugPanels } = props;

  return (action: string) => {
    console.log(`Workspace action: ${action}`);
    // Handle different workspace actions
    if (action === "home") openPanel("Home");
    if (action === "logs") openPanel("Logs");
    if (action === "debug") setShowDebugPanels(!showDebugPanels);
  };
};
