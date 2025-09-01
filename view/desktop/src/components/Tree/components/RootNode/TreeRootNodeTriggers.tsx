import { cn } from "@/utils/cn";

interface TreeRootNodeTriggersProps {
  children: React.ReactNode;
}

export const TreeRootNodeTriggers = ({ children }: TreeRootNodeTriggersProps) => {
  return <div className={cn("flex grow items-center gap-1 overflow-hidden bg-amber-300")}>{children}</div>;
};
