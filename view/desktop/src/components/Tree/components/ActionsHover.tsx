import { cn } from "@/utils";

interface ActionsHoverProps {
  children: React.ReactNode;
  className?: string;
  props?: React.HTMLAttributes<HTMLDivElement>;
}

export const ActionsHover = ({ children, className, ...props }: ActionsHoverProps) => {
  return (
    <div
      className={cn(
        "hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover/TreeRootNode:contents group-hover/TreeRootNode:opacity-100",
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};
