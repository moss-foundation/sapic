import { cn } from "@/utils";

export const PageWrapper = ({ children, className }: { children: React.ReactNode; className?: string }) => {
  return <div className={cn("px-5 pt-3 pb-5", className)}>{children}</div>;
};
