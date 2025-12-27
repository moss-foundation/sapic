import { useEffect, useMemo, useState } from "react";

import { useDescribeApp } from "@/hooks";
import { PillTabs } from "@/lib/ui/Tabs/index";
import { useGitProviderStore } from "@/workbench/store/gitProvider";
import { VcsProviderSwitcher } from "@/workbench/ui/components/VcsProviderSwitcher";
import { ImportProjectSource } from "@repo/ipc";

import { AccountSelect } from "../components/AccountSelect";
import { BranchInput } from "../components/BranchInput";
import { NameInput } from "../components/NameInput";
import { RepositoryInput } from "../components/RepositoryInput";
import { DEFAULT_BRANCH, DEFAULT_NAME, DEFAULT_PROVIDER, DEFAULT_REPOSITORY } from "../defaults";
import { Subheader } from "./Subheader";

interface ImportSectionProps {
  onValuesUpdate: (values: { name: string; importParams: ImportProjectSource | undefined }) => void;
}

export const ImportSection = ({ onValuesUpdate }: ImportSectionProps) => {
  const { data: appState } = useDescribeApp();
  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState(DEFAULT_NAME);
  //TODO repository expects input like this: https://github.com/brutusyhy/test-empty-collection.git.
  const [repository, setRepository] = useState(DEFAULT_REPOSITORY);
  const [branch, setBranch] = useState(DEFAULT_BRANCH);
  const [provider, setProvider] = useState<"github" | "gitlab">(DEFAULT_PROVIDER);
  const [accountId, setAccountId] = useState("");

  const importParams = useMemo(() => {
    const params = { repository, branch, accountId };
    const providerMap = {
      github: { gitHub: params },
      gitlab: { gitLab: params },
    } as const;

    return providerMap[provider] ?? undefined;
  }, [repository, branch, accountId, provider]);

  useEffect(() => {
    onValuesUpdate({ name, importParams });
  }, [name, importParams, onValuesUpdate]);

  const handleSetAccount = (accountId: string) => {
    setAccountId(accountId);
  };

  const accounts = useMemo(() => {
    const accountKind = provider === "github" ? "GITHUB" : "GITLAB";
    return appState?.profile?.accounts.filter((account) => account.kind === accountKind) ?? [];
  }, [provider, appState?.profile?.accounts]);

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pb-2">
        <VcsProviderSwitcher
          value={provider}
          onValueChange={(value) => setProvider(value as "github" | "gitlab")}
          label="From:"
          layout="grid"
        >
          <PillTabs.Content value="github" className="contents">
            <NameInput name={name} setName={setName} />
          </PillTabs.Content>
          <PillTabs.Content value="gitlab" className="contents">
            <NameInput name={name} setName={setName} />
          </PillTabs.Content>
        </VcsProviderSwitcher>
      </div>

      <div>
        <Subheader>
          <span>Git</span>
          <div className="background-(--moss-border) my-auto h-px w-full" />
          {gitProvider === null && (
            <button
              className="text-(--moss-primary) cursor-pointer whitespace-nowrap hover:underline"
              onClick={() => {}}
              type="button"
            >
              Log In via GitHub
            </button>
          )}
        </Subheader>

        <span className="text-(--moss-secondary-foreground) text-sm">
          You can switch modes in the workspace at any time and as often as needed.
        </span>

        <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pb-2 pl-5 pt-3">
          <AccountSelect accounts={accounts} onValueChange={handleSetAccount} />

          <RepositoryInput repository={repository} setRepository={setRepository} />

          <BranchInput branch={branch} setBranch={setBranch} />
        </div>
      </div>
    </div>
  );
};
