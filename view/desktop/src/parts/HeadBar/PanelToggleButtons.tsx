import { ActionButton } from "@/components";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

export interface PanelToggleButtonsProps {
  className?: string;
}

export const PanelToggleButtons = ({ className }: PanelToggleButtonsProps) => {
  const { sideBarPosition, bottomPane, sideBar } = useAppResizableLayoutStore();

  const toggleSidebar = () => {
    sideBar.setVisible(!sideBar.visible);
  };

  const toggleBottomPane = () => {
    bottomPane.setVisible(!bottomPane.visible);
  };

  return (
    <div className={cn("flex shrink-0 -space-x-0.5", className)}>
      {sideBarPosition === "left" ? (
        <>
          <ActionButton
            iconClassName="size-4.5 text-(--moss-headBar-icon-primary-text)"
            icon={sideBar.visible ? "HeadBarLeftSideBarActive" : "HeadBarLeftSideBar"}
            onClick={toggleSidebar}
            title="Toggle Left Sidebar"
          />
          <ActionButton
            iconClassName="size-4.5 text-(--moss-headBar-icon-primary-text)"
            icon={bottomPane.visible ? "HeadBarPanelActive" : "HeadBarPanel"}
            onClick={toggleBottomPane}
            title="Toggle Bottom Panel"
          />
        </>
      ) : (
        <>
          <ActionButton
            iconClassName="size-4.5 text-(--moss-headBar-icon-primary-text)"
            icon={bottomPane.visible ? "HeadBarPanelActive" : "HeadBarPanel"}
            onClick={toggleBottomPane}
            title="Toggle Bottom Panel"
          />
          <ActionButton
            iconClassName="size-4.5 text-(--moss-headBar-icon-primary-text)"
            icon={sideBar.visible ? "HeadBarRightSideBarActive" : "HeadBarRightSideBar"}
            onClick={toggleSidebar}
            title="Toggle Right Sidebar"
          />
        </>
      )}
    </div>
  );
};

export default PanelToggleButtons;
