import { cn } from "@/utils";

interface RoundedCounterProps {
  count: number;
  color?: "gray" | "primary";
  className?: string;
}

export const RoundedCounter = ({ count, color = "primary", className }: RoundedCounterProps) => {
  return (
    <span
      className={cn(
        "flex size-4 shrink-0 items-center justify-center rounded-full text-xs leading-2.5",
        {
          "background-(--moss-secondary-background) text-(--moss-secondary-foreground)": color === "gray",
          "background-(--moss-accent) text-white": color === "primary",
        },
        className
      )}
    >
      {count}
    </span>
  );
};
