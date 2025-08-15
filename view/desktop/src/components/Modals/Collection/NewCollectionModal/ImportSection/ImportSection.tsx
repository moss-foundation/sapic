import { useEffect, useRef, useState } from "react";

import InputOutlined from "@/components/InputOutlined";
import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useGitProviderStore } from "@/store/gitProvider";

import { Provider, Providers, ProvidersRadioGroup } from "../ProvidersRadioGroup/ProvidersRadioGroup";

interface ImportSectionProps {
  onValuesUpdate: (values: { name: string; repository: string; branch: string; provider: Provider | null }) => void;
}

export const ImportSection = ({ onValuesUpdate }: ImportSectionProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { gitProvider } = useGitProviderStore();

  const [name, setName] = useState("New Collection");
  const [repository, setRepository] = useState("github.com/moss-foundation/sapic");
  const [branch, setBranch] = useState("main");
  const [provider, setProvider] = useState<Provider | null>(null);

  const providers: Providers = [
    { value: "github", label: "GitHub", icon: "github" },
    { value: "gitlab", label: "GitLab", icon: "gitlab" },
  ];

  useEffect(() => {
    onValuesUpdate({
      name,
      repository,
      branch,
      provider,
    });
  }, [name, onValuesUpdate, repository, branch, provider]);

  return (
    <div className="flex flex-col gap-2">
      <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 gap-y-6">
        <div>From:</div>
        <ProvidersRadioGroup selected={provider} setSelected={setProvider} providers={providers} />

        <div>Name:</div>
        <InputOutlined
          ref={inputRef}
          value={name}
          className="max-w-72"
          onChange={(e) => setName(e.target.value)}
          pattern={VALID_NAME_PATTERN}
          required
        />
        <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
      </div>

      <div className="flex justify-between gap-2">
        <span>Git</span>
        <div className="background-(--moss-border-color) my-auto h-px w-full" />
        {gitProvider === null && (
          <a className="cursor-pointer whitespace-nowrap text-(--moss-primary) hover:underline" href="">
            Login In via GitHub
          </a>
        )}
      </div>

      <span className="text-xs text-(--moss-secondary-text)">
        You can switch modes in the workspace at any time and as often as needed.
      </span>

      <div className="grid grid-cols-[min-content_1fr] items-center gap-3 py-3 pl-5">
        <div className="col-span-2 grid grid-cols-subgrid items-center">
          <div>Repository:</div>
          <InputOutlined
            ref={inputRef}
            value={repository}
            className="max-w-72"
            onChange={(e) => setRepository(e.target.value)}
            pattern={VALID_NAME_PATTERN}
            required
          />
        </div>

        <div className="col-span-2 grid grid-cols-subgrid items-center">
          <div>Branch:</div>
          <InputOutlined
            ref={inputRef}
            value={branch}
            className="max-w-72"
            onChange={(e) => setBranch(e.target.value)}
            pattern={VALID_NAME_PATTERN}
            required
          />
        </div>
      </div>
    </div>
  );
};
