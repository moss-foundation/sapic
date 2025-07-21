import { ActionButton, ActionMenu } from "@/components";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { SIDEBAR_POSITION } from "@/constants/layoutPositions";

import PanelToggleButtons from "./PanelToggleButtons";

export interface CollapsibleActionMenuProps {
  isCompact: boolean;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
}

// Collapsible Menu component that shows action buttons or collapses them into a dropdown
export const CollapsibleActionMenu = ({ isCompact }: CollapsibleActionMenuProps) => {
  const { sideBarPosition, bottomPane, sideBar } = useAppResizableLayoutStore();

  // When not in compact mode, show all buttons
  if (!isCompact) {
    return (
      <div className="flex items-center gap-0">
        <PanelToggleButtons className="mr-1" />
        <ActionButton icon="Bell" iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5" />
      </div>
    );
  }

  // In compact mode, use ActionMenu
  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <ActionButton
          icon="MoreHorizontal"
          iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
          title="More actions"
        />
      </ActionMenu.Trigger>
      <ActionMenu.Content>
        <ActionMenu.Item onClick={() => {}} icon="Bell" className="size-4.5">
          Notifications
        </ActionMenu.Item>
        {sideBarPosition === SIDEBAR_POSITION.LEFT ? (
          <>
            <ActionMenu.Item
              onClick={() => sideBar.setVisible(!sideBar.visible)}
              icon={sideBar.visible ? "OpenPanelLeftFilled" : "OpenPanelLeft"}
              className="size-4.5"
            >
              {sideBar.visible ? "Hide Left Sidebar" : "Show Left Sidebar"}
            </ActionMenu.Item>
            <ActionMenu.Item
              onClick={() => bottomPane.setVisible(!bottomPane.visible)}
              icon={bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
              className="size-4.5"
            >
              {bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel"}
            </ActionMenu.Item>
          </>
        ) : (
          <>
            <ActionMenu.Item
              onClick={() => bottomPane.setVisible(!bottomPane.visible)}
              icon={bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
              className="size-4.5"
            >
              {bottomPane.visible ? "Hide Bottom Panel" : "Show Bottom Panel"}
            </ActionMenu.Item>
            <ActionMenu.Item
              onClick={() => sideBar.setVisible(!sideBar.visible)}
              icon={sideBar.visible ? "OpenPanelRightFilled" : "OpenPanelRight"}
              className="size-4.5"
            >
              {sideBar.visible ? "Hide Right Sidebar" : "Show Right Sidebar"}
            </ActionMenu.Item>
          </>
        )}
      </ActionMenu.Content>
    </ActionMenu.Root>
  );
};

export default CollapsibleActionMenu;
