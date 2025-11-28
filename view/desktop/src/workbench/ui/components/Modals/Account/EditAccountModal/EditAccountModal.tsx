import { FormEvent, useState } from "react";

import { useUpdateProfile } from "@/adapters";
import { Button, Link, Modal } from "@/lib/ui";
import { AccountInfo } from "@repo/base";
import { UpdateAccountParams, UpdateProfileInput } from "@repo/window";

import { ModalWrapperProps } from "../../types";
import { getPatPlaceholder, getProviderName, getProviderSettingsUrl } from "../accountUtils";

interface EditAccountModalProps extends ModalWrapperProps {
  account: AccountInfo | null;
  onAccountUpdated?: () => void;
}

export const EditAccountModal = ({ showModal, closeModal, account, onAccountUpdated }: EditAccountModalProps) => {
  const [token, setToken] = useState("");
  const { mutateAsync: updateProfile, isPending: isUpdatingProfile } = useUpdateProfile();

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!account || !token) return;

    try {
      const accountParams: UpdateAccountParams = {
        id: account.id,
        pat: token,
      };

      const input: UpdateProfileInput = {
        accountsToAdd: [],
        accountsToRemove: [],
        accountsToUpdate: [accountParams],
      };

      await updateProfile(input);

      handleClose();
      onAccountUpdated?.();
    } catch (error) {
      console.error("Error updating account:", error);
      alert(`Failed to update account: ${error}`);
    }
  };

  const handleClose = () => {
    closeModal();
    reset();
  };

  const reset = () => {
    setTimeout(() => {
      setToken("");
    }, 200);
  };

  const isSubmitDisabled = isUpdatingProfile || !token;

  if (!account) return null;

  const providerName = getProviderName(account.kind);
  const settingsUrl = getProviderSettingsUrl(account.kind);

  return (
    <Modal onBackdropClick={handleClose} showModal={showModal} className="max-w-136 w-full">
      <form onSubmit={handleSubmit} className="flex flex-col overflow-hidden">
        <h2 className="border-(--moss-border) flex items-center justify-center border-b py-2 font-medium leading-4">
          Edit details
        </h2>

        <div className="px-6 pb-3.5 pt-6">
          <div className="grid grid-cols-[min-content_1fr] items-start gap-x-3 gap-y-1.5">
            <label className="pt-1.5 text-base">Token:</label>
            <textarea
              value={token}
              onChange={(e) => setToken(e.target.value)}
              placeholder={getPatPlaceholder(account.kind)}
              className="h-24.5 border-(--moss-border) placeholder-(--moss-secondary-foreground) focus:outline-(--moss-primary) w-full resize-none rounded-sm border px-2 py-1.5 text-sm focus:outline-2"
              autoFocus
            />
            <div></div>
            <p className="text-(--moss-secondary-foreground) text-sm leading-4">
              Enter your personal access token (PAT). You can get it in your{" "}
              <Link href={settingsUrl} target="_blank" rel="noopener noreferrer">
                {providerName}
              </Link>{" "}
              settings. The token is stored locally and used only for login.
            </p>
          </div>
        </div>

        <div className="border-(--moss-border) flex items-center justify-end gap-3 border-t px-6 py-4">
          <Button intent="outlined" type="button" onClick={handleClose} disabled={isUpdatingProfile}>
            Close
          </Button>
          <Button intent="primary" disabled={isSubmitDisabled} type="submit">
            {isUpdatingProfile ? "Saving..." : "Save"}
          </Button>
        </div>
      </form>
    </Modal>
  );
};
