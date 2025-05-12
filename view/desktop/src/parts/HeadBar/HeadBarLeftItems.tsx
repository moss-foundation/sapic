import { ActionMenu, IconLabelButton } from "@/components";
import { useWorkspaceContext } from "@/context/WorkspaceContext";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

import { windowsMenuItems } from "./mockHeadBarData";
import { ModeToggle } from "./ModeToggle";
import { useWorkspaceMenu } from "./WorkspaceMenuProvider";

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
  handleWindowsMenuAction,
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
          <ActionMenu.Root>
            <ActionMenu.Trigger>
              <Icon icon="WindowsMenu" className="size-4.5 cursor-pointer text-(--moss-headBar-icon-primary-text)" />
            </ActionMenu.Trigger>
            <ActionMenu.Content>
              {windowsMenuItems.map((item) => renderActionMenuItem(item, handleWindowsMenuAction))}
            </ActionMenu.Content>
          </ActionMenu.Root>

          {selectedWorkspace && (
            <ModeToggle className="mr-0.5 border-1 border-[var(--moss-headBar-border-color)]" compact={isLarge} />
          )}
        </>
      )}

      <ActionMenu.Root>
        <ActionMenu.Trigger>
          <IconLabelButton
            rightIcon="ChevronDown"
            title={selectedWorkspace || "My Workspace"}
            placeholder="No workspace selected"
            showPlaceholder={!selectedWorkspace}
            labelClassName="text-md"
            className="h-[24px]"
          />
        </ActionMenu.Trigger>
        <ActionMenu.Content>
          {selectedWorkspace
            ? selectedWorkspaceMenuItems.map((item) => renderActionMenuItem(item, handleWorkspaceMenuAction))
            : workspaceMenuItems.map((item) => renderActionMenuItem(item, handleWorkspaceMenuAction))}
        </ActionMenu.Content>
      </ActionMenu.Root>

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
