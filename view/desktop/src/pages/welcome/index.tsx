import { useDeleteWorkspace, useListWorkspaces } from "@/adapters/tanstackQuery/workspace";
import { useModal } from "@/hooks/useModal";
import { Button, Icon } from "@/lib/ui";
import { useWelcomeOpenWorkspace } from "@/welcome/adapters/tanstackQuery/workspace";
import { ConfirmationModal } from "@/workbench/ui/components";
import { NewWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/NewWorkspaceModal";
import { WorkspaceInfo } from "@repo/ipc";

export const WelcomePage = () => {
  const { data: workspaces } = useListWorkspaces();

  const {
    showModal: showNewWorkspaceModal,
    closeModal: closeNewWorkspaceModal,
    openModal: openNewWorkspaceModal,
  } = useModal();

  return (
    <>
      <div className="grid h-screen w-full place-items-center" data-tauri-drag-region>
        <div className="flex flex-col gap-6">
          <h1 className="text-3xl font-bold">Your workspaces</h1>

          <div className="flex flex-col gap-2">
            {workspaces && workspaces.length === 0 ? (
              <span className="text-(--moss-secondary-foreground)">No workspaces</span>
            ) : (
              <div className="grid grid-cols-[1fr_max-content_max-content] items-start gap-1.5">
                {workspaces?.map((workspace) => <WorkspaceLine key={workspace.id} workspace={workspace} />)}
              </div>
            )}
          </div>

          <div className="flex justify-end">
            <Button className="flex cursor-pointer gap-1.5" onClick={openNewWorkspaceModal}>
              New workspace
            </Button>
          </div>
        </div>
      </div>

      {showNewWorkspaceModal && (
        <NewWorkspaceModal
          closeModal={closeNewWorkspaceModal}
          showModal={showNewWorkspaceModal}
          forceNewWindow
          window="welcome"
        />
      )}
    </>
  );
};

const WorkspaceLine = ({ workspace }: { workspace: WorkspaceInfo }) => {
  const { mutate: openWelcomeWorkspace } = useWelcomeOpenWorkspace();
  const { mutateAsync: deleteWorkspace } = useDeleteWorkspace();
  const {
    showModal: showDeleteWorkspaceModal,
    closeModal: closeDeleteWorkspaceModal,
    openModal: openDeleteWorkspaceModal,
  } = useModal();

  return (
    <div className="col-span-full grid grid-cols-subgrid">
      <button
        onClick={() => openWelcomeWorkspace({ id: workspace.id })}
        className="cursor-pointer text-left text-lg text-blue-500 hover:text-blue-600 hover:underline"
      >
        {workspace.name}
      </button>

      <button
        onClick={openDeleteWorkspaceModal}
        className="flex cursor-pointer items-center gap-1.5 text-lg text-red-500 hover:text-red-600 hover:underline"
      >
        <span className="">Delete</span>
        <Icon icon="Trash" className="size-4 bg-red-500 text-white" />
      </button>

      <ConfirmationModal
        title="Delete Workspace"
        message={`Are you sure you want to delete the workspace "${workspace.name}"?`}
        closeModal={closeDeleteWorkspaceModal}
        showModal={showDeleteWorkspaceModal}
        onConfirm={async () => await deleteWorkspace({ id: workspace.id })}
        onCancel={closeDeleteWorkspaceModal}
      />
    </div>
  );
};
