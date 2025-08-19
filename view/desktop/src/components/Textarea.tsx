import { TextareaHTMLAttributes } from "react";

import { cn } from "@/utils";

function Textarea({ className, ...props }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return (
    <textarea
      className={cn(
        "background-(--moss-gray-14) w-full resize-none rounded-sm border border-(--moss-gray-9) px-1.5 py-1 placeholder:text-(--moss-gray-6) focus-visible:border-(--moss-blue-5) focus-visible:outline-none",
        className
      )}
      rows={7}
      placeholder="Commit message"
      {...props}
    />
  );
}

export { Textarea };
