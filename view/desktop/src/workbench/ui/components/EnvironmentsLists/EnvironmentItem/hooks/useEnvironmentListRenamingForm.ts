import { useState } from "react";

import { useUpdateEnvironment } from "@/adapters/tanstackQuery/environment";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";

interface UseGlobalEnvironmentsListRenamingFormProps {
  environment: EnvironmentSummary;
}

export const useGlobalEnvironmentsListRenamingForm = ({ environment }: UseGlobalEnvironmentsListRenamingFormProps) => {
  const { mutateAsync: updateEnvironment } = useUpdateEnvironment();
  const [isEditing, setIsEditing] = useState(false);

  const handleRename = async (name: string) => {
    await updateEnvironment({
      id: environment.id,
      projectId: environment.projectId ?? undefined,
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
