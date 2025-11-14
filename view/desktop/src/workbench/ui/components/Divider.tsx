import { cn } from "@/utils";

interface DividerProps {
  className?: string;
}

export const Divider = ({ className = "" }: DividerProps) => {
  return (
    <div className={cn("mx-1 flex h-full items-center", className)}>
      <div className={`background-(--moss-border) h-full w-[1px]`}></div>
    </div>
  );
};
