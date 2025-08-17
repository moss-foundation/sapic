import { useEffect, useState } from "react";

import InputOutlined from "@/components/InputOutlined";
import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useAddAccount } from "@/hooks/account/useAddAccount";
import { useGitProviderStore } from "@/store/gitProvider";
import { ImportCollectionSource } from "@repo/moss-workspace";

import ProviderTabs from "../ProviderTabs/ProviderTabs";
import { Subheader } from "../Subheader";

interface ImportSectionProps {
  onValuesUpdate: (values: { name: string; importParams: ImportCollectionSource | undefined }) => void;
}

export const ImportSection = ({ onValuesUpdate }: ImportSectionProps) => {
  const { mutateAsync: addAccount } = useAddAccount();
  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState("New Collection");
  //TODO repository expects input like this: https://github.com/brutusyhy/test-empty-collection.git.
  const [repository, setRepository] = useState("");
  const [branch, setBranch] = useState("main");
  const [provider, setProvider] = useState<"github" | "gitlab">("github");

  useEffect(() => {
    const deriveGitParams = () => {
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
      importParams: deriveGitParams(),
    });
  }, [name, onValuesUpdate, repository, branch, provider]);

  const handleAddAccount = () => {
    if (provider === "gitlab") return;

    addAccount({ gitProviderType: "GitHub" });
  };

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pb-2">
        <ProviderTabs.Root
          value={provider}
          onValueChange={(value) => setProvider(value as "github" | "gitlab")}
          className="contents"
        >
          <ProviderTabs.List className="col-span-2 grid h-min grid-cols-subgrid grid-rows-subgrid">
            <div>From:</div>
            <div className="flex gap-2">
              <ProviderTabs.Trigger value="github" label="GitHub" icon="github" />
              <ProviderTabs.Trigger value="gitlab" label="GitLab" icon="gitlab" />
            </div>
          </ProviderTabs.List>

          <ProviderTabs.Content value="github" className="contents">
            <div className="col-span-2 grid grid-cols-subgrid items-center gap-y-1.5">
              <div>Name:</div>
              <InputOutlined
                value={name}
                className="max-w-72"
                onChange={(e) => setName(e.target.value)}
                pattern={VALID_NAME_PATTERN}
                required
              />
              <p className="col-start-2 max-w-72 text-sm whitespace-pre text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |)\nwill be escaped`}</p>
            </div>
          </ProviderTabs.Content>
          <ProviderTabs.Content value="gitlab" className="contents">
            <div className="col-span-2 grid grid-cols-subgrid items-center gap-y-1.5">
              <div>Name:</div>
              <InputOutlined
                value={name}
                className="max-w-72"
                onChange={(e) => setName(e.target.value)}
                pattern={VALID_NAME_PATTERN}
                required
              />
              <p className="col-start-2 max-w-72 text-sm whitespace-pre text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |)\nwill be escaped`}</p>
            </div>
          </ProviderTabs.Content>
        </ProviderTabs.Root>
      </div>

      <div>
        <Subheader>
          <span>Git</span>
          <div className="background-(--moss-border-color) my-auto h-px w-full" />
          {gitProvider === null && (
            <button
              className="cursor-pointer whitespace-nowrap text-(--moss-primary) hover:underline"
              onClick={handleAddAccount}
              type="button"
            >
              Log In via GitHub
            </button>
          )}
        </Subheader>

        <span className="text-sm text-(--moss-secondary-text)">
          You can switch modes in the workspace at any time and as often as needed.
        </span>

        <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6 pt-3 pb-2 pl-5">
          <div className="col-span-2 grid grid-cols-subgrid items-center">
            <div>Repository:</div>
            <InputOutlined
              value={repository}
              className="max-w-72"
              onChange={(e) => setRepository(e.target.value)}
              placeholder="github.com/user/example.git"
              required
            />
          </div>

          <div className="col-span-2 grid grid-cols-subgrid items-center">
            <div>Branch:</div>
            <InputOutlined
              value={branch}
              className="max-w-72"
              onChange={(e) => setBranch(e.target.value)}
              pattern={VALID_NAME_PATTERN}
              required
            />
          </div>
        </div>
      </div>
    </div>
  );
};
