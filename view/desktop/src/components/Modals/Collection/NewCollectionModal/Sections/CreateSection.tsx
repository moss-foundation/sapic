import { useEffect, useRef, useState } from "react";

import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { useFocusInputOnMount } from "@/hooks";
import { useAddAccount } from "@/hooks/account/useAddAccount";
import { useGitProviderStore } from "@/store/gitProvider";
import { cn } from "@/utils";
import { CheckedState } from "@radix-ui/react-checkbox";
import { CreateCollectionGitParams } from "@repo/moss-workspace";

import { BranchInput } from "../components/BranchInput";
import { NameInput } from "../components/NameInput";
import ProviderTabs from "../components/ProviderTabs";
import { RepositoryInput } from "../components/RepositoryInput";

interface CreateSectionProps {
  onValuesUpdate: (values: { name: string; gitParams: CreateCollectionGitParams | undefined }) => void;
}

export const CreateSection = ({ onValuesUpdate }: CreateSectionProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { mutateAsync: addAccount } = useAddAccount();

  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState("New Collection");
  const [provider, setProvider] = useState<"github" | "gitlab">("github");
  const [repository, setRepository] = useState("");
  const [branch, setBranch] = useState("main");
  const [vcs, setVCS] = useState(true);

  useFocusInputOnMount({ inputRef });

  useEffect(() => {
    const deriveGitParams = () => {
      if (!vcs) return undefined;

      if (provider === "github") {
        return {
          gitHub: { repository, branch },
        };
      }

      if (provider === "gitlab") {
        return {
          gitLab: { repository, branch },
        };
      }

      return undefined;
    };

    onValuesUpdate({
      name,
      gitParams: deriveGitParams(),
    });
  }, [name, onValuesUpdate, repository, branch, provider, vcs]);

  const handleSetVCS = (checked: CheckedState) => {
    if (checked === "indeterminate") return;
    setVCS(checked);
  };

  const handleAddAccount = () => {
    if (provider === "gitlab") return;

    addAccount({ gitProviderType: "GitHub" });
  };

  return (
    <div className="flex flex-col gap-3">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-x-2 py-3">
        <NameInput name={name} setName={setName} ref={inputRef} />
      </div>

      <div>
        <div className="flex flex-col gap-2">
          <CheckboxWithLabel checked={vcs} onCheckedChange={handleSetVCS} label="VCS" />
          <span className="text-sm text-(--moss-secondary-text)">
            You can switch modes in the workspace at any time and as often as needed.
          </span>
        </div>

        <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pt-3 pb-2 pl-5">
          <ProviderTabs.Root
            value={provider}
            onValueChange={(value) => setProvider(value as "github" | "gitlab")}
            className="contents"
          >
            <ProviderTabs.List className="col-span-2 grid h-min grid-cols-subgrid grid-rows-subgrid">
              <div className={cn(!vcs && "opacity-50")}>Provider:</div>
              <div className="flex gap-2">
                <ProviderTabs.Trigger value="github" label="GitHub" icon="github" disabled={!vcs} />
                <ProviderTabs.Trigger value="gitlab" label="GitLab" icon="gitlab" disabled={!vcs} />
              </div>
            </ProviderTabs.List>

            <>
              <ProviderTabs.Content value="github" className="contents">
                <RepositoryInput repository={repository} setRepository={setRepository} disabled={!vcs} />
                <BranchInput branch={branch} setBranch={setBranch} disabled={!vcs} />
              </ProviderTabs.Content>
              <ProviderTabs.Content value="gitlab" className="contents">
                <RepositoryInput repository={repository} setRepository={setRepository} disabled={!vcs} />
                <BranchInput branch={branch} setBranch={setBranch} disabled={!vcs} />
              </ProviderTabs.Content>
            </>
          </ProviderTabs.Root>
        </div>

        {/* {gitProvider === null && (
          <div className={cn("flex w-full gap-5 py-3 pl-5", !vcs && "opacity-50")}>
            <ButtonPrimary className="px-3 py-1.5" disabled={!vcs} onClick={handleAddAccount}>
              {provider === "github" ? "Log In via GitHub..." : "Log In via GitLab..."}
            </ButtonPrimary>
            <ButtonNeutralOutlined
              className="px-3 py-1.5"
              disabled={!vcs}
              // onClick={handleAddAccount}
            >
              Log In with Token...
            </ButtonNeutralOutlined>
          </div>
        )} */}
      </div>
    </div>
  );
};
