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
  if (!entry) return <div className={cn("size-4 shrink-0 opacity-0", className)} />;

  if (entry.kind === "Dir") {
    return <Icon icon="Folder" className={className} />;
  }
  if (entry.kind === "Item") {
    return <Icon icon="Http" className={className} />;
  }

  return (
    <div className="relative size-4">
      <span className={cn(defaultProtocolClassName, "text-(--moss-gray-4)", className)}>{entry.protocol}</span>
    </div>
  );
};
