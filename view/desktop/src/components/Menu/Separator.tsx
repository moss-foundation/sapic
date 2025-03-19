import { ComponentPropsWithoutRef } from "react";

export const Separator = (props: ComponentPropsWithoutRef<"div">) => {
  return <div className="background-(--moss-menu-separator-bg) my-1 h-px w-full" {...props} />;
};
