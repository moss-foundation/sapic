import { EnvironmentGroup, StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface GroupedEnvironments extends EnvironmentGroup {
  environments: StreamEnvironmentsEvent[];
}
