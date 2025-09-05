import { cn } from "@/utils";

interface RootNodeActionsProps {
  children: React.ReactNode;
  className?: string;
}

export const RootNodeActions = ({ children, className, ...props }: RootNodeActionsProps) => {
  return (
    <div className={cn("z-10 flex items-center justify-end gap-1", className)} {...props}>
      {children}
    </div>
  );
};
