import { ActionButton, IconLabelButton } from "@/components";
import ActionMenu from "@/components/ActionMenu/ActionMenu";
import { cn } from "@/utils";

import { windowsMenuItems } from "./mockHeadBarData";
import { useWorkspaceMenu } from "./WorkspaceMenuProvider";
import { useWorkspaceContext } from "@/context/WorkspaceContext";
import { ModeToggle } from "./ModeToggle";

export interface HeadBarLeftItemsProps {
  isLarge: boolean;
  breakpoint: string;
  windowsMenuOpen: boolean;
  setWindowsMenuOpen: (open: boolean) => void;
  handleWindowsMenuAction: (action: string) => void;
  workspaceMenuOpen: boolean;
  setWorkspaceMenuOpen: (open: boolean) => void;
  handleWorkspaceMenuAction: (action: string) => void;
  os: string | null;
  selectedWorkspace?: string | null;
}

export const HeadBarLeftItems = ({
  isLarge,
  breakpoint,
  windowsMenuOpen,
  setWindowsMenuOpen,
  handleWindowsMenuAction,
  workspaceMenuOpen,
  setWorkspaceMenuOpen,
  handleWorkspaceMenuAction,
  os,
}: HeadBarLeftItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";
  const { workspaceMenuItems, selectedWorkspaceMenuItems } = useWorkspaceMenu();
  const { selectedWorkspace } = useWorkspaceContext();

  return (
    <div
      className={cn("flex items-center", {
        "gap-0.5": breakpoint === "md",
        "gap-1.5": ["lg", "xl", "2xl"].includes(breakpoint),
      })}
      data-tauri-drag-region
    >
      {isWindowsOrLinux && (
        <>
          <ActionMenu
            items={windowsMenuItems}
            trigger={
              <ActionButton
                icon="WindowsMenu"
                iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
                title="Menu"
              />
            }
            open={windowsMenuOpen}
            onOpenChange={setWindowsMenuOpen}
            onSelect={(item) => {
              handleWindowsMenuAction(item.id);
            }}
          />
          {selectedWorkspace && (
            <ModeToggle className="mr-0.5 border-1 border-[var(--moss-headBar-border-color)]" compact={isLarge} />
          )}
        </>
      )}
      <ActionMenu
        items={selectedWorkspace ? selectedWorkspaceMenuItems : workspaceMenuItems}
        trigger={
          <IconLabelButton
            rightIcon="ChevronDown"
            title={selectedWorkspace || "My Workspace"}
            placeholder="No workspace selected"
            showPlaceholder={!selectedWorkspace}
            labelClassName="text-md"
            className="h-[24px]"
          />
        }
        open={workspaceMenuOpen}
        onOpenChange={setWorkspaceMenuOpen}
        onSelect={(item) => {
          handleWorkspaceMenuAction(item.id);
        }}
      />
      {selectedWorkspace && (
        <IconLabelButton
          leftIcon="Key"
          leftIconClassName="--moss-headBar-icon-primary-text size-4.5"
          title="Vault"
          className="h-[24px]"
          compact={isLarge}
        />
      )}
    </div>
  );
};
