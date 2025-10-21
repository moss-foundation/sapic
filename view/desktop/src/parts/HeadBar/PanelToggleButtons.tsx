import { ActionButton } from "@/components/ActionButton";
import { SIDEBAR_POSITION } from "@/constants/layoutPositions";
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
    <div className={cn("flex -space-x-0.5", className)}>
      {sideBarPosition === SIDEBAR_POSITION.LEFT && (
        <ActionButton
          iconClassName="!size-4.5"
          icon={sideBar.visible ? "OpenPanelLeftFilled" : "OpenPanelLeft"}
          onClick={toggleSidebar}
          title="Toggle Left Sidebar"
        />
      )}

      <ActionButton
        iconClassName="!size-4.5"
        icon={bottomPane.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
        onClick={toggleBottomPane}
        title="Toggle Bottom Panel"
      />

      {sideBarPosition === SIDEBAR_POSITION.RIGHT && (
        <ActionButton
          iconClassName="!size-4.5"
          icon={sideBar.visible ? "OpenPanelRightFilled" : "OpenPanelRight"}
          onClick={toggleSidebar}
          title="Toggle Right Sidebar"
        />
      )}
    </div>
  );
};

export default PanelToggleButtons;
