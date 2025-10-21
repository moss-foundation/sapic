import { useContext } from "react";

import { TreeContext } from "./TreeContext";

export const DirDepthIndicator = ({ depth }: { depth: number }) => {
  const { nodeOffset } = useContext(TreeContext);

  const iconSize = 16;
  const left = depth * nodeOffset + iconSize + 1;

  return (
    <div
      //prettier-ignore
      className={`
        absolute top-0  
        h-full w-px 
        z-5 
        background-(--moss-border) 
        transition-[display,opacity] transition-discrete duration-100
        hidden opacity-0
        group-hover/TreeRootNode:flex 
        group-hover/TreeRootNode:opacity-100
      `}
      style={{ left }}
    />
  );
};
