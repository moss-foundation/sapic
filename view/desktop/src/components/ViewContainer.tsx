import { useDescribeWorkspaceState } from "@/hooks";
import { useModal } from "@/hooks/useModal";
import { useGetViewGroup } from "@/hooks/viewGroups/useGetViewGroup";

import CollectionTreeView from "./CollectionTreeView";
import { Icon } from "./index";
import { NewWorkspaceModal } from "./Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "./Modals/Workspace/OpenWorkspaceModal";

export const ViewContainer = ({ groupId }: { groupId: string }) => {
  const { data: viewGroup } = useGetViewGroup(groupId);

  const { isFetched: isWorkspaceStateFetched } = useDescribeWorkspaceState();

  if (!isWorkspaceStateFetched) {
    return (
      <div className="flex h-full flex-col">
        <NoWorkspaceComponent />
      </div>
    );
  }

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
  const {
    showModal: showNewWorkspaceModal,
    closeModal: closeNewWorkspaceModal,
    openModal: openNewWorkspaceModal,
  } = useModal();
  const {
    showModal: showOpenWorkspaceModal,
    closeModal: closeOpenWorkspaceModal,
    openModal: openOpenWorkspaceModal,
  } = useModal();

  return (
    <div className="flex flex-col gap-4.25 px-2">
      <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />

      <div>
        <Icon icon="ErrorNaughtyDog" className="mx-auto h-auto w-full max-w-[200px]" />
        <p className="text-(--moss-secondary-text)">
          You need to open a workspace before accessing collections, environments, or sending requests. Please open or
          create a workspace to proceed.
        </p>
      </div>

      <div className="flex flex-col gap-3.5">
        {/* //TODO This should be a button component */}
        <button
          onClick={openNewWorkspaceModal}
          className="background-(--moss-primary) hover:background-(--moss-blue-3) flex cursor-pointer items-center justify-center rounded py-1.5 text-white"
        >
          New workspace
        </button>
        {/* //TODO This should be a button component */}
        <button
          onClick={openOpenWorkspaceModal}
          className="background-(--moss-primary) hover:background-(--moss-blue-3) flex cursor-pointer items-center justify-center rounded py-1.5 text-white"
        >
          Open workspace
        </button>
      </div>
    </div>
  );
};
