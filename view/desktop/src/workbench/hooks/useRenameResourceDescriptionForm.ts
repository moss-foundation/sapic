import { useState } from "react";

import { ResourceDescription, resourcesDescriptionsCollection } from "@/app/resourcesDescriptionsCollection";

import { useTabbedPaneStore } from "../store/tabbedPane";

export const useRenameResourceDescriptionForm = (description?: ResourceDescription) => {
  const { api } = useTabbedPaneStore();

  const [isRenamingResourceDescription, setIsRenamingResourceDescription] = useState(false);

  const handleRenamingResourceDescriptionSubmit = async (newName: string) => {
    if (!description) return;

    try {
      const trimmedNewName = newName.trim();

      if (trimmedNewName === description.name) {
        return;
      }

      resourcesDescriptionsCollection.update(description.id, (draft) => {
        draft.name = trimmedNewName;
      });

      const panel = api?.getPanel(description.id);
      if (panel) {
        panel.setTitle(trimmedNewName);
      }
    } catch (error) {
      console.error("Error renaming resource description", error);
    } finally {
      setIsRenamingResourceDescription(false);
    }
  };

  const handleRenamingResourceDescriptionCancel = () => {
    setIsRenamingResourceDescription(false);
  };

  return {
    isRenamingResourceDescription,
    setIsRenamingResourceDescription,
    handleRenamingResourceDescriptionSubmit,
    handleRenamingResourceDescriptionCancel,
  };
};
