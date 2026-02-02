import { useCurrentWorkspace } from "@/hooks";
import { useGetLayout } from "@/workbench/adapters";
import {
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
  TREE_VIEW_GROUP_PROJECTS,
} from "@repo/moss-workspace";

import { SidebarHeader } from "./SidebarHeader";
import { EnvironmentsListView } from "./views/EnvironmentsListView";
import { ProjectTreesView } from "./views/ProjectTreesView";
import { SourceControlView } from "./views/SourceControlView";

export const SidebarContent = () => {
  const { currentWorkspace } = useCurrentWorkspace();
  const { data: layout } = useGetLayout();

  const activeContainerId = layout?.activitybarState.activeContainerId;

  if (!currentWorkspace) {
    return (
      <div className="flex h-full w-full items-center justify-center p-4">
        <div className="text-center">
          <p className="text-sm text-gray-500">The workspace is not found</p>
        </div>
      </div>
    );
  }

  switch (activeContainerId) {
    case TREE_VIEW_GROUP_PROJECTS:
      return (
        <div className="flex h-full flex-col">
          <ProjectTreesView />
        </div>
      );

    case TREE_VIEW_GROUP_ENVIRONMENTS:
      return (
        <div className="flex h-full flex-col">
          <EnvironmentsListView />
        </div>
      );

    case TREE_VIEW_GROUP_MOCK_SERVERS:
      return (
        <div className="flex h-full flex-col">
          <SidebarHeader />
          <div className="flex grow items-center justify-center p-2">
            <p className="mt-2 text-center text-sm text-gray-500">Under construction</p>
          </div>
        </div>
      );

    case "workbench.view.commit":
      return (
        <div className="flex h-full flex-col">
          <SourceControlView />
        </div>
      );

    default:
      return (
        <div className="flex h-full flex-col">
          <div className="p-4">
            <h3 className="text-lg font-semibold">No content</h3>
            <p className="mt-2 text-sm text-gray-500">No content for this group, showing default view</p>
            <div>{activeContainerId}</div>
          </div>
        </div>
      );
  }
};

export default SidebarContent;
