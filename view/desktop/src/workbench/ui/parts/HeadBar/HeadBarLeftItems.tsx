import { useActiveEnvironments } from "@/adapters/tanstackQuery/environment/derived/useActiveEnvironments";
import { useStreamedProjectsWithResources } from "@/adapters/tanstackQuery/project";
import { useCurrentWorkspace } from "@/hooks";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu, IconLabelButton } from "@/workbench/ui/components";
import { renderActionMenuItem } from "@/workbench/utils/renderActionMenuItem";

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

  const { currentWorkspace } = useCurrentWorkspace();
  const { selectedWorkspaceMenuItems } = useWorkspaceMenu();
  const { data: streamedProjectsWithResources } = useStreamedProjectsWithResources();
  const { activeGlobalEnvironment, activeProjectEnvironments } = useActiveEnvironments();
  const { activePanelId } = useTabbedPaneStore();

  const activeProject = streamedProjectsWithResources?.find((project) =>
    project.resources.some((resource) => resource.id === activePanelId)
  );

  const currentProjectEnvironment = activeProjectEnvironments.find(
    (environment) => environment.projectId === activeProject?.id
  );

  return (
    <div className={cn("flex items-center justify-start gap-[6px] overflow-hidden")} data-tauri-drag-region>
      {isWindowsOrLinux && (
        <ActionMenu.Root>
          <ActionMenu.Trigger className="hover:background-(--moss-toolbarItem-background-hover) rounded p-1">
            <Icon icon="WindowsMenu" className="size-4.5 cursor-pointer" />
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            {windowsMenuItems.map((item) => renderActionMenuItem(item, handleWindowsMenuAction))}
          </ActionMenu.Content>
        </ActionMenu.Root>
      )}

      <NavigationButtons onBack={() => {}} onForward={() => {}} canGoBack={true} canGoForward={true} />

      <div className="flex items-center">
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <IconLabelButton title={currentWorkspace?.name} placeholder="No workspace selected" />
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            {selectedWorkspaceMenuItems.map((item) => renderActionMenuItem(item, handleWorkspaceMenuAction))}
          </ActionMenu.Content>
        </ActionMenu.Root>

        <Icon icon="ChevronRight" />

        <IconLabelButton title={activeGlobalEnvironment?.name} placeholder="No environment" />

        <Icon icon="ChevronRight" />

        <IconLabelButton title={currentProjectEnvironment?.name} placeholder="No environment" />
      </div>
    </div>
  );
};
