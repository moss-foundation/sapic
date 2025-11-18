import { StreamEnvironmentsEvent, StreamEnvironmentsOutput } from "@repo/moss-workspace";

export interface StreamEnvironmentsResult {
  environments: StreamEnvironmentsEvent[];
  groups: StreamEnvironmentsOutput["groups"];
}
