import { useRef } from "react";

import { InputOutlined, RadioGroup } from "@/components";
import { AccountKind } from "@repo/moss-user";

interface FlowSectionProps {
  flow: "OAUTH" | "PAT";
  setFlow: (flow: "OAUTH" | "PAT") => void;
  token: string;
  setToken: (token: string) => void;
  provider: AccountKind;
}

export const FlowSection = ({ flow, setFlow, token, setToken, provider }: FlowSectionProps) => {
  const tokenInputRef = useRef<HTMLInputElement>(null);

  return (
    <div className="flex flex-col gap-1">
      <div>
        <span className="text-base">Flow</span>
      </div>

      <p className="text-sm text-(--moss-secondary-text)">
        You can switch modes in the workspace at any time and as often as needed.
      </p>

      <div className="pt-1.5 pl-4.5">
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
        <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3 pt-2">
          <label className="text-base">Token:</label>
          <InputOutlined
            ref={tokenInputRef}
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder={`${provider === "GITHUB" ? "github.com" : "gitlab.com"}/moss-foundation/sapic`}
            className="w-full"
          />
        </div>
      )}
    </div>
  );
};
