import { useState } from "react";

import { useUpdateProjectResource } from "@/adapters";
import { ResourceDetails } from "@/db/resourceDetails/types";

import { useTabbedPaneStore } from "../store/tabbedPane";

export const useRenameResourceDetailsForm = (
  resourceDetails: ResourceDetails | undefined,
  projectId: string | undefined
) => {
  const { api } = useTabbedPaneStore();
  const { mutateAsync: updateProjectResource, isPending: isUpdatingResourceDetails } = useUpdateProjectResource();

  const [isRenamingResourceDetails, setIsRenamingResourceDetails] = useState(false);

  const handleRenamingResourceDetailsSubmit = async (newName: string) => {
    if (!resourceDetails || !projectId) return;

    const trimmedNewName = newName.trim();

    if (trimmedNewName === resourceDetails.name) return;

    await updateProjectResource({
      projectId,
      updateResourceInput: {
        ITEM: {
          id: resourceDetails.id,
          name: trimmedNewName,
          headersToAdd: [],
          headersToUpdate: [],
          headersToRemove: [],
          pathParamsToAdd: [],
          pathParamsToUpdate: [],
          pathParamsToRemove: [],
          queryParamsToAdd: [],
          queryParamsToUpdate: [],
          queryParamsToRemove: [],
        },
      },
    });

    const panel = api?.getPanel(resourceDetails.id);
    panel?.setTitle(trimmedNewName);

    setIsRenamingResourceDetails(false);
  };
  const handleRenamingResourceDetailsCancel = () => {
    setIsRenamingResourceDetails(false);
  };

  return {
    isRenamingResourceDetails,
    setIsRenamingResourceDetails,
    isUpdatingResourceDetails,
    handleRenamingResourceDetailsSubmit,
    handleRenamingResourceDetailsCancel,
  };
};
