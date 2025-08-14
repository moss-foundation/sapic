import { StreamCollectionsEvent, StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface GroupedWithEnvironment extends StreamCollectionsEvent {
  environments: StreamEnvironmentsEvent[];
}
