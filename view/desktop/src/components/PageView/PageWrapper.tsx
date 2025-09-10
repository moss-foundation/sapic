import { cn } from "@/utils";

export const PageWrapper = ({ children, className }: { children: React.ReactNode; className?: string }) => {
  return <div className={cn("px-5 py-3", className)}>{children}</div>;
};
