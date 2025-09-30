import SimpleBar, { type Props as SimpleBarProps } from "simplebar-react";

import { cn } from "@/utils";

export const Scrollbar = ({ children, className, classNames, ...props }: SimpleBarProps) => {
  //Scrollbar is sensitive to whitespace in classNames, so we need to trim them
  const trimmedClassNames = { ...classNames };
  for (const key in trimmedClassNames) {
    trimmedClassNames[key] = trimmedClassNames[key].trim();
  }

  return (
    <SimpleBar
      classNames={{
        contentEl: "w-full h-full",
        ...trimmedClassNames,
      }}
      className={cn("h-full w-full", className)}
      {...props}
    >
      {children}
    </SimpleBar>
  );
};

export default Scrollbar;
