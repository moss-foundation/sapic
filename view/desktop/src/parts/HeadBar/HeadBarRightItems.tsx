import { IconLabelButton } from "@/components";
import ActionMenu from "@/components/ActionMenu/ActionMenu";

import CollapsibleActionMenu from "./CollapsibleActionMenu";
import { getUserMenuItems } from "./mockHeadBarData";
import { useWorkspaceContext } from "@/context/WorkspaceContext";
import { ModeToggle } from "./ModeToggle";

export interface HeadBarRightItemsProps {
  isMedium: boolean;
  isLarge: boolean;
  breakpoint: string;
  userMenuOpen: boolean;
  setUserMenuOpen: (open: boolean) => void;
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
  userMenuOpen,
  setUserMenuOpen,
  handleUserMenuAction,
  showDebugPanels,
  setShowDebugPanels,
  openPanel,
  os,
  selectedUser,
}: HeadBarRightItemsProps) => {
  const isMac = os === "macos";
  const { selectedWorkspace } = useWorkspaceContext();

  return (
    <div className="flex items-center">
      <ActionMenu
        items={getUserMenuItems(selectedUser)}
        trigger={
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
        }
        open={userMenuOpen}
        onOpenChange={setUserMenuOpen}
        onSelect={(item) => {
          handleUserMenuAction(item.id);
        }}
      />

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
