import { ActionLabel } from "./components/ActionLabel";
import { ActionsHover } from "./components/ActionsHover";
import { ActionsPersistent } from "./components/ActionsPersistent";
import { RootNode } from "./components/RootNode/RootNode";
import { TreeRootNodeActions } from "./components/RootNode/TreeRootNodeActions";
import { TreeRootNodeChildren } from "./components/RootNode/TreeRootNodeChildren";
import { TreeRootNodeControls } from "./components/RootNode/TreeRootNodeControls";
import { TreeRootNodeHeader } from "./components/RootNode/TreeRootNodeHeader";
import { TreeRootNodeIcon } from "./components/RootNode/TreeRootNodeIcon";
import { TreeRootNodeLabel } from "./components/RootNode/TreeRootNodeLabel";
import { TreeRootNodeOrder } from "./components/RootNode/TreeRootNodeOrder";
import { TreeRootNodeTriggers } from "./components/RootNode/TreeRootNodeTriggers";
import { Tree } from "./Tree";

interface TreeWithSubcomponents {
  (props: React.ComponentProps<typeof Tree>): React.ReactElement;
  RootNode: typeof RootNode;
  RootNodeActions: typeof TreeRootNodeActions;
  RootNodeChildren: typeof TreeRootNodeChildren;
  RootNodeControls: typeof TreeRootNodeControls;
  RootNodeHeader: typeof TreeRootNodeHeader;
  RootNodeIcon: typeof TreeRootNodeIcon;
  RootNodeLabel: typeof TreeRootNodeLabel;
  RootNodeOrder: typeof TreeRootNodeOrder;
  RootNodeTriggers: typeof TreeRootNodeTriggers;

  PersistentActions: typeof ActionsPersistent;
  HoverActions: typeof ActionsHover;
  ActionLabel: typeof ActionLabel;
}

const TreeWithSubs = Tree as TreeWithSubcomponents;

TreeWithSubs.RootNode = RootNode;
TreeWithSubs.RootNodeActions = TreeRootNodeActions;
TreeWithSubs.RootNodeChildren = TreeRootNodeChildren;
TreeWithSubs.RootNodeControls = TreeRootNodeControls;
TreeWithSubs.RootNodeHeader = TreeRootNodeHeader;
TreeWithSubs.RootNodeIcon = TreeRootNodeIcon;
TreeWithSubs.RootNodeLabel = TreeRootNodeLabel;
TreeWithSubs.RootNodeOrder = TreeRootNodeOrder;
TreeWithSubs.RootNodeTriggers = TreeRootNodeTriggers;

TreeWithSubs.PersistentActions = ActionsPersistent;
TreeWithSubs.HoverActions = ActionsHover;
TreeWithSubs.ActionLabel = ActionLabel;

export { TreeWithSubs as Tree };
