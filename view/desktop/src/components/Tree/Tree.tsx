import { TreeContext, TreeContextProps } from "./components/TreeContext";

interface TreeProps extends Partial<TreeContextProps> {
  children: React.ReactNode;
}

export const Tree = ({
  children,
  id = "",
  iconPath = "",
  treePaddingLeft = 8,
  treePaddingRight = 8,
  nodeOffset = 12,
  allFoldersAreExpanded = false,
  allFoldersAreCollapsed = false,
  displayMode = "REQUEST_FIRST",
  showNodeOrders = false,
}: TreeProps) => {
  return (
    <TreeContext.Provider
      value={{
        id,
        iconPath,
        treePaddingLeft,
        treePaddingRight,
        nodeOffset,
        allFoldersAreExpanded,
        allFoldersAreCollapsed,
        displayMode,
        showNodeOrders,
      }}
    >
      {children}
    </TreeContext.Provider>
  );
};
