import { useState } from "react";

import { Icon, Icons } from "@/lib/ui";
import Input from "@/lib/ui/Input";

import * as iconsNames from "../../../../assets/icons";
import { KitchenSinkSection } from "../KitchenSinkSection";

export const IconsSection = () => {
  const [iconsSearchInput, setIconsSearchInput] = useState("");

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
            <div key={value} className="flex flex-col items-center gap-2">
              <Icon icon={value as Icons} />
              <span className="cursor-text select-text rounded px-1 hover:bg-gray-100 dark:hover:bg-gray-700">
                {value}
              </span>
            </div>
          ))}
      </div>
    </KitchenSinkSection>
  );
};
