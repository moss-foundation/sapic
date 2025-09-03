import { cn } from "@/utils";

interface NodeActionsProps {
  className?: string;
  children: React.ReactNode;
}

export const NodeActions = ({ className, children, ...props }: NodeActionsProps) => {
  return (
    <div className={cn("flex items-center gap-0.5", className)} {...props}>
      {children}
    </div>
  );
};
