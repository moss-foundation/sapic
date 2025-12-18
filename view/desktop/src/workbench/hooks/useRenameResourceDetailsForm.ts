import { useState } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetailsCollection";
import { ResourceDetails } from "@/db/types";

import { useTabbedPaneStore } from "../store/tabbedPane";

export const useRenameResourceDetailsForm = (details?: ResourceDetails) => {
  const { api } = useTabbedPaneStore();

  const [isRenamingResourceDetails, setIsRenamingResourceDetails] = useState(false);

  const handleRenamingResourceDetailsSubmit = async (newName: string) => {
    if (!details) return;

    try {
      const trimmedNewName = newName.trim();

      if (trimmedNewName === details.name) {
        return;
      }

      resourceDetailsCollection.update(details.id, (draft) => {
        draft.name = trimmedNewName;
      });

      const panel = api?.getPanel(details.id);
      panel?.setTitle(trimmedNewName);
    } catch (error) {
      console.error("Error renaming resource details", error);
    } finally {
      setIsRenamingResourceDetails(false);
    }
  };

  const handleRenamingResourceDetailsCancel = () => {
    setIsRenamingResourceDetails(false);
  };

  return {
    isRenamingResourceDetails,
    setIsRenamingResourceDetails,
    handleRenamingResourceDetailsSubmit,
    handleRenamingResourceDetailsCancel,
  };
};
