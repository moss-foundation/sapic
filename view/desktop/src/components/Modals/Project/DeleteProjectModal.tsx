import { useBatchUpdateProject, useDeleteProject, useStreamProjects } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ConfirmationModal } from "../ConfirmationModal";
import { ModalWrapperProps } from "../types";

export const DeleteProjectModal = ({ closeModal, showModal, id }: ModalWrapperProps & { id: string }) => {
  const { data: streamedProject } = useStreamProjects();
  const { mutateAsync: deleteProject, isPending: isDeleteProjectLoading } = useDeleteProject();
  const { mutateAsync: batchUpdateProject } = useBatchUpdateProject();

  const { removePanel } = useTabbedPaneStore();

  const project = streamedProject?.find((p) => p.id === id);

  const handleSubmit = async () => {
    const projectToDelete = streamedProject?.find((p) => p.id === id);

    if (!projectToDelete) {
      return;
    }

    try {
      await deleteProject({ id: projectToDelete.id });

      const projectsAfterDeleted = streamedProject?.filter((col) => col.order! > projectToDelete.order!);
      if (projectsAfterDeleted) {
        await batchUpdateProject({
          items: projectsAfterDeleted.map((col) => ({
            id: col.id,
            order: col.order! - 1,
          })),
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
