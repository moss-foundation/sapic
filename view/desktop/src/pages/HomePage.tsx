import React from "react";
import { useTranslation } from "react-i18next";

import { invokeMossCommand } from "@/lib/backend/platfrom.ts";

import { Scrollbar } from "../components";

export const Home = () => {
  const { t } = useTranslation(["ns1", "ns2"]);

  return (
    <div className="flex h-full flex-col p-5">
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
        className="hover:bg-opacity-80 mb-10 rounded-md px-4 py-2 font-medium shadow-sm"
        onClick={() => {
          invokeMossCommand("example.generateLog", {});
        }}
      >
        Example Command
      </button>

      <Scrollbar className="border border-stone-500">
        {Array.from({ length: 100 }, (_, i) => (
          <div key={i} className="mb-1 h-10 w-full text-center">
            {i + 1}
          </div>
        ))}
      </Scrollbar>
    </>
  );
};
