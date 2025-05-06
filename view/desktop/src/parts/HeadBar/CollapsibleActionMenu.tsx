import { useState } from "react";

import { ActionButton } from "@/components";
import ActionMenu from "@/components/ActionMenu/ActionMenu";
import { type Icons } from "@/components/Icon";
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
    <ActionMenu
      items={[
        {
          id: "notifications",
          type: "action" as const,
          label: "Notifications",
          icon: "Bell" as Icons,
        },
        ...(sideBarPosition === "left"
          ? [
              {
                id: "toggleLeftSidebar",
                type: "action" as const,
                label: sideBar.visible ? "Hide Left Sidebar" : "Show Left Sidebar",
                icon: (sideBar.visible ? "OpenPanelLeftFilled" : "OpenPanelLeft") as Icons,
              },
              {
                id: "toggleBottomPanel",
                type: "action" as const,
                label: bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel",
                icon: (bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom") as Icons,
              },
            ]
          : [
              {
                id: "toggleBottomPanel",
                type: "action" as const,
                label: bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel",
                icon: (bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom") as Icons,
              },
              {
                id: "toggleRightSidebar",
                type: "action" as const,
                label: sideBar.visible ? "Hide Right Sidebar" : "Show Right Sidebar",
                icon: (sideBar.visible ? "OpenPanelRightFilled" : "OpenPanelRight") as Icons,
              },
            ]),
        {
          id: "settings",
          type: "action" as const,
          label: "Settings",
          icon: "Settings" as Icons,
        },
      ]}
      trigger={
        <ActionButton
          icon="MoreHorizontal"
          iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
          title="More actions"
        />
      }
      open={isMenuOpen}
      onOpenChange={setIsMenuOpen}
      onSelect={(item) => {
        if (item.id === "notifications") {
          // Handle notifications
        }
        if (item.id === "toggleLeftSidebar" || item.id === "toggleRightSidebar") {
          sideBar.setVisible(!sideBar.visible);
        }
        if (item.id === "toggleBottomPanel") {
          bottomPane.setVisible(!bottomPane.visible);
        }
        if (item.id === "settings") openPanel("Settings");
      }}
    />
  );
};

export default CollapsibleActionMenu;
