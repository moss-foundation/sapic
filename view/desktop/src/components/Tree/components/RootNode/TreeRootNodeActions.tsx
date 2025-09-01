import { cn } from "@/utils";

interface TreeRootNodeActionsProps {
  children: React.ReactNode;
  className?: string;
}

export const TreeRootNodeActions = ({ children, className, ...props }: TreeRootNodeActionsProps) => {
  return (
    <div className={cn("z-10 flex items-center gap-1", className)} {...props}>
      {children}
    </div>
  );
};
