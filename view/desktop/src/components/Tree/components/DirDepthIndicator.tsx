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
        background-(--moss-divider-color) 

        transition-[display,opacity] transition-discrete duration-100
        hidden opacity-0
        group-hover/Tree:flex 
        group-hover/Tree:opacity-100
      `}
      style={{ left }}
    />
  );
};
