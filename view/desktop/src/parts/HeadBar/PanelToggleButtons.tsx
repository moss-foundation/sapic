import { ActionButton } from "@/components/ActionButton";
import { SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useGetBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useGetBottomPanel";
import { useUpdateBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useUpdateBottomPanel";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useGetSidebarPanel";
import { useUpdateSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useUpdateSidebarPanel";
import { cn } from "@/utils";

export interface PanelToggleButtonsProps {
  className?: string;
}

export const PanelToggleButtons = ({ className }: PanelToggleButtonsProps) => {
  const { data: bottomPane } = useGetBottomPanel();
  const { mutate: updateBottomPanel } = useUpdateBottomPanel();

  const { activeWorkspaceId } = useActiveWorkspace();
  const { data: sideBar } = useGetSidebarPanel();
  const { mutate: updateSidebarPanel } = useUpdateSidebarPanel();

  const toggleSidebar = () => {
    updateSidebarPanel({ visible: !sideBar?.visible, workspaceId: activeWorkspaceId });
  };

  const toggleBottomPane = () => {
    if (!activeWorkspaceId) return;
    updateBottomPanel({ visible: !bottomPane?.visible, workspaceId: activeWorkspaceId });
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
        icon={bottomPane?.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
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
