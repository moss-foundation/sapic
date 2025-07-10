import { useState, useRef } from "react";

import { InputOutlined } from "@/components";
import { VALID_NAME_PATTERN } from "@/constants/validation";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { ModalForm } from "@/components/ModalForm";
import { useUpdateWorkspace } from "@/hooks/workbench/useUpdateWorkspace";

interface RenameWorkspaceModalProps {
  showModal: boolean;
  closeModal: () => void;
  workspaceId: string;
  currentName: string;
}

export const RenameWorkspaceModal = ({ closeModal, showModal, currentName }: RenameWorkspaceModalProps) => {
  const { mutate: updateWorkspace, isPending } = useUpdateWorkspace();
  const [name, setName] = useState(currentName);
  const isSelectingText = useRef(false);

  const handleSubmit = () => {
    if (name.trim() && name.trim() !== currentName) {
      updateWorkspace(
        { name: name.trim() },
        {
          onSuccess: () => {
            closeModal();
            resetForm();
          },
          onError: (error) => {
            console.error("Failed to rename workspace:", error.message);
          },
        }
      );
    } else {
      closeModal();
      resetForm();
    }
  };

  const handleCancel = () => {
    closeModal();
    resetForm();
  };

  const resetForm = () => {
    setTimeout(() => {
      setName(currentName);
    }, 200);
  };

  const handleBackdropClick = () => {
    // Only close if we're not in the middle of a text selection
    if (!isSelectingText.current) {
      handleCancel();
    }
  };

  const handleInputMouseDown = () => {
    // Mark that we're starting a text selection
    isSelectingText.current = true;
  };

  const handleInputMouseUp = () => {
    // End text selection after a short delay to ensure backdrop click is processed
    setTimeout(() => {
      isSelectingText.current = false;
    }, 50);
  };

  const isSubmitDisabled = !name.trim() || name.trim() === currentName || isPending;

  return (
    <ModalForm
      showModal={showModal}
      onBackdropClick={handleBackdropClick}
      title="Rename Workspace"
      content={
        <div className="space-y-4">
          <div>
            <label className="mb-2 block text-sm font-medium text-(--moss-primary-text)">Workspace Name</label>
            <InputOutlined
              size="md"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter workspace name..."
              pattern={VALID_NAME_PATTERN}
              autoFocus
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  e.preventDefault();
                  handleSubmit();
                } else if (e.key === "Escape") {
                  e.preventDefault();
                  handleCancel();
                }
              }}
              onMouseDown={handleInputMouseDown}
              onMouseUp={handleInputMouseUp}
              onFocus={(e) => {
                // Select all text when the input is focused for better UX
                e.target.select();
              }}
            />
            <p className="mt-1 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \\ : * ? " < > |) will be escaped`}</p>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-end gap-3 py-0.75">
          <ButtonNeutralOutlined onClick={handleCancel}>Cancel</ButtonNeutralOutlined>
          <ButtonPrimary disabled={isSubmitDisabled} onClick={handleSubmit}>
            {isPending ? "Renaming..." : "Rename"}
          </ButtonPrimary>
        </div>
      }
    />
  );
};
