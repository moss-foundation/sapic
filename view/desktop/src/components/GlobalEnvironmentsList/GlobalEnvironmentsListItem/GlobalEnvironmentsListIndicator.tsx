import { cn } from "@/utils/cn";

interface GlobalEnvironmentsListIndicatorProps {
  isActive?: boolean;
}

export const GlobalEnvironmentsListIndicator = ({ isActive }: GlobalEnvironmentsListIndicatorProps) => {
  return (
    <div
      className={cn(
        "group-hover/GlobalEnvironmentsList:background-(--moss-secondary-background-hover) absolute top-0 left-0 z-0 h-full w-full",
        {
          "background-(--moss-secondary-background-hover) border-l border-(--moss-primary)": isActive,
        }
      )}
    />
  );
};
