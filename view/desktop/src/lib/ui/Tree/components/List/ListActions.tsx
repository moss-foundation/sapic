import { cn } from "@/utils";

interface ListActionsProps {
  className?: string;
  children: React.ReactNode;
}

export const ListActions = ({ className, children, ...props }: ListActionsProps) => {
  return (
    <div className={cn("z-10 flex items-center justify-end gap-1", className)} {...props}>
      {children}
    </div>
  );
};
