import { ActionMenu, IconLabelButton } from "@/components";
import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

import CollapsibleActionMenu from "./CollapsibleActionMenu";
import { getUserMenuItems } from "./mockHeadBarData";
import { ModeToggle } from "./ModeToggle";

export interface HeadBarRightItemsProps {
  isMedium: boolean;
  isLarge: boolean;
  breakpoint: string;
  handleUserMenuAction: (action: string) => void;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
  os: string | null;
  selectedWorkspace?: string | null;
  selectedUser: string | null;
}

export const HeadBarRightItems = ({
  isMedium,
  isLarge,
  handleUserMenuAction,
  showDebugPanels,
  setShowDebugPanels,
  openPanel,
  os,
  selectedWorkspace: propSelectedWorkspace,
  selectedUser,
}: HeadBarRightItemsProps) => {
  const isMac = os === "macos";

  // Get workspace from appState as backup to prop
  const { data: appState } = useDescribeAppState();
  const { getNameById } = useWorkspaceMapping();

  // Get workspace name from prop first, then appState as backup
  const currentWorkspaceId = appState?.lastWorkspace;
  const currentWorkspaceName = currentWorkspaceId ? getNameById(currentWorkspaceId) : null;
  const selectedWorkspace = propSelectedWorkspace || currentWorkspaceName;

  return (
    <div className="flex items-center place-self-end">
      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <IconLabelButton
            leftIcon="UserAvatar"
            leftIconClassName="text-(--moss-primary) size-4.5"
            rightIcon="ChevronDown"
            title={selectedUser || "g10z3r"}
            placeholder="No user selected"
            showPlaceholder={!selectedUser}
            className="mr-2 h-[24px]"
            compact={isMedium}
          />
        </ActionMenu.Trigger>
        <ActionMenu.Content>
          {getUserMenuItems(selectedUser).map((item) => renderActionMenuItem(item, handleUserMenuAction))}
        </ActionMenu.Content>
      </ActionMenu.Root>

      {isMac && selectedWorkspace && (
        <ModeToggle className="mr-2 border-1 border-[var(--moss-headBar-border-color)]" compact={isLarge} />
      )}

      <CollapsibleActionMenu
        isCompact={isMedium}
        showDebugPanels={showDebugPanels}
        setShowDebugPanels={setShowDebugPanels}
        openPanel={openPanel}
      />
    </div>
  );
};
