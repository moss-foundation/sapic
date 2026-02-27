interface DirDepthIndicatorProps {
  offset?: number;
}
export const DirDepthIndicator = ({ offset }: DirDepthIndicatorProps) => {
  //TODO add treePaddingLeft and nodeOffset to the props
  //const { treePaddingLeft } = useContext(ProjectTreeContext);
  // const { nodeOffset } = useContext(TreeContext);

  //const iconSize = 16;
  //const left = depth * nodeOffset + treePaddingLeft + iconSize + 1;

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
      style={{ left: offset }}
    />
  );
};
