import { cn } from "@/utils";

interface CounterProps {
  count: number;
  color?: "gray" | "primary";
  className?: string;
}

export const Counter = ({ count, color = "primary", className }: CounterProps) => {
  return (
    <span
      className={cn(
        "flex size-4 shrink-0 items-center justify-center rounded-full text-xs leading-2.5",
        {
          "background-(--moss-icon-primary-background) text-black": color === "gray",
          "background-(--moss-primary) text-white": color === "primary",
        },
        className
      )}
    >
      {count}
    </span>
  );
};
