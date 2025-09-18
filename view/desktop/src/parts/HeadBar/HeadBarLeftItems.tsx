import { ActionMenu, IconLabelButton } from "@/components";
import { useActiveWorkspace, useListWorkspaces } from "@/hooks";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { renderActionMenuItem } from "@/utils/renderActionMenuItem";

import { windowsMenuItems } from "./mockHeadBarData";
import NavigationButtons from "./NavigationButtons";
import { useWorkspaceMenu } from "./WorkspaceMenuProvider";

export interface HeadBarLeftItemsProps {
  handleWindowsMenuAction: (action: string) => void;
  handleWorkspaceMenuAction: (action: string) => void;
  os: string | null;
}

export const HeadBarLeftItems = ({ handleWindowsMenuAction, handleWorkspaceMenuAction, os }: HeadBarLeftItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";
  const { workspaceMenuItems, selectedWorkspaceMenuItems } = useWorkspaceMenu();

  const { hasActiveWorkspace, activeWorkspace } = useActiveWorkspace();
  const { data: workspaceList } = useListWorkspaces();

  const selectedWorkspaceName = workspaceList?.find((workspace) => workspace.id === activeWorkspace?.id)?.name || null;

  return (
    <div className={cn("flex items-center justify-start gap-[6px] overflow-hidden")} data-tauri-drag-region>
      {isWindowsOrLinux && (
        <ActionMenu.Root>
          <ActionMenu.Trigger className="hover:!background-(--moss-icon-secondary-background-hover) rounded p-1">
            <Icon icon="WindowsMenu" className="size-4.5 cursor-pointer text-(--moss-headBar-icon-primary-text)" />
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            {windowsMenuItems.map((item) => renderActionMenuItem(item, handleWindowsMenuAction))}
          </ActionMenu.Content>
        </ActionMenu.Root>
      )}

      <NavigationButtons onBack={() => {}} onForward={() => {}} canGoBack={true} canGoForward={true} />

      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <IconLabelButton
            rightIcon="ChevronDown"
            title={selectedWorkspaceName || "My Workspace"}
            placeholder="No workspace selected"
            showPlaceholder={!hasActiveWorkspace}
            labelClassName="text-md"
            className="hover:!background-(--moss-icon-secondary-background-hover) h-[24px] min-w-[46px]"
          />
        </ActionMenu.Trigger>
        <ActionMenu.Content>
          {hasActiveWorkspace
            ? selectedWorkspaceMenuItems.map((item) => renderActionMenuItem(item, handleWorkspaceMenuAction))
            : workspaceMenuItems.map((item) => renderActionMenuItem(item, handleWorkspaceMenuAction))}
        </ActionMenu.Content>
      </ActionMenu.Root>
    </div>
  );
};
