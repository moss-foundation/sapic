import { RefObject, useEffect, useState } from "react";

import { isSourceResourceNode } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/validation/isSourceResourceTreeNode.ts";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

interface UseToggleProjectCreationZoneProps {
  ref: RefObject<HTMLDivElement | null>;
}

export const useToggleProjectCreationZone = ({ ref }: UseToggleProjectCreationZoneProps) => {
  const [showProjectCreationZone, setShowProjectCreationZone] = useState<boolean>(false);

  useEffect(() => {
    if (!ref.current) return;
    const element = ref.current;

    return dropTargetForElements({
      element,
      canDrop({ source }) {
        return isSourceResourceNode(source);
      },
      onDragStart() {
        setShowProjectCreationZone(true);
      },
      onDragEnter() {
        setShowProjectCreationZone(true);
      },
      onDragLeave() {
        setShowProjectCreationZone(false);
      },
      onDrop() {
        setShowProjectCreationZone(false);
      },
    });
  }, [ref]);

  return {
    showProjectCreationZone,
  };
};
