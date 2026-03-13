import { useContext, useState } from "react";

import { useCreateEnvironment } from "@/adapters";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { useCurrentWorkspace } from "@/hooks";
import { environmentItemStateService } from "@/workbench/services/environmentItemStateService";

import { ProjectTreeContext } from "../../../ProjectTree/ProjectTreeContext";

interface UseAddProjectEnvironmentFormProps {
  environmentsList: EnvironmentSummary[];
}

export const useAddProjectEnvironmentForm = ({ environmentsList }: UseAddProjectEnvironmentFormProps) => {
  const { id } = useContext(ProjectTreeContext);
  const { currentWorkspaceId } = useCurrentWorkspace();

  const [isAddingProjectEnvironment, setIsAddingProjectEnvironment] = useState(false);

  const { mutateAsync: createEnvironment } = useCreateEnvironment();

  const handleAddProjectEnvironmentSubmit = async (name: string) => {
    try {
      const newOrder = environmentsList.length + 1;
      const newEnvironment = await createEnvironment({
        projectId: id,
        name: name.trim(),
        color: undefined,
        variables: [],
        order: newOrder,
        expanded: false,
      });

      await environmentItemStateService.putOrder(newEnvironment.id, newOrder, currentWorkspaceId);
      await environmentItemStateService.putExpanded(newEnvironment.id, true, currentWorkspaceId);
    } catch (error) {
      console.error(error);
    } finally {
      setIsAddingProjectEnvironment(false);
    }
  };

  const handleAddProjectEnvironmentFormCancel = () => {
    setIsAddingProjectEnvironment(false);
  };

  return {
    isAddingProjectEnvironment,
    setIsAddingProjectEnvironment,
    handleAddProjectEnvironmentSubmit,
    handleAddProjectEnvironmentFormCancel,
  };
};
