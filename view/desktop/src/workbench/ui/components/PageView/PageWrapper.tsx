import { cn } from "@/utils";

export const PageWrapper = ({ children, className }: { children: React.ReactNode; className?: string }) => {
  return <div className={cn("px-3.5 pb-3.5 pt-2.5", className)}>{children}</div>;
};
