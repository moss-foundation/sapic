import { HTMLAttributes } from "react";

interface TreeRootNodeLabelProps extends HTMLAttributes<HTMLDivElement> {
  label: string;
}

export const TreeRootNodeLabel = ({ label, ...props }: TreeRootNodeLabelProps) => {
  return (
    <div className="w-max truncate font-medium" {...props}>
      {label}
    </div>
  );
};
