import { useState } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useGetLocalProjectSummaryById } from "@/db/projectSummaries/hooks/useGetLocalProjectSummaryById";
import { projectService } from "@/domains/project/projectService";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ConfirmationModal } from "../ConfirmationModal";
import { ModalWrapperProps } from "../types";

export const DeleteProjectModal = ({ closeModal, showModal, id }: ModalWrapperProps & { id: string }) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const [isDeleteProjectLoading, setIsDeleteProjectLoading] = useState(false);

  const { data: localProjectSummaries } = useGetAllLocalProjectSummaries();
  const projectSummaryToDelete = useGetLocalProjectSummaryById(id);

  const { removePanel } = useTabbedPaneStore();

  const handleSubmit = async () => {
    try {
      setIsDeleteProjectLoading(true);
      await projectService.deleteProject({ id });

      await treeItemStateService.removeOrder(id, currentWorkspaceId);

      const projectsAfterDeleted = localProjectSummaries?.filter(
        (p) => p.order && p.order > (projectSummaryToDelete?.order ?? 0)
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
    } finally {
      setIsDeleteProjectLoading(false);
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
      message={`Are you sure you want to delete ${projectSummaryToDelete?.name} project?`}
      onConfirm={handleSubmit}
      onCancel={handleCancel}
      loading={isDeleteProjectLoading}
    />
  );
};
