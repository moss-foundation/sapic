import { useEffect } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../constants";
import { handleReorderProjects } from "../handlers/handleReorderProjects";

export const useMonitorProjectRootNodes = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => {
        return source.data.type === ProjectDragType.ROOT_NODE;
      },
      onDrop: ({ location, source }) => handleReorderProjects({ location, source, currentWorkspaceId }),
    });
  }, [currentWorkspaceId]);
};
