import { HTMLAttributes, ReactNode, RefObject } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { CombineDNDIndicator } from "../CombineDNDIndicator";

interface ListProps extends HTMLAttributes<HTMLDivElement> {
  combineInstruction?: Instruction | null;
  childNodeHasBlockedOperation?: boolean;

  ref?: RefObject<HTMLDivElement | null>;
  children: ReactNode;
  className?: string;
}

export const List = ({
  children,
  className,
  ref,
  combineInstruction,
  childNodeHasBlockedOperation,
  ...props
}: ListProps) => {
  return (
    <section className={cn("relative", className)} ref={ref} {...props}>
      {combineInstruction && <CombineDNDIndicator combineInstruction={combineInstruction} />}

      {childNodeHasBlockedOperation && (
        <CombineDNDIndicator
          combineInstruction={{
            operation: "combine",
            blocked: true,
            axis: "vertical",
          }}
        />
      )}
      {children}
    </section>
  );
};
