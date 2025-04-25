import React from "react";
import { useTranslation } from "react-i18next";

import { invokeMossCommand } from "@/lib/backend/platfrom.ts";

import { Button } from "../components";

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

  const intent = ["primary", "neutral"];
  const variant = ["solid", "outlined"];

  return (
    <>
      {data !== null && (
        <p>
          {t("receivedData")}: {data}
        </p>
      )}

      <button
        className="hover:bg-opacity-80 mb-10 rounded-md px-4 py-2 font-medium shadow-sm"
        onClick={() => {
          invokeMossCommand("example.generateLog", {});
        }}
      >
        Example Command
      </button>
      <div>
        <div className="grid grid-cols-[repeat(3,min-content)] gap-4">
          <div></div>
          <div>primary</div>
          <div>neutral</div>
          {variant.map((v) => {
            return intent.map((i, index) => (
              <>
                {index == 0 && <div key={i}>{v}</div>}
                <Button variant={v as any} intent={i as any} size="md" loading={false}>
                  OK
                </Button>
              </>
            ));
          })}
        </div>

        <hr />

        <h2 className="text-xl">Disabled:</h2>

        <div className="grid grid-cols-[repeat(3,min-content)] gap-4">
          <div></div>
          <div>primary</div>
          <div>neutral</div>
          {variant.map((v) => {
            return intent.map((i, index) => (
              <>
                {index == 0 && <div key={i}>{v}</div>}
                <Button disabled variant={v as any} intent={i as any} size="md">
                  {t("button")}
                </Button>
              </>
            ));
          })}
        </div>
        <hr />

        <h2 className="text-xl">Loading:</h2>

        <div className="grid grid-cols-[repeat(3,min-content)] gap-4">
          <div></div>
          <div>primary</div>
          <div>neutral</div>
          {variant.map((v) => {
            return intent.map((i, index) => (
              <>
                {index == 0 && <div key={i}>{v}</div>}
                <Button variant={v as any} intent={i as any} size="md" loading>
                  {t("button")}
                </Button>
              </>
            ));
          })}
        </div>
      </div>
      {/* <Scrollbar className="border border-dotted border-stone-500">
        {Array.from({ length: 100 }, (_, i) => (
          <div key={i} className="mb-1 h-10 w-full text-center">
            {i + 1}
          </div>
        ))}
      </Scrollbar> */}
    </>
  );
};
