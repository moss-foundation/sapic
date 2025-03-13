import React from "react";
import { useTranslation } from "react-i18next";

import { ResizablePanel, Scrollbar } from "../components";
import { invokeMossCommand } from "@/lib/backend/platfrom.ts";

export const Home: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  return (
    <div className="p-5 text-[var(--moss-primary)]">
      <h1 className="mb-3 text-2xl">{t("home")}</h1>

      <SessionComponent />
    </div>
  );
};

const SessionComponent = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [data, setData] = React.useState<number | null>(null);

  React.useEffect(() => {
    const fetchData = async () => {
      setData(Math.floor(Math.random() * 100));
    };

    fetchData();
  }, []);

  return (
    <>
      {data !== null && (
        <p>
          {t("receivedData")}: {data}
        </p>
      )}
      {/* An example of `mossCommand` */}
      <button
        onClick={() => {
          invokeMossCommand("example.generateLog", {});
        }}
      >
        Example Command
      </button>
      <ResizablePanel>
        <main className="background-[var(--moss-test-background-1)] text-[var(--moss-test-text-1)]font-sans flex h-screen grow flex-col justify-center text-center transition">
          <Scrollbar>
            {Array.from({ length: 100 }, (_, i) => (
              <div key={i} className="background-[var(--moss-test-background-2)] mb-1 h-10 w-full">
                {i + 1}
              </div>
            ))}
          </Scrollbar>
        </main>
      </ResizablePanel>
    </>
  );
};
