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
import { Root } from "./components/Root/Root";
import { RootActions } from "./components/Root/RootActions";
import { RootChildren } from "./components/Root/RootChildren";
import { RootDetails } from "./components/Root/RootDetails";
import { RootHeader } from "./components/Root/RootHeader";
import { RootIcon } from "./components/Root/RootIcon";
import { RootLabel } from "./components/Root/RootLabel";
import { RootOrder } from "./components/Root/RootOrder";
import { RootTriggers } from "./components/Root/RootTriggers";
import { Tree } from "./Tree";

interface TreeWithSubComponents {
  (props: React.ComponentProps<typeof Tree>): React.ReactElement;
  Root: typeof Root;
  RootActions: typeof RootActions;
  RootChildren: typeof RootChildren;
  RootDetails: typeof RootDetails;
  RootHeader: typeof RootHeader;
  RootIcon: typeof RootIcon;
  RootLabel: typeof RootLabel;
  RootOrder: typeof RootOrder;
  RootTriggers: typeof RootTriggers;

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

TreeWithSubs.Root = Root;
TreeWithSubs.RootActions = RootActions;
TreeWithSubs.RootChildren = RootChildren;
TreeWithSubs.RootDetails = RootDetails;
TreeWithSubs.RootHeader = RootHeader;
TreeWithSubs.RootIcon = RootIcon;
TreeWithSubs.RootLabel = RootLabel;
TreeWithSubs.RootOrder = RootOrder;
TreeWithSubs.RootTriggers = RootTriggers;

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
