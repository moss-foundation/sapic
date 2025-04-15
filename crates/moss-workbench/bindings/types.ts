// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type ActivityEvent =
  | { "oneshot": { id: number; activityId: string; title: string; detail?: string } }
  | { "start": { id: number; activityId: string; title: string; detail?: string } }
  | { "progress": { id: number; activityId: string; detail?: string } }
  | { "finish": { id: number; activityId: string } };
