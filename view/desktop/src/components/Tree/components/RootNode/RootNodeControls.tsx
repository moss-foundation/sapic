interface RootNodeControlsProps {
  children: React.ReactNode;
}

export const RootNodeControls = ({ children }: RootNodeControlsProps) => {
  return <div className="group/TreeRootNodeControls flex w-full min-w-0 items-center justify-between">{children}</div>;
};
