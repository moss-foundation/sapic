import { useDeleteProject } from "@/adapters/tanstackQuery/project";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchUpdateTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchUpdateTreeItemState";
import { useRemoveTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useRemoveTreeItemState";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ConfirmationModal } from "../ConfirmationModal";
import { ModalWrapperProps } from "../types";

export const DeleteProjectModal = ({ closeModal, showModal, id }: ModalWrapperProps & { id: string }) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { mutateAsync: deleteProject, isPending: isDeleteProjectLoading } = useDeleteProject();
  const localProjectSummaries = useGetAllLocalProjectSummaries();

  const { mutateAsync: removeTreeItemState } = useRemoveTreeItemState();
  const { mutateAsync: batchUpdateTreeItemState } = useBatchUpdateTreeItemState();

  const { removePanel } = useTabbedPaneStore();

  const project = localProjectSummaries?.find((p) => p.id === id);

  const handleSubmit = async () => {
    const projectToDelete = localProjectSummaries?.find((p) => p.id === id);

    if (!projectToDelete) return;

    try {
      await deleteProject({ id: projectToDelete.id });

      await removeTreeItemState({
        id: projectToDelete.id,
        workspaceId: currentWorkspaceId,
      });

      const projectsAfterDeleted = localProjectSummaries?.filter((p) => p.order > projectToDelete.order);

      if (projectsAfterDeleted) {
        await batchUpdateTreeItemState({
          treeItemStates: projectsAfterDeleted.map((project) => ({
            id: project.id,
            order: project.order! - 1,
            expanded: project.expanded,
          })),
          workspaceId: currentWorkspaceId,
        });
      }

      closeModal();
      removePanel(id);
    } catch (error) {
      console.error(error);
    }
  };

  const handleCancel = () => {
    closeModal();
  };

  return (
    <ConfirmationModal
      showModal={showModal}
      closeModal={closeModal}
      title="Delete Project"
      message={`Are you sure you want to delete ${project?.name} project?`}
      onConfirm={handleSubmit}
      onCancel={handleCancel}
      loading={isDeleteProjectLoading}
    />
  );
};
