import { ComponentProps } from "react";

import { cn } from "@/utils";

import { HeadBar } from "../parts/HeadBar/HeadBar";
import StatusBar from "../parts/StatusBar/StatusBar";

export const RootLayout = ({ children, className, ...props }: ComponentProps<"main">) => {
  return (
    <div className="grid h-full grid-rows-[minmax(0px,33px)_1fr_auto] select-none">
      <HeadBar />

      <main className={cn(className)} {...props}>
        {children}
      </main>

      <StatusBar className="h-[29px] w-full" />
    </div>
  );
};
