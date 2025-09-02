import { ActionLabel } from "./components/ActionLabel";
import { ActionsHover } from "./components/ActionsHover";
import { ActionsPersistent } from "./components/ActionsPersistent";
import { NodeAddForm } from "./components/NodeAddForm";
import { RootNode } from "./components/RootNode/RootNode";
import { RootNodeActions } from "./components/RootNode/RootNodeActions";
import { RootNodeChildren } from "./components/RootNode/RootNodeChildren";
import { RootNodeControls } from "./components/RootNode/RootNodeControls";
import { RootNodeHeader } from "./components/RootNode/RootNodeHeader";
import { RootNodeIcon } from "./components/RootNode/RootNodeIcon";
import { RootNodeLabel } from "./components/RootNode/RootNodeLabel";
import { RootNodeOrder } from "./components/RootNode/RootNodeOrder";
import { RootNodeRenameForm } from "./components/RootNode/RootNodeRenamingForm";
import { RootNodeTriggers } from "./components/RootNode/RootNodeTriggers";
import { Tree } from "./Tree";

interface TreeWithSubcomponents {
  (props: React.ComponentProps<typeof Tree>): React.ReactElement;
  RootNode: typeof RootNode;
  RootNodeActions: typeof RootNodeActions;
  RootNodeChildren: typeof RootNodeChildren;
  RootNodeControls: typeof RootNodeControls;
  RootNodeHeader: typeof RootNodeHeader;
  RootNodeIcon: typeof RootNodeIcon;
  RootNodeLabel: typeof RootNodeLabel;
  RootNodeOrder: typeof RootNodeOrder;
  RootNodeTriggers: typeof RootNodeTriggers;
  RootNodeRenameForm: typeof RootNodeRenameForm;

  NodeAddForm: typeof NodeAddForm;

  PersistentActions: typeof ActionsPersistent;
  HoverActions: typeof ActionsHover;
  ActionLabel: typeof ActionLabel;
}

const TreeWithSubs = Tree as TreeWithSubcomponents;

TreeWithSubs.RootNode = RootNode;
TreeWithSubs.RootNodeActions = RootNodeActions;
TreeWithSubs.RootNodeChildren = RootNodeChildren;
TreeWithSubs.RootNodeControls = RootNodeControls;
TreeWithSubs.RootNodeHeader = RootNodeHeader;
TreeWithSubs.RootNodeIcon = RootNodeIcon;
TreeWithSubs.RootNodeLabel = RootNodeLabel;
TreeWithSubs.RootNodeOrder = RootNodeOrder;
TreeWithSubs.RootNodeTriggers = RootNodeTriggers;
TreeWithSubs.RootNodeRenameForm = RootNodeRenameForm;
TreeWithSubs.NodeAddForm = NodeAddForm;
TreeWithSubs.PersistentActions = ActionsPersistent;
TreeWithSubs.HoverActions = ActionsHover;
TreeWithSubs.ActionLabel = ActionLabel;

export { TreeWithSubs as Tree };
