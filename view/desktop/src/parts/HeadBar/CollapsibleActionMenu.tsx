import { useState } from "react";

import { ActionButton, ActionMenuRadix } from "@/components";
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
    <ActionMenuRadix.Root>
      <ActionMenuRadix.Trigger>
        <ActionButton
          icon="MoreHorizontal"
          iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
          title="More actions"
        />
      </ActionMenuRadix.Trigger>
      <ActionMenuRadix.Content>
        <ActionMenuRadix.Item onClick={() => {}} icon="Bell">
          Notifications
        </ActionMenuRadix.Item>
        {sideBarPosition === "left" ? (
          <>
            <ActionMenuRadix.Item
              onClick={() => sideBar.setVisible(!sideBar.visible)}
              icon={sideBar.visible ? "OpenPanelLeftFilled" : "OpenPanelLeft"}
            >
              {sideBar.visible ? "Hide Left Sidebar" : "Show Left Sidebar"}
            </ActionMenuRadix.Item>
            <ActionMenuRadix.Item
              onClick={() => bottomPane.setVisible(!bottomPane.visible)}
              icon={bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
            >
              {bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel"}
            </ActionMenuRadix.Item>
          </>
        ) : (
          <>
            <ActionMenuRadix.Item
              onClick={() => bottomPane.setVisible(!bottomPane.visible)}
              icon={bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
            >
              {bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel"}
            </ActionMenuRadix.Item>
            <ActionMenuRadix.Item
              onClick={() => sideBar.setVisible(!sideBar.visible)}
              icon={sideBar.visible ? "OpenPanelRightFilled" : "OpenPanelRight"}
            >
              {sideBar.visible ? "Hide Right Sidebar" : "Show Right Sidebar"}
            </ActionMenuRadix.Item>
          </>
        )}
        <ActionMenuRadix.Item onClick={() => openPanel("Settings")} icon="Settings">
          Settings
        </ActionMenuRadix.Item>
      </ActionMenuRadix.Content>
    </ActionMenuRadix.Root>
  );
};

export default CollapsibleActionMenu;
