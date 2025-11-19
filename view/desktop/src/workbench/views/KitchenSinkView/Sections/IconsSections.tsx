import { useState } from "react";
import { toast } from "sonner";

import { Icon, Icons } from "@/lib/ui";
import Input from "@/lib/ui/Input";

import * as iconsNames from "../../../../assets/icons";
import { KitchenSinkSection } from "../KitchenSinkSection";

export const IconsSection = () => {
  const [iconsSearchInput, setIconsSearchInput] = useState("");

  const handleCopyIcon = (icon: string) => {
    navigator.clipboard.writeText(icon);
    toast.success(`"${icon}" copied to clipboard`, {
      duration: 3000,
    });
  };

  return (
    <KitchenSinkSection header="Icons" description="Various icons available in the application.">
      <div>
        <Input
          intent="outlined"
          value={iconsSearchInput}
          onChange={(e) => setIconsSearchInput(e.target.value)}
          placeholder="Search icons"
        />
      </div>
      <div className="grid grid-cols-6 gap-y-2">
        {Object.keys(iconsNames)
          .filter((value) => {
            if (iconsSearchInput === "") return true;
            return value.toLowerCase().includes(iconsSearchInput.toLowerCase());
          })
          .map((value) => (
            <button key={value} className="flex flex-col items-center gap-2" onClick={() => handleCopyIcon(value)}>
              <Icon icon={value as Icons} />
              <span className="cursor-copy select-text rounded px-1 hover:bg-gray-100 dark:hover:bg-gray-700">
                {value}
              </span>
            </button>
          ))}
      </div>
    </KitchenSinkSection>
  );
};
