import React from "react";

import { Resizable, ResizablePanel, Scrollbar } from "../components";

export const Home: React.FC = () => {
  const handleNewWindowButton = async () => {
    // Handle new window button click
  };

  return (
    <div className="p-5 text-[var(--moss-primary)]">
      <h1 className="mb-3 text-2xl">Home</h1>

      <div>
        <span>Icon Placeholder</span>
      </div>

      <SessionComponent />
    </div>
  );
};

const SessionComponent = () => {
  const [data, setData] = React.useState<number | null>(null);

  React.useEffect(() => {
    // Simulate data fetching
    const fetchData = async () => {
      setData(Math.floor(Math.random() * 100));
    };

    fetchData();
  }, []);

  return (
    <>
      <span className="text-[var(--moss-primary)]">Description part 1</span>
      <br />
      <span className="bg-secondary text-[var(--moss-primary)]">Description part 2</span>
      {data !== null && <p>Received data: {data}</p>}
      <ResizablePanel>
        <main className="h-screen flex grow flex-col justify-center text-center background-[var(--moss-test-background-1)] text-[var(--moss-test-text-1)]font-sans transition">
          <Scrollbar>
            {Array.from({ length: 100 }, (_, i) => (
              <div key={i} className="h-10 mb-1 w-full background-[var(--moss-test-background-2)]">
                {i + 1}
              </div>
            ))}
          </Scrollbar>
        </main>
      </ResizablePanel>
    </>
  );
};
