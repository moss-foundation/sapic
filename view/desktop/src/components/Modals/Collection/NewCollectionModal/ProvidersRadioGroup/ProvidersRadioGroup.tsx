import { useId } from "react";

import { ProviderIcon } from "./ProviderIcon";

const radioButtonStyles = `
    has-[:focus-visible]:outline-3 
    has-[:focus-visible]:outline-offset-1 
    has-[:focus-visible]:outline-(--moss-primary) 
    
    rounded-full 
    border border-(--moss-border-color)

    hover:not-has-checked:hover:border-black 

    has-checked:ring-2 
    has-checked:ring-offset-0
    has-checked:ring-(--moss-primary) 
`;

export type Provider = "github" | "gitlab" | "postman" | "insomnia";
export type Providers = Array<{
  value: Provider;
  label: string;
  icon: Provider;
}>;

interface ProvidersRadioGroupProps {
  selected: Provider | null;
  setSelected: (selected: Provider | null) => void;
  providers: Providers;
}

export const ProvidersRadioGroup = ({ selected, setSelected, providers }: ProvidersRadioGroupProps) => {
  const uniqueId = useId();

  return (
    <div className="flex gap-2">
      {providers.map((provider) => (
        <div key={provider.value} className={radioButtonStyles}>
          <input
            className="sr-only"
            type="radio"
            id={provider.value}
            name={uniqueId}
            value={provider.value}
            checked={selected === provider.value}
            onChange={(e) => setSelected(e.target.value as Provider)}
          />
          <label className="flex cursor-pointer items-center gap-[5px] py-2 pr-3 pl-2" htmlFor={provider.value}>
            <ProviderIcon icon={provider.icon} />
            <span>{provider.label}</span>
          </label>
        </div>
      ))}
    </div>
  );
};
