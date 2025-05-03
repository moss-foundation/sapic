import { ActionButton, IconLabelButton } from "@/components";
import { cn } from "@/utils";
import ActionMenu from "@/components/ActionMenu/ActionMenu";
import { ModeToggle } from "./ModeToggle";
import { windowsMenuItems } from "./mockHeadBarData";
import { workspaceMenuItems, selectedWorkspaceMenuItems } from "./HeadBarData";

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
  selectedWorkspace: string | null;
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
  selectedWorkspace,
}: HeadBarLeftItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";

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
                icon="HeadBarWindowsMenu"
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
          leftIcon="HeadBarVault"
          leftIconClassName="--moss-headBar-icon-primary-text size-4.5"
          title="Vault"
          className="h-[24px]"
          compact={isLarge}
        />
      )}
    </div>
  );
};
