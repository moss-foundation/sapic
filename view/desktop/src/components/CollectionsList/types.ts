import { StreamCollectionsEvent, StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface CollectionWithEnvironment extends StreamCollectionsEvent {
  environments: StreamEnvironmentsEvent[];
}
