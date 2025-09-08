import { ComponentProps } from "react";

interface TreeProps extends ComponentProps<"div"> {
  paddingLeft?: number;
  paddingRight?: number;
  children: React.ReactNode;
}

export const Tree = ({ children, paddingLeft, paddingRight, ...props }: TreeProps) => {
  return (
    <div style={{ paddingLeft, paddingRight }} {...props}>
      {children}
    </div>
  );
};
