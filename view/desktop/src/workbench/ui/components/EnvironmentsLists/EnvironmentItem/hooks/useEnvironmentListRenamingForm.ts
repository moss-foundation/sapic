import { useState } from "react";

import { useUpdateEnvironment } from "@/adapters/tanstackQuery/environment";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface UseGlobalEnvironmentsListRenamingFormProps {
  environment: StreamEnvironmentsEvent;
}

export const useGlobalEnvironmentsListRenamingForm = ({ environment }: UseGlobalEnvironmentsListRenamingFormProps) => {
  const { mutateAsync: updateEnvironment } = useUpdateEnvironment();
  const [isEditing, setIsEditing] = useState(false);

  const handleRename = async (name: string) => {
    await updateEnvironment({
      id: environment.id,
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
