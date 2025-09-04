import { EnvironmentGroup, StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface GroupedWithEnvironment extends EnvironmentGroup {
  environments: StreamEnvironmentsEvent[];
}
