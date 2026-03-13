interface DirDepthIndicatorProps {
  offset?: number;
}
export const DirDepthIndicator = ({ offset }: DirDepthIndicatorProps) => {
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
        group-hover/TreeRoot:flex 
        group-hover/TreeRoot:opacity-100
      `}
      style={{ left: offset }}
    />
  );
};
