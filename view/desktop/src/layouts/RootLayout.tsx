import { ComponentProps } from "react";

import { cn } from "@/utils";

import { HeadBar } from "../parts/HeadBar/HeadBar";
import StatusBar from "../parts/StatusBar/StatusBar";

export const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <div className="grid h-full grid-rows-[minmax(33px,min-content)_1fr_29px] text-(--moss-primary-foreground) select-none">
      <HeadBar />

      <main className={cn(className)} {...props}>
        {children}
      </main>

      <StatusBar className="w-full" />
    </div>
  );
};
