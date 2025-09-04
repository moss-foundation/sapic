import { ActionLabel } from "./components/ActionLabel";
import { ActionsHover } from "./components/ActionsHover";
import { ActionsPersistent } from "./components/ActionsPersistent";
import { Node } from "./components/Node/Node";
import { NodeActions } from "./components/Node/NodeActions";
import { NodeChildren } from "./components/Node/NodeChildren";
import { NodeControls } from "./components/Node/NodeControls";
import { NodeDirCount } from "./components/Node/NodeDirCount";
import { NodeDirToggleIcon } from "./components/Node/NodeDirToggleIcon";
import { NodeLabel } from "./components/Node/NodeLabel";
import { NodeOrder } from "./components/Node/NodeOrders";
import { NodeTriggers } from "./components/Node/NodeTriggers";
import { NodeAddForm } from "./components/NodeAddForm";
import { NodeRenamingForm } from "./components/NodeRenamingForm";
import { RootNode } from "./components/RootNode/RootNode";
import { RootNodeActions } from "./components/RootNode/RootNodeActions";
import { RootNodeChildren } from "./components/RootNode/RootNodeChildren";
import { RootNodeControls } from "./components/RootNode/RootNodeControls";
import { RootNodeHeader } from "./components/RootNode/RootNodeHeader";
import { RootNodeIcon } from "./components/RootNode/RootNodeIcon";
import { RootNodeLabel } from "./components/RootNode/RootNodeLabel";
import { RootNodeOrder } from "./components/RootNode/RootNodeOrder";
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

  Node: typeof Node;
  NodeActions: typeof NodeActions;
  NodeControls: typeof NodeControls;
  NodeChildren: typeof NodeChildren;
  NodeDirCount: typeof NodeDirCount;
  NodeDirToggleIcon: typeof NodeDirToggleIcon;
  NodeLabel: typeof NodeLabel;
  NodeOrder: typeof NodeOrder;
  NodeTriggers: typeof NodeTriggers;

  NodeAddForm: typeof NodeAddForm;
  NodeRenamingForm: typeof NodeRenamingForm;

  ActionsPersistent: typeof ActionsPersistent;
  ActionsHover: typeof ActionsHover;
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

TreeWithSubs.Node = Node;
TreeWithSubs.NodeActions = NodeActions;
TreeWithSubs.NodeControls = NodeControls;
TreeWithSubs.NodeChildren = NodeChildren;
TreeWithSubs.NodeDirToggleIcon = NodeDirToggleIcon;
TreeWithSubs.NodeOrder = NodeOrder;
TreeWithSubs.NodeDirCount = NodeDirCount;
TreeWithSubs.NodeLabel = NodeLabel;
TreeWithSubs.NodeTriggers = NodeTriggers;

TreeWithSubs.NodeAddForm = NodeAddForm;
TreeWithSubs.NodeRenamingForm = NodeRenamingForm;

TreeWithSubs.ActionsPersistent = ActionsPersistent;
TreeWithSubs.ActionsHover = ActionsHover;
TreeWithSubs.ActionLabel = ActionLabel;

export { TreeWithSubs as Tree };
