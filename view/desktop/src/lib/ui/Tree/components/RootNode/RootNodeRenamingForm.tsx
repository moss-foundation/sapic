import { useContext } from "react";

import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";

import { RenamingForm } from "../RenamingForm";
import { TreeContext } from "../TreeContext";

interface RootNodeRenameFormProps {
  name: string;
  shouldRenderChildNodes: boolean;
  restrictedNames: string[];
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

export const RootNodeRenameForm = ({
  name,
  shouldRenderChildNodes,
  restrictedNames,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: RootNodeRenameFormProps) => {
  const { iconPath } = useContext(TreeContext);

  return (
    <div className="flex grow cursor-pointer items-center gap-1.5">
      <div className="flex size-5 shrink-0 items-center justify-center rounded">
        {iconPath ? (
          <img src={iconPath} className="h-full w-full" />
        ) : (
          <span className="flex size-5 shrink-0 items-center justify-center">
            <button className="flex cursor-pointer items-center justify-center rounded-full">
              <Icon icon="ChevronRight" className={cn({ "rotate-90": shouldRenderChildNodes })} />
            </button>
          </span>
        )}
      </div>

      <RenamingForm
        onSubmit={handleRenamingFormSubmit}
        onCancel={handleRenamingFormCancel}
        currentName={name}
        restrictedNames={restrictedNames}
      />
    </div>
  );
};
