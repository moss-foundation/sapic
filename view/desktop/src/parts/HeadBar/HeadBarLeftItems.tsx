import { ActionMenu, IconLabelButton } from "@/components";
import { useActiveWorkspace } from "@/hooks";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

import { windowsMenuItems } from "./mockHeadBarData";
import { ModeToggle } from "./ModeToggle";
import { useWorkspaceMenu } from "./WorkspaceMenuProvider";

export interface HeadBarLeftItemsProps {
  isLarge: boolean;
  breakpoint: string;
  handleWindowsMenuAction: (action: string) => void;
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
  selectedWorkspace: propSelectedWorkspace,
}: HeadBarLeftItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";
  const { workspaceMenuItems, selectedWorkspaceMenuItems } = useWorkspaceMenu();

  const workspace = useActiveWorkspace();
  const selectedWorkspace = propSelectedWorkspace || workspace?.name || null;

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
            <ActionMenu.Trigger className="hover:background-(--moss-secondary-background-hover) rounded p-1">
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
        <ActionMenu.Trigger asChild>
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
        <button className="hover:background-(--moss-icon-primary-background-hover) flex h-[24px] cursor-pointer items-center gap-1 rounded px-1">
          <Icon icon="Key" className="size-4.5 text-(--moss-headBar-icon-primary-text)" />
          <span className="text-md">Vault</span>
        </button>
      )}
    </div>
  );
};
