import { useRef } from "react";

import { RadioGroup } from "@/components";
import { AccountKind } from "@repo/moss-user";

import { Subheader } from "./Subheader";

interface FlowSectionProps {
  flow: "OAUTH" | "PAT";
  setFlow: (flow: "OAUTH" | "PAT") => void;
  token: string;
  setToken: (token: string) => void;
  provider: AccountKind;
}

export const FlowSection = ({ flow, setFlow, token, setToken, provider }: FlowSectionProps) => {
  const tokenInputRef = useRef<HTMLTextAreaElement>(null);

  return (
    <div>
      <Subheader>
        <span>Flow</span>
        <div className="background-(--moss-border-color) my-auto h-px w-full" />
      </Subheader>
      <p className="text-sm leading-5 text-(--moss-secondary-text)">
        You can switch modes in the workspace at any time and as often as needed.
      </p>
      <div className="pl-5">
        <RadioGroup.Root>
          <RadioGroup.ItemWithLabel
            label="OAuth 2.0"
            description="This mode is suitable when your collection is stored in a separate repository or doesn't have a repository at all."
            value="OAUTH"
            checked={flow === "OAUTH"}
            onClick={() => setFlow("OAUTH")}
            className={flow !== "OAUTH" ? "opacity-50" : ""}
          />

          <RadioGroup.ItemWithLabel
            label="PAT"
            description="This mode is suitable if you want to store the collection in your project's repository or in any other folder you specify."
            value="PAT"
            checked={flow === "PAT"}
            onClick={() => setFlow("PAT")}
            className={flow !== "PAT" ? "opacity-50" : ""}
          />
        </RadioGroup.Root>
      </div>

      {/* PAT Token Input */}
      {flow === "PAT" && (
        <div className="grid grid-cols-[min-content_1fr] items-start gap-x-3 pt-2 pl-10.5">
          <label className="pt-1.5 text-base">Token:</label>
          <textarea
            ref={tokenInputRef}
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder={`${provider === "GITHUB" ? "github.com" : "gitlab.com"}/moss-foundation/sapic`}
            className="h-24.5 w-full resize-none rounded-sm border border-(--moss-border-color) px-2 py-1.5 text-sm placeholder-(--moss-secondary-text) focus:outline-2 focus:outline-(--moss-primary)"
          />
        </div>
      )}
    </div>
  );
};
