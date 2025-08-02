import { cn } from "@/utils";

export const ActiveNodeIndicator = ({ isActive }: { isActive: boolean }) => {
  return (
    <div
      //prettier-ignore
      className={cn(`
          absolute top-0 left-0 
          h-full w-full 
          z-5
          hover:background-(--moss-secondary-background-hover) 
          pointer-events-none
        `,
        {
          "background-(--moss-secondary-background-hover) border-l border-l-(--moss-primary)": isActive,
        }
      )}
    />
  );
};
