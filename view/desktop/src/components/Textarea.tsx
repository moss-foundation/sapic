import { TextareaHTMLAttributes } from "react";

import { cn } from "@/utils";

function Textarea({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return (
    <textarea
      // prettier-ignore
      className={cn(`
        background-(--moss-controls-background) 
        w-full resize-none rounded-sm 
        border border-(--moss-controls-border) 
        px-1.5 py-1 
        placeholder:text-(--moss-controls-placeholder) 
        focus-visible:border-(--moss-accent) 
        focus-visible:outline-none`,

        className
      )}
      rows={7}
      placeholder="Commit message"
      {...props}
    />
  );
}

export { Textarea };
