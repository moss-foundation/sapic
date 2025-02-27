import React from "react";

export const Settings = () => {
  const handleLanguageChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    // Handle language change
  };

  const handleThemeChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    // Handle theme change
  };

  return (
    <main>
      <div className="p-5 text-[var(--moss-primary)]">
        <h1 className="mb-3 text-2xl">Settings</h1>
        <div>
          <h3>Select Language</h3>
          <select id="lang-select" className="rounded border bg-gray-400 p-2" onChange={handleLanguageChange}>
            {/* Language options */}
          </select>
        </div>

        <div>
          <h3>Select Theme</h3>
          <select id="theme-select" className="rounded border bg-gray-400 p-2" onChange={handleThemeChange}>
            {/* Theme options */}
          </select>
        </div>
      </div>
    </main>
  );
};
