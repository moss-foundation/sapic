import { useEffect, useMemo, useRef, useState } from "react";

import { useListUserAccounts } from "@/adapters";
import { useFocusInputOnMount, useModal } from "@/hooks";
import { Button } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import { PillTabs } from "@/lib/ui/Tabs/index";
import { cn } from "@/utils";
import { VcsProviderSwitcher } from "@/workbench/ui/components/VcsProviderSwitcher";
import { CheckedState } from "@radix-ui/react-checkbox";
import { CreateProjectGitParams } from "@repo/ipc";

import { NewAccountModal } from "../../../Account/NewAccountModal";
import { AccountSelect } from "../components/AccountSelect";
import { BranchInput } from "../components/BranchInput";
import { NameInput } from "../components/NameInput";
import { RepositoryInput } from "../components/RepositoryInput";
import { DEFAULT_BRANCH, DEFAULT_NAME, DEFAULT_PROVIDER, DEFAULT_REPOSITORY, DEFAULT_VCS } from "../defaults";

interface CreateSectionProps {
  onValuesUpdate: (values: { name: string; gitParams: CreateProjectGitParams | undefined }) => void;
}

export const CreateSection = ({ onValuesUpdate }: CreateSectionProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { data: userAccounts } = useListUserAccounts();

  const githubAccounts = userAccounts?.accounts.filter((account) => account.kind === "GITHUB") ?? [];
  const gitlabAccounts = userAccounts?.accounts.filter((account) => account.kind === "GITLAB") ?? [];
  const hasNoAccounts = userAccounts?.accounts.length === 0;

  const [name, setName] = useState(DEFAULT_NAME);
  const [provider, setProvider] = useState<"github" | "gitlab">(DEFAULT_PROVIDER);
  const [repository, setRepository] = useState(DEFAULT_REPOSITORY);
  const [branch, setBranch] = useState(DEFAULT_BRANCH);
  const [vcs, setVCS] = useState(DEFAULT_VCS);
  const [accountId, setAccountId] = useState("");

  useFocusInputOnMount({ inputRef });

  const {
    openModal: openNewAccountModal,
    closeModal: closeNewAccountModal,
    showModal: isNewAccountModalOpen,
  } = useModal();

  const gitParams = useMemo(() => {
    if (!vcs) return undefined;

    const params = { repository, branch, accountId };
    const providerMap = {
      github: { gitHub: params },
      gitlab: { gitLab: params },
    } as const;

    return providerMap[provider] ?? undefined;
  }, [vcs, provider, repository, branch, accountId]);

  useEffect(() => {
    onValuesUpdate({ name, gitParams });
  }, [name, gitParams, onValuesUpdate]);

  const handleSetVCS = (checked: CheckedState) => {
    if (checked === "indeterminate") return;
    setVCS(checked);
  };

  const handleSetAccount = (accountId: string) => {
    setAccountId(accountId);
  };

  return (
    <>
      <div className="flex flex-col gap-3">
        <div className="grid grid-cols-[min-content_1fr] items-center gap-x-2 py-3">
          <NameInput name={name} setName={setName} ref={inputRef} />
        </div>

        <div>
          <div className="flex flex-col gap-2">
            <CheckboxWithLabel checked={vcs} onCheckedChange={handleSetVCS} label="VCS" />
            <span className="text-(--moss-secondary-foreground) text-sm">
              You can switch modes in the workspace at any time and as often as needed.
            </span>
          </div>

          <div
            className={cn("grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pb-2 pt-3", {
              "": hasNoAccounts,
              "pl-5": !hasNoAccounts,
            })}
          >
            {hasNoAccounts ? (
              <Button intent="primary" onClick={openNewAccountModal} disabled={!vcs}>
                Connect new account
              </Button>
            ) : (
              <VcsProviderSwitcher
                value={provider}
                onValueChange={(value) => setProvider(value as "github" | "gitlab")}
                label="Provider:"
                disabled={!vcs}
                layout="grid"
                showGitHub={githubAccounts.length > 0}
                showGitLab={gitlabAccounts.length > 0}
              >
                <>
                  <PillTabs.Content value="github" className="contents">
                    <AccountSelect accounts={githubAccounts} onValueChange={handleSetAccount} disabled={!vcs} />
                    <RepositoryInput repository={repository} setRepository={setRepository} disabled={!vcs} />
                    <BranchInput branch={branch} setBranch={setBranch} disabled={!vcs} />
                  </PillTabs.Content>

                  <PillTabs.Content value="gitlab" className="contents">
                    <AccountSelect accounts={gitlabAccounts} onValueChange={handleSetAccount} disabled={!vcs} />
                    <RepositoryInput repository={repository} setRepository={setRepository} disabled={!vcs} />
                    <BranchInput branch={branch} setBranch={setBranch} disabled={!vcs} />
                  </PillTabs.Content>
                </>
              </VcsProviderSwitcher>
            )}
          </div>
        </div>
      </div>

      <NewAccountModal showModal={isNewAccountModalOpen} closeModal={closeNewAccountModal} />
    </>
  );
};
