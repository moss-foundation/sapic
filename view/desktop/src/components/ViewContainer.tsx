import { useGetViewGroup } from "@/hooks/useGetViewGroup";
import { useWorkspaceStore } from "@/store/workspace";

import CollectionTreeView from "./CollectionTreeView";
import { Icon } from "./index";

export const ViewContainer = ({ groupId }: { groupId: string }) => {
  const { workspace } = useWorkspaceStore((state) => state);
  const { data: viewGroup } = useGetViewGroup(groupId);

  console.log({ viewGroup });
  if (!workspace)
    return (
      <div className="flex h-full flex-col">
        <NoWorkspaceComponent />
      </div>
    );

  if (!viewGroup) {
    return <div>No view group found</div>;
  }

  switch (groupId) {
    case "collections.groupId":
      return <CollectionTreeView />;
    case "environments.groupId":
      return <div>No view group found</div>;
    case "mock.groupId":
      return <div>No view group found</div>;
    default:
      return <div>No view group found</div>;
  }
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
