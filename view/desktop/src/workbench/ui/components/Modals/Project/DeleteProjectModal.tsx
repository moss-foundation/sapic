import { useDeleteProject } from "@/adapters/tanstackQuery/project";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ConfirmationModal } from "../ConfirmationModal";
import { ModalWrapperProps } from "../types";

export const DeleteProjectModal = ({ closeModal, showModal, id }: ModalWrapperProps & { id: string }) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { mutateAsync: deleteProject, isPending: isDeleteProjectLoading } = useDeleteProject();
  const localProjectSummaries = useGetAllLocalProjectSummaries();

  const { removePanel } = useTabbedPaneStore();

  const project = localProjectSummaries?.find((p) => p.id === id);

  const handleSubmit = async () => {
    const projectToDelete = localProjectSummaries?.find((p) => p.id === id);

    if (!projectToDelete) return;

    try {
      await deleteProject({ id: projectToDelete.id });

      await treeItemStateService.removeOrder(projectToDelete.id, currentWorkspaceId);

      const projectsAfterDeleted = localProjectSummaries?.filter(
        (p) => p.order && p.order > (projectToDelete.order ?? 0)
      );

      if (projectsAfterDeleted) {
        await treeItemStateService.batchPutOrder(
          Object.fromEntries(projectsAfterDeleted.map((project) => [project.id, project.order! - 1])),
          currentWorkspaceId
        );
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
