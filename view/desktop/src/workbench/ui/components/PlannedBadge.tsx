import { cva } from "class-variance-authority";
import { HTMLAttributes } from "react";

interface PlannedBadgeProps extends HTMLAttributes<HTMLDivElement> {
  variant?: "default" | "outlined";
}

const badgeStyles = cva("rounded-full px-1.5 py-px text-sm leading-4", {
  variants: {
    variant: {
      default: "background-(--moss-purple-8) text-(--moss-purple-2)",
      outlined: "border-(--moss-purple-2) text-(--moss-purple-2) border",
    },
  },
});

export const PlannedBadge = ({ variant = "default", className, ...props }: PlannedBadgeProps) => {
  return (
    <div className={badgeStyles({ variant, className })} {...props}>
      Planned
    </div>
  );
};
