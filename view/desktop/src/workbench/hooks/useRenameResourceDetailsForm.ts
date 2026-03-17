import { useState } from "react";

import { ResourceDetails } from "@/db/resourceDetails/types";
import { resourceService } from "@/domains/resource/resourceService";

import { useTabbedPaneStore } from "../store/tabbedPane";

export const useRenameResourceDetailsForm = (
  resourceDetails: ResourceDetails | undefined,
  projectId: string | undefined
) => {
  const { api } = useTabbedPaneStore();

  const [isRenamingResourceDetails, setIsRenamingResourceDetails] = useState(false);
  const [isUpdatingResourceDetails, setIsUpdatingResourceDetails] = useState(false);

  const handleRenamingResourceDetailsSubmit = async (newName: string) => {
    if (!resourceDetails || !projectId) return;

    const trimmedNewName = newName.trim();
    if (trimmedNewName === resourceDetails.name) return;

    setIsUpdatingResourceDetails(true);

    await resourceService.update(projectId, {
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
    });

    const panel = api?.getPanel(resourceDetails.id);
    panel?.setTitle(trimmedNewName);

    setIsRenamingResourceDetails(false);
    setIsUpdatingResourceDetails(false);
  };
  const handleRenamingResourceDetailsCancel = () => {
    setIsRenamingResourceDetails(false);
  };

  return {
    isRenamingResourceDetails,
    isUpdatingResourceDetails,
    setIsRenamingResourceDetails,
    setIsUpdatingResourceDetails,
    handleRenamingResourceDetailsSubmit,
    handleRenamingResourceDetailsCancel,
  };
};
