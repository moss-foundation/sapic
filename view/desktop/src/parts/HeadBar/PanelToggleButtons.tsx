import { ActionButton } from "@/components/ActionButton";
import { SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/useGetSidebarPanel";
import { useUpdateSidebarPanel } from "@/hooks/sharedStorage/layout/useUpdateSidebarPanel";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

export interface PanelToggleButtonsProps {
  className?: string;
}

export const PanelToggleButtons = ({ className }: PanelToggleButtonsProps) => {
  const { bottomPane } = useAppResizableLayoutStore();
  const { activeWorkspaceId } = useActiveWorkspace();
  const { data: sideBar } = useGetSidebarPanel();
  const { mutate: updateSidebarPanel } = useUpdateSidebarPanel();

  const toggleSidebar = () => {
    updateSidebarPanel({ visible: !sideBar?.visible });
  };

  const toggleBottomPane = () => {
    if (!activeWorkspaceId) return;
    bottomPane.setVisible(!bottomPane.visible, activeWorkspaceId);
  };

  return (
    <div className={cn("flex -space-x-0.5", className)}>
      {sideBar?.position === SIDEBAR_POSITION.LEFT && (
        <ActionButton
          iconClassName="!size-4.5"
          icon={sideBar?.visible ? "OpenPanelLeftFilled" : "OpenPanelLeft"}
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

      {sideBar?.position === SIDEBAR_POSITION.RIGHT && (
        <ActionButton
          iconClassName="!size-4.5"
          icon={sideBar?.visible ? "OpenPanelRightFilled" : "OpenPanelRight"}
          onClick={toggleSidebar}
          title="Toggle Right Sidebar"
        />
      )}
    </div>
  );
};

export default PanelToggleButtons;
