import React from "react";

import { useGetViewGroup } from "@/hooks/useGetViewGroup";

import * as components from "./index";

export const ViewContainer = ({ groupId }: { groupId: string }) => {
  const { data: viewGroup } = useGetViewGroup(groupId);

  if (!viewGroup) return <div>Loading...</div>;

  const ComponentToRender = components[viewGroup.component as keyof typeof components] as
    | React.ComponentType<unknown>
    | undefined;

  if (!ComponentToRender) {
    return <div className="flex h-full flex-col">No group view was returned</div>;
  }

  return <ComponentToRender />;
};
