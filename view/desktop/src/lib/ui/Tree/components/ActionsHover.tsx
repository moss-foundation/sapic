import { cva } from "class-variance-authority";

import { cn } from "@/utils";

interface HoverActionsProps {
  children: React.ReactNode;
  className?: string;
  forceVisible?: boolean;
  props?: React.HTMLAttributes<HTMLDivElement>;
  invisible?: boolean;
}

const hoverActions = cva(["transition-[display,opacity] transition-discrete duration-100"], {
  variants: {
    invisible: {
      false: ["hidden group-hover/TreeNodeControls:contents group-hover/TreeRootNodeControls:contents"],
      true: ["opacity-0 group-hover/TreeNodeControls:opacity-100 group-hover/TreeRootNodeControls:opacity-100"],
    },
  },
});

export const HoverActions = ({ children, className, forceVisible, invisible = false, ...props }: HoverActionsProps) => {
  return (
    <div
      className={cn(
        hoverActions({ invisible }),
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
