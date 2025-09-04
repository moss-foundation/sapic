import { cn } from "@/utils";

interface ActionsPersistentProps {
  children: React.ReactNode;
  className?: string;
  props?: React.HTMLAttributes<HTMLDivElement>;
}

export const ActionsPersistent = ({ children, className, ...props }: ActionsPersistentProps) => {
  return (
    <div className={cn("contents", className)} {...props}>
      {children}
    </div>
  );
};
