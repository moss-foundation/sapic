import { cn } from "@/utils";

interface RootActionsProps {
  children: React.ReactNode;
  className?: string;
}

export const RootActions = ({ children, className, ...props }: RootActionsProps) => {
  return (
    <div className={cn("z-10 flex items-center justify-end gap-1", className)} {...props}>
      {children}
    </div>
  );
};
