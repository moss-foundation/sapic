import { EnvironmentGroup, StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface GroupedWithEnvironment extends EnvironmentGroup {
  environments: StreamEnvironmentsEvent[];
}

export interface DragGroupedEnvironmentsListItem {
  type: "GroupedEnvironmentsListItem";
  data: {
    groupWithEnvironments: GroupedWithEnvironment;
  };
  [key: string | symbol]: unknown;
}

export interface DropGroupedEnvironmentsListItem {
  type: "GroupedEnvironmentsListItem";
  data: {
    groupWithEnvironments: GroupedWithEnvironment;
  };
  [key: string | symbol]: unknown;
}
