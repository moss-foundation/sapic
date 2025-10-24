import { cn } from "@/utils";

interface ActionLabelProps {
  children: React.ReactNode;
  className?: string;
  props?: React.HTMLAttributes<HTMLDivElement>;
}

export const ActionLabel = ({ children, className, ...props }: ActionLabelProps) => {
  return (
    <div className={cn("text-(--moss-primary-foreground) rounded-[3px] px-1 text-xs leading-4", className)} {...props}>
      {children}
    </div>
  );
};
