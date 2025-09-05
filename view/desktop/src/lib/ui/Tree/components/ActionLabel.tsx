import { cn } from "@/utils";

interface ActionLabelProps {
  children: React.ReactNode;
  className?: string;
  props?: React.HTMLAttributes<HTMLDivElement>;
}

export const ActionLabel = ({ children, className, ...props }: ActionLabelProps) => {
  return (
    <div
      className={cn(
        "background-(--moss-secondary-background-active) rounded-[3px] px-1 text-xs leading-4 text-(--moss-primary-text)",
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};
