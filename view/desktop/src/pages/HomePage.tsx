import React from "react";

export const Home: React.FC = () => {
  const handleNewWindowButton = async () => {
    // Handle new window button click
  };

  return (
    <div className="p-5 text-[var(--moss-primary)]">
      <h1 className="mb-3 text-2xl">Home</h1>

      <button className="mb-2 rounded !bg-green-500 p-1" onClick={handleNewWindowButton}>
        New Window
      </button>

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
    </>
  );
};
