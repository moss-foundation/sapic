import { cva } from "class-variance-authority";

import { cn } from "@/utils";

interface ActionsHoverProps {
  children: React.ReactNode;
  className?: string;
  forceVisible?: boolean;
  props?: React.HTMLAttributes<HTMLDivElement>;
  invisible?: boolean;
}

const actionsHover = cva(["transition-[display,opacity] transition-discrete duration-100"], {
  variants: {
    invisible: {
      false: ["hidden group-hover/TreeNodeControls:contents group-hover/TreeRootNodeControls:contents"],
      true: ["opacity-0 group-hover/TreeNodeControls:opacity-100 group-hover/TreeRootNodeControls:opacity-100"],
    },
  },
});

export const ActionsHover = ({ children, className, forceVisible, invisible = false, ...props }: ActionsHoverProps) => {
  return (
    <div
      className={cn(
        actionsHover({ invisible }),
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
