import { TextareaHTMLAttributes } from "react";

import { cn } from "@/utils";

function Textarea({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return (
    <textarea
      // prettier-ignore
      className={cn(`
        background-(--moss-textarea-bg) 
        w-full resize-none rounded-sm 
        border border-(--moss-textarea-border) 
        px-1.5 py-1 
        placeholder:text-(--moss-textarea-placeholder) 
        focus-visible:border-(--moss-primary) 
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
