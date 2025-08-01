import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { EntryInfo } from "@repo/moss-collection";

import { TreeCollectionNode } from "./CollectionTree/types";

interface EntryIconProps {
  entry: TreeCollectionNode | EntryInfo;
  className?: string;
}

const defaultProtocolClassName = "text-xs min-w-[30px] text-left uppercase" as const;

export const EntryIcon = ({ entry, className }: EntryIconProps) => {
  const calculateIsRoot = entry.path.segments.length === 1;

  if (calculateIsRoot) {
    switch (entry.class) {
      case "Schema":
        return <Icon icon="SchemasFolder" className={className} />;
      case "Endpoint":
        return <Icon icon="EndpointsFolder" className={className} />;
      case "Component":
        return <Icon icon="ComponentsFolder" className={className} />;
      default:
        return <Icon icon="RequestsFolder" className={className} />;
    }
  }

  if (entry.kind === "Dir") {
    return <Icon icon="Folder" className={className} />;
  }

  const rnd = randomIntFromInterval(1, 4);

  switch (rnd) {
    case 1:
      return <span className={cn(defaultProtocolClassName, "text-(--moss-green-4)", className)}>Get</span>;
    case 2:
      return <span className={cn(defaultProtocolClassName, "text-(--moss-orange-4)", className)}>Post</span>;
    case 3:
      return <span className={cn(defaultProtocolClassName, "text-(--moss-blue-4)", className)}>Put</span>;
    case 4:
      return <span className={cn(defaultProtocolClassName, "text-(--moss-red-4)", className)}>Del</span>;

    default:
      return <span className={cn(defaultProtocolClassName, "text-(--moss-gray-4)", className)}>{entry.protocol}</span>;
  }
};

function randomIntFromInterval(min: number, max: number) {
  return Math.floor(Math.random() * (max - min + 1) + min);
}
