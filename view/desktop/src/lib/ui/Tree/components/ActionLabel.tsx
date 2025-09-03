import { cn } from "@/utils";

interface ActionLabelProps {
  children: React.ReactNode;
  className?: string;
  props?: React.HTMLAttributes<HTMLDivElement>;
}

export const ActionLabel = ({ children, className, ...props }: ActionLabelProps) => {
  return (
    <div
      //TODO change background color to a variable
      className={cn("background-(--moss-gray-11) rounded-[3px] px-1 text-xs leading-4 text-black", className)}
      {...props}
    >
      {children}
    </div>
  );
};
