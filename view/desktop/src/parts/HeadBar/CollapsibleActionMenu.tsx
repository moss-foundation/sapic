import { ActionButton, ActionMenu } from "@/components";
import { SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import PanelToggleButtons from "./PanelToggleButtons";

export interface CollapsibleActionMenuProps {
  isCompact?: boolean;
  openPanel: (panel: string) => void;
}

// Collapsible Menu component that shows action buttons or collapses them into a dropdown
export const CollapsibleActionMenu = ({ isCompact = false }: CollapsibleActionMenuProps) => {
  const { sideBarPosition, bottomPane, sideBar } = useAppResizableLayoutStore();

  if (!isCompact) {
    return <PanelToggleButtons />;
  }

  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <ActionButton
          icon="MoreHorizontal"
          iconClassName="text-(--moss-headBar-icon-primary-text)"
          title="More actions"
        />
      </ActionMenu.Trigger>
      <ActionMenu.Content>
        {sideBarPosition === SIDEBAR_POSITION.LEFT ? (
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
            <ActionMenu.Item onClick={() => sideBar.setVisible(!sideBar.visible)}>
              {sideBar.visible ? "Hide Right Sidebar" : "Show Right Sidebar"}
            </ActionMenu.Item>
          </>
        )}
      </ActionMenu.Content>
    </ActionMenu.Root>
  );
};

export default CollapsibleActionMenu;
