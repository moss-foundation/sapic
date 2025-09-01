interface TreeRootNodeControlsProps {
  children: React.ReactNode;
}

export const TreeRootNodeControls = ({ children }: TreeRootNodeControlsProps) => {
  return <div className="flex w-full min-w-0 items-center justify-between">{children}</div>;
};
