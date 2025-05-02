import React from "react";

const colors = [
  "rgba(255,0,0,0.2)",
  "rgba(0,255,0,0.2)",
  "rgba(0,0,255,0.2)",
  "rgba(255,255,0,0.2)",
  "rgba(0,255,255,0.2)",
  "rgba(255,0,255,0.2)",
];
let count = 0;

export const useDockviewLogger = () => {
  const [logLines, setLogLines] = React.useState<{ text: string; timestamp?: Date; backgroundColor?: string }[]>([]);
  const [pending, setPending] = React.useState<{ text: string; timestamp?: Date }[]>([]);

  const addLogLine = (message: string) => {
    setPending((lines) => [{ text: message, timestamp: new Date() }, ...lines]);
  };

  React.useLayoutEffect(() => {
    if (pending.length === 0) return;

    const color = colors[count++ % colors.length];
    setLogLines((lines) => [...pending.map((line) => ({ ...line, backgroundColor: color })), ...lines]);
    setPending([]);
  }, [pending]);

  return { logLines, addLogLine, setLogLines };
};
