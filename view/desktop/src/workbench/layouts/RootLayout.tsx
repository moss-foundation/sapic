import { ComponentProps } from "react";

import { cn } from "@/utils";

import { HeadBar } from "../ui/parts/HeadBar/HeadBar";
import StatusBar from "../ui/parts/StatusBar/StatusBar";

export const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <div className="text-(--moss-primary-foreground) grid h-full select-none grid-rows-[minmax(33px,min-content)_1fr_29px]">
      <HeadBar />

      <main className={cn(className)} {...props}>
        {children}
      </main>

      <StatusBar className="w-full" />
    </div>
  );
};
