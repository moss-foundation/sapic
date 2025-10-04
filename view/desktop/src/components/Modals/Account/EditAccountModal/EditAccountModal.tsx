import { FormEvent, useState } from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { Modal } from "@/lib/ui";
import { Link } from "@/lib/ui";
import { invoke } from "@tauri-apps/api/core";
import { AddAccountParams, UpdateProfileInput } from "@repo/moss-app";
import { AccountInfo } from "@repo/moss-user";

import { ModalWrapperProps } from "../../types";
import { getPatPlaceholder, getProviderName, getProviderSettingsUrl } from "../accountUtils";

interface EditAccountModalProps extends ModalWrapperProps {
  account: AccountInfo | null;
  onAccountUpdated?: () => void;
}

export const EditAccountModal = ({ showModal, closeModal, account, onAccountUpdated }: EditAccountModalProps) => {
  const [token, setToken] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!account || !token) return;

    try {
      setIsSubmitting(true);

      // TODO: Replace with dedicated update account endpoint when available
      // Strategy: Remove old account and add new account with updated PAT
      // This is necessary because there's no dedicated "update account" endpoint
      const accountParams: AddAccountParams = {
        host: account.host,
        label: "",
        kind: account.kind,
        pat: token,
      };

      const input: UpdateProfileInput = {
        accountsToAdd: [accountParams],
        accountsToRemove: [account.id],
      };

      await invoke("update_profile", { input });

      handleClose();
      onAccountUpdated?.();
    } catch (error) {
      console.error("Error updating account:", error);
      alert(`Failed to update account: ${error}`);
    } finally {
      setIsSubmitting(false);
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

  const isSubmitDisabled = isSubmitting || !token;

  if (!account) return null;

  const providerName = getProviderName(account.kind);
  const settingsUrl = getProviderSettingsUrl(account.kind);

  return (
    <Modal onBackdropClick={handleClose} showModal={showModal} className="w-full max-w-136">
      <form onSubmit={handleSubmit} className="flex flex-col overflow-hidden">
        <h2 className="flex items-center justify-center border-b border-(--moss-border-color) py-2 leading-4 font-medium">
          Edit details
        </h2>

        <div className="px-6 pt-6 pb-3.5">
          <div className="grid grid-cols-[min-content_1fr] items-start gap-x-3 gap-y-1.5">
            <label className="pt-1.5 text-base">Token:</label>
            <textarea
              value={token}
              onChange={(e) => setToken(e.target.value)}
              placeholder={getPatPlaceholder(account.kind)}
              className="h-24.5 w-full resize-none rounded-sm border border-(--moss-border-color) px-2 py-1.5 text-sm placeholder-(--moss-secondary-text) focus:outline-2 focus:outline-(--moss-primary)"
              autoFocus
            />
            <div></div>
            <p className="text-sm leading-4 text-(--moss-secondary-text)">
              Enter your personal access token (PAT). You can get it in your{" "}
              <Link href={settingsUrl} target="_blank" rel="noopener noreferrer">
                {providerName}
              </Link>{" "}
              settings. The token is stored locally and used only for login.
            </p>
          </div>
        </div>

        <div className="flex items-center justify-end gap-3 border-t border-(--moss-border-color) px-6 py-4">
          <ButtonNeutralOutlined type="button" onClick={handleClose} disabled={isSubmitting}>
            Close
          </ButtonNeutralOutlined>
          <ButtonPrimary disabled={isSubmitDisabled} type="submit">
            {isSubmitting ? "Saving..." : "Save"}
          </ButtonPrimary>
        </div>
      </form>
    </Modal>
  );
};
