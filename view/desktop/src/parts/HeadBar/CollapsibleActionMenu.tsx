import { useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import PanelToggleButtons from "./PanelToggleButtons";

export interface CollapsibleActionMenuProps {
  isCompact: boolean;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
}

// Collapsible Menu component that shows action buttons or collapses them into a dropdown
export const CollapsibleActionMenu = ({ isCompact, openPanel }: CollapsibleActionMenuProps) => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const { sideBarPosition, bottomPane, sideBar } = useAppResizableLayoutStore();

  // When not in compact mode, show all buttons
  if (!isCompact) {
    return (
      <div className="flex items-center gap-0">
        <PanelToggleButtons className="mr-1" />
        <ActionButton icon="Bell" iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5" />
        <ActionButton
          icon="Settings"
          iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
          onClick={() => openPanel("Settings")}
          title="Settings"
        />
      </div>
    );
  }

  // In compact mode, use ActionMenu
  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger>
        <ActionButton
          icon="MoreHorizontal"
          iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
          title="More actions"
        />
      </ActionMenu.Trigger>
      <ActionMenu.Content>
        <ActionMenu.Item onClick={() => {}} icon="Bell">
          Notifications
        </ActionMenu.Item>
        {sideBarPosition === "left" ? (
          <>
            <ActionMenu.Item
              onClick={() => sideBar.setVisible(!sideBar.visible)}
              icon={sideBar.visible ? "OpenPanelLeftFilled" : "OpenPanelLeft"}
            >
              {sideBar.visible ? "Hide Left Sidebar" : "Show Left Sidebar"}
            </ActionMenu.Item>
            <ActionMenu.Item
              onClick={() => bottomPane.setVisible(!bottomPane.visible)}
              icon={bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
            >
              {bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel"}
            </ActionMenu.Item>
          </>
        ) : (
          <>
            <ActionMenu.Item
              onClick={() => bottomPane.setVisible(!bottomPane.visible)}
              icon={bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
            >
              {bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel"}
            </ActionMenu.Item>
            <ActionMenu.Item
              onClick={() => sideBar.setVisible(!sideBar.visible)}
              icon={sideBar.visible ? "OpenPanelRightFilled" : "OpenPanelRight"}
            >
              {sideBar.visible ? "Hide Right Sidebar" : "Show Right Sidebar"}
            </ActionMenu.Item>
          </>
        )}
        <ActionMenu.Item onClick={() => openPanel("Settings")} icon="Settings">
          Settings
        </ActionMenu.Item>
      </ActionMenu.Content>
    </ActionMenu.Root>
  );
};

export default CollapsibleActionMenu;
