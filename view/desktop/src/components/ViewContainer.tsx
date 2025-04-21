import React from "react";

import { useGetViewGroup } from "@/hooks/useGetViewGroup";

import * as components from "./index";
import { Icon } from "./index";

export const ViewContainer = ({ groupId }: { groupId: string }) => {
  const { data: viewGroup } = useGetViewGroup(groupId);

  if (!viewGroup) return <div>Empty</div>;

  const ComponentToRender = components[viewGroup.component as keyof typeof components] as
    | React.ComponentType<unknown>
    | undefined;

  if (!ComponentToRender) {
    return (
      <div className="flex h-full flex-col">
        <NoWorkspaceComponent />
      </div>
    );
  }

  return <ComponentToRender />;
};

const NoWorkspaceComponent = () => {
  return (
    <div className="flex flex-col gap-4.25 px-2">
      <div>
        <Icon icon="ErrorNaughtyDog" className="mx-auto size-[200px] w-full" />
        <p className="text-(--moss-secondary-text)">
          You need to open a workspace before accessing collections, environments, or sending requests. Please open or
          create a workspace to proceed.
        </p>
      </div>

      <div className="flex flex-col gap-3.5">
        <button className="background-(--moss-primary) flex cursor-pointer items-center justify-center rounded py-1.5 text-white">
          New workspace
        </button>
        <button className="background-(--moss-primary) flex cursor-pointer items-center justify-center rounded py-1.5 text-white">
          Open workspace
        </button>
      </div>
    </div>
  );
};
