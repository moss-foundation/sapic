import { useBatchUpdateProject, useDeleteProject, useStreamProjects } from "@/adapters/tanstackQuery/project";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchUpdateTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchUpdateTreeItemState";
import { useRemoveTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useRemoveTreeItemState";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ConfirmationModal } from "../ConfirmationModal";
import { ModalWrapperProps } from "../types";

export const DeleteProjectModal = ({ closeModal, showModal, id }: ModalWrapperProps & { id: string }) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: streamedProject } = useStreamProjects();
  const { mutateAsync: deleteProject, isPending: isDeleteProjectLoading } = useDeleteProject();
  const { mutateAsync: batchUpdateProject } = useBatchUpdateProject();

  const { mutateAsync: removeTreeItemState } = useRemoveTreeItemState();
  const { mutateAsync: batchUpdateTreeItemState } = useBatchUpdateTreeItemState();

  const { removePanel } = useTabbedPaneStore();

  const project = streamedProject?.find((p) => p.id === id);

  const handleSubmit = async () => {
    const projectToDelete = streamedProject?.find((p) => p.id === id);

    if (!projectToDelete) return;

    try {
      await deleteProject({ id: projectToDelete.id });

      await removeTreeItemState({
        id: projectToDelete.id,
        workspaceId: currentWorkspaceId,
      });

      const projectsAfterDeleted = streamedProject?.filter((col) => col.order! > projectToDelete.order!);
      if (projectsAfterDeleted) {
        await batchUpdateProject({
          items: projectsAfterDeleted.map((col) => ({
            id: col.id,
            order: col.order! - 1,
          })),
        });

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
