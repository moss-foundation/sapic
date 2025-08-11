import { useState } from "react";

import { useUpdateEnvironment } from "@/hooks/environment";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface UseWorkspaceListItemRenamingFormProps {
  environment: StreamEnvironmentsEvent;
}

export const useWorkspaceListItemRenamingForm = ({ environment }: UseWorkspaceListItemRenamingFormProps) => {
  const { mutateAsync: updateEnvironment } = useUpdateEnvironment();
  const [isEditing, setIsEditing] = useState(false);

  const handleRename = async (name: string) => {
    await updateEnvironment({
      id: environment.id,
      name,
      //FIXME: add varsToAdd, varsToUpdate, varsToDelete are required
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
