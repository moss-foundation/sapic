import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { StreamEntriesEvent } from "@repo/moss-project";

import { ProjectTreeNode } from "./ProjectTree/types";

interface EntryIconProps {
  entry?: ProjectTreeNode | StreamEntriesEvent;
  className?: string;
}

const defaultProtocolClassName = "text-xs right-0 uppercase absolute top-1/2 -translate-y-1/2   " as const;

export const EntryIcon = ({ entry, className }: EntryIconProps) => {
  if (!entry) return <div className={cn("size-[18px] shrink-0 opacity-0", className)} />;

  if (entry.kind === "Dir") {
    return <Icon icon="Folder" className={cn("size-[18px]", className)} />;
  }
  if (entry.kind === "Item") {
    return <Icon icon="Http" className={cn("size-[18px]", className)} />;
  }

  return (
    <div className="relative">
      <span className={cn(defaultProtocolClassName, "size-[18px] shrink-0 text-(--moss-gray-4)", className)}>
        {entry.protocol}
      </span>
    </div>
  );
};
