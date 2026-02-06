import { useState } from "react";

import { useUpdateEnvironment } from "@/adapters";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";

interface UseGlobalEnvironmentsListRenamingFormProps {
  environment: EnvironmentSummary;
}

export const useEnvironmentItemRenamingForm = ({ environment }: UseGlobalEnvironmentsListRenamingFormProps) => {
  const { mutateAsync: updateEnvironment } = useUpdateEnvironment();
  const [isEditing, setIsEditing] = useState(false);

  const handleRename = async (name: string) => {
    await updateEnvironment({
      id: environment.id,
      ...(environment.projectId && { projectId: environment.projectId }),
      name,
      varsToAdd: [],
      varsToUpdate: [],
      varsToDelete: [],
    });

    setIsEditing(false);
  };

  const handleCancel = () => {
    setIsEditing(false);
  };

  return { isEditing, setIsEditing, handleRename, handleCancel };
};
