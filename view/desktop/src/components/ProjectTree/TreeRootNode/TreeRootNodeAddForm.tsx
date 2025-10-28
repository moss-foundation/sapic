import { useContext } from "react";

import { Icon } from "@/lib/ui/Icon";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { ResourceIcon } from "../../ResourceIcon";
import { ProjectTreeContext } from "../ProjectTreeContext";

interface TreeRootNodeAddFormProps {
  handleAddFormRootSubmit: (name: string) => void;
  handleAddFormRootCancel: () => void;
  restrictedNames: string[];
}

export const TreeRootNodeAddForm = ({
  handleAddFormRootSubmit,
  handleAddFormRootCancel,
  restrictedNames,
}: TreeRootNodeAddFormProps) => {
  const { nodeOffset } = useContext(ProjectTreeContext);
  return (
    <div className="flex w-full min-w-0 items-center gap-1.5 py-0.5" style={{ paddingLeft: nodeOffset * 2 }}>
      <Icon icon="ChevronRight" className={cn("shrink-0 opacity-0")} />
      <ResourceIcon />
      <Tree.NodeAddForm
        onSubmit={handleAddFormRootSubmit}
        onCancel={handleAddFormRootCancel}
        restrictedNames={restrictedNames}
      />
    </div>
  );
};
