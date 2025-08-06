import { useContext } from "react";

import { TreeContext } from "./Tree";

export const DirDepthIndicator = ({ depth }: { depth: number }) => {
  const { nodeOffset } = useContext(TreeContext);

  const iconSize = 16;
  const left = depth * nodeOffset + iconSize + 1;

  return <div className="background-(--moss-divider-color) absolute top-0 z-5 h-full w-px" style={{ left }} />;
};
