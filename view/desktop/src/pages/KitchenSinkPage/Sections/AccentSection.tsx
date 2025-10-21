import { useEffect, useState } from "react";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const AccentSection = () => {
  const [accent, setAccent] = useState("#3574f0");

  const handleAccentChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setAccent(e.target.value);
    document.documentElement.style.setProperty("--moss-accent", e.target.value);
  };

  useEffect(() => {
    setTimeout(() => {
      setAccent(document.documentElement.style.getPropertyValue("--moss-accent"));
    }, 5000);
  }, []);

  return (
    <KitchenSinkSection header="Accent" description="The accent color of the application.">
      <div className="flex flex-col gap-2">
        <input type="color" value={accent} onChange={handleAccentChange} />
      </div>
    </KitchenSinkSection>
  );
};
