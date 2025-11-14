import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { StreamResourcesEvent } from "@repo/moss-project";

import { ProjectTreeNode } from "./ProjectTree/types";

interface ResourceIconProps {
  resource?: ProjectTreeNode | StreamResourcesEvent;
  className?: string;
}

const defaultProtocolClassName = "text-xs right-0 uppercase absolute top-1/2 -translate-y-1/2   " as const;

export const ResourceIcon = ({ resource, className }: ResourceIconProps) => {
  if (!resource) return <div className={cn("size-[18px] shrink-0 opacity-0", className)} />;

  if (resource.kind === "Dir") {
    return <Icon icon="Folder" className={cn("size-[18px]", className)} />;
  }
  if (resource.kind === "Item") {
    return <Icon icon="Http" className={cn("size-[18px]", className)} />;
  }

  return (
    <div className="relative">
      <span className={cn(defaultProtocolClassName, "text-(--moss-gray-4) size-[18px] shrink-0", className)}>
        {resource.protocol}
      </span>
    </div>
  );
};
