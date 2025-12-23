import { useCurrentWorkspace } from "@/hooks";
import { cn } from "@/utils";
import { useGetLayout, useUpdateLayout } from "@/workbench/adapters";
import { SIDEBAR_POSITION } from "@/workbench/domains/layout";
import { ActionButton } from "@/workbench/ui/components/ActionButton";

export interface PanelToggleButtonsProps {
  className?: string;
}

export const PanelToggleButtons = ({ className }: PanelToggleButtonsProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  //TODO later we should handle the JsonValue differently
  const sideBarPosition = layout?.sidebarState.position || SIDEBAR_POSITION.LEFT;

  const toggleSidebar = () => {
    if (!currentWorkspaceId) return;
    updateLayout({
      layout: { sidebarState: { visible: !layout?.sidebarState.visible } },
      workspaceId: currentWorkspaceId,
    });
  };

  const toggleBottomPane = () => {
    if (!currentWorkspaceId) return;
    updateLayout({
      layout: { bottomPanelState: { visible: !layout?.bottomPanelState.visible } },
      workspaceId: currentWorkspaceId,
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
