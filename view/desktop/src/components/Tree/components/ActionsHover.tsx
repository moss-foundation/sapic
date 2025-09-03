import { cn } from "@/utils";

interface ActionsHoverProps {
  children: React.ReactNode;
  className?: string;
  forceVisible?: boolean;
  props?: React.HTMLAttributes<HTMLDivElement>;
}

export const ActionsHover = ({ children, className, forceVisible, ...props }: ActionsHoverProps) => {
  return (
    <div
      className={cn(
        "transition-[display,opacity] transition-discrete duration-100",
        "hidden group-hover/TreeNodeControls:contents group-hover/TreeRootNodeControls:contents",
        "opacity-0 group-hover/TreeNodeControls:opacity-100 group-hover/TreeRootNodeControls:opacity-100",
        {
          "contents opacity-100": forceVisible,
        },
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};
