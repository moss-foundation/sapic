import { useActiveWorkspace, useDescribeApp } from "@/hooks";
import { useGetLayout } from "@/hooks/workbench/layout/useGetLayout";
import { useUpdateLayout } from "@/hooks/workbench/layout/useUpdateLayout";
import { cn } from "@/utils";
import { SIDEBAR_POSITION } from "@/workbench/domains/layout";
import { ActionButton } from "@/workbench/ui/components/ActionButton";

export interface PanelToggleButtonsProps {
  className?: string;
}

export const PanelToggleButtons = ({ className }: PanelToggleButtonsProps) => {
  const { data: appState } = useDescribeApp();
  const { activeWorkspaceId } = useActiveWorkspace();
  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  //TODO later we should handle the JsonValue differently
  const sideBarPosition = appState?.configuration.contents.sideBarPosition as SIDEBAR_POSITION;

  const toggleSidebar = () => {
    updateLayout({
      layout: { sidebarState: { visible: !layout?.sidebarState.visible } },
      workspaceId: activeWorkspaceId,
    });
  };

  const toggleBottomPane = () => {
    if (!activeWorkspaceId) return;
    updateLayout({
      layout: { bottomPanelState: { visible: !layout?.bottomPanelState.visible } },
      workspaceId: activeWorkspaceId,
    });
  };

  return (
    <div className={cn("flex -space-x-0.5", className)}>
      {sideBarPosition === SIDEBAR_POSITION.LEFT && (
        <ActionButton
          iconClassName="!size-4.5"
          icon={layout?.sidebarState.visible ? "OpenPanelLeftFilled" : "OpenPanelLeft"}
          onClick={toggleSidebar}
          title="Toggle Left Sidebar"
        />
      )}

      <ActionButton
        iconClassName="!size-4.5"
        icon={layout?.bottomPanelState.visible ? "OpenPanelBottomFilled" : "OpenPanelBottom"}
        onClick={toggleBottomPane}
        title="Toggle Bottom Panel"
      />

      {sideBarPosition === SIDEBAR_POSITION.RIGHT && (
        <ActionButton
          iconClassName="!size-4.5"
          icon={layout?.sidebarState.visible ? "OpenPanelRightFilled" : "OpenPanelRight"}
          onClick={toggleSidebar}
          title="Toggle Right Sidebar"
        />
      )}
    </div>
  );
};

export default PanelToggleButtons;
