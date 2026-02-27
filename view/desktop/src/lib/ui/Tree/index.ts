import { ActionLabel } from "./components/ActionLabel";
import { ActionsHover } from "./components/ActionsHover";
import { ActionsPersistent } from "./components/ActionsPersistent";
import { Decorator } from "./components/Decorator";
import { List } from "./components/List/List";
import { ListActions } from "./components/List/ListActions";
import { ListChildren } from "./components/List/ListChildren";
import { ListDirCount } from "./components/List/ListDirCount";
import { ListHeader } from "./components/List/ListHeader";
import { ListHeaderDetails } from "./components/List/ListHeaderDetails";
import { ListLabel } from "./components/List/ListLabel";
import { Node } from "./components/Node/Node";
import { NodeActions } from "./components/Node/NodeActions";
import { NodeChildren } from "./components/Node/NodeChildren";
import { NodeDetails } from "./components/Node/NodeDetails";
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
import { RootNodeDetails } from "./components/RootNode/RootNodeDetails";
import { RootNodeHeader } from "./components/RootNode/RootNodeHeader";
import { RootNodeIcon } from "./components/RootNode/RootNodeIcon";
import { RootNodeLabel } from "./components/RootNode/RootNodeLabel";
import { RootNodeOrder } from "./components/RootNode/RootNodeOrder";
import { RootNodeTriggers } from "./components/RootNode/RootNodeTriggers";
import { Tree } from "./Tree";

interface TreeWithSubComponents {
  (props: React.ComponentProps<typeof Tree>): React.ReactElement;
  RootNode: typeof RootNode;
  RootNodeActions: typeof RootNodeActions;
  RootNodeChildren: typeof RootNodeChildren;
  RootNodeDetails: typeof RootNodeDetails;
  RootNodeHeader: typeof RootNodeHeader;
  RootNodeIcon: typeof RootNodeIcon;
  RootNodeLabel: typeof RootNodeLabel;
  RootNodeOrder: typeof RootNodeOrder;
  RootNodeTriggers: typeof RootNodeTriggers;

  List: typeof List;
  ListHeader: typeof ListHeader;
  ListLabel: typeof ListLabel;
  ListChildren: typeof ListChildren;
  ListHeaderDetails: typeof ListHeaderDetails;
  ListDirCount: typeof ListDirCount;
  ListActions: typeof ListActions;

  Node: typeof Node;
  NodeActions: typeof NodeActions;
  NodeDetails: typeof NodeDetails;
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

  Decorator: typeof Decorator;
}

const TreeWithSubs = Tree as TreeWithSubComponents;

TreeWithSubs.RootNode = RootNode;
TreeWithSubs.RootNodeActions = RootNodeActions;
TreeWithSubs.RootNodeChildren = RootNodeChildren;
TreeWithSubs.RootNodeDetails = RootNodeDetails;
TreeWithSubs.RootNodeHeader = RootNodeHeader;
TreeWithSubs.RootNodeIcon = RootNodeIcon;
TreeWithSubs.RootNodeLabel = RootNodeLabel;
TreeWithSubs.RootNodeOrder = RootNodeOrder;
TreeWithSubs.RootNodeTriggers = RootNodeTriggers;

TreeWithSubs.Node = Node;
TreeWithSubs.NodeActions = NodeActions;
TreeWithSubs.NodeDetails = NodeDetails;
TreeWithSubs.NodeChildren = NodeChildren;
TreeWithSubs.NodeDirToggleIcon = NodeDirToggleIcon;
TreeWithSubs.NodeOrder = NodeOrder;
TreeWithSubs.NodeDirCount = NodeDirCount;
TreeWithSubs.NodeLabel = NodeLabel;
TreeWithSubs.NodeTriggers = NodeTriggers;

TreeWithSubs.List = List;
TreeWithSubs.ListHeader = ListHeader;
TreeWithSubs.ListLabel = ListLabel;
TreeWithSubs.ListChildren = ListChildren;
TreeWithSubs.ListHeaderDetails = ListHeaderDetails;
TreeWithSubs.ListDirCount = ListDirCount;
TreeWithSubs.ListActions = ListActions;

TreeWithSubs.NodeAddForm = NodeAddForm;
TreeWithSubs.NodeRenamingForm = NodeRenamingForm;

TreeWithSubs.ActionsPersistent = ActionsPersistent;
TreeWithSubs.ActionsHover = ActionsHover;
TreeWithSubs.ActionLabel = ActionLabel;

TreeWithSubs.Decorator = Decorator;

export { TreeWithSubs as Tree };
