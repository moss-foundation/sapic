import React, { useState } from "react";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { ActivityEvent } from "@repo/moss-workbench";

interface ActivityEventSimulatorProps {
  className?: string;
}

export const ActivityEventSimulator: React.FC<ActivityEventSimulatorProps> = ({ className = "" }) => {
  const { clearEvents } = useActivityEvents();
  const [isSimulating, setIsSimulating] = useState(false);
  const [simulationDelay, setSimulationDelay] = useState(1000);
  const [simulationCount, setSimulationCount] = useState(10);
  const [isPaused, setIsPaused] = useState(false);
  const [currentSimulation, setCurrentSimulation] = useState<number | null>(null);

  // Simulated activity events function with delays
  const simulateActivityEvents = () => {
    if (isSimulating) return;

    setIsSimulating(true);
    setIsPaused(false);
    clearEvents();

    // Helper to emit an event with a delay
    const emitEvent = (event: ActivityEvent, delay: number) => {
      return new Promise<void>((resolve) => {
        const timeoutId = setTimeout(() => {
          if (!isPaused) {
            // Dispatch the event to the Tauri event system
            window.dispatchEvent(
              new CustomEvent("workbench://activity-indicator", {
                detail: { payload: event },
              })
            );
            resolve();
          }
        }, delay);

        // Store the current timeout ID to be able to clear it
        // Convert to number for React state
        setCurrentSimulation(Number(timeoutId));
      });
    };

    // Simulate a sequence of events
    const simulateSequence = async () => {
      try {
        // Start event
        await emitEvent(
          {
            start: {
              id: 0,
              activityId: "test/simulation",
              title: "Test Indexing",
            },
          } as ActivityEvent,
          0
        );

        // Progress events with configurable delay between them
        for (let i = 1; i <= simulationCount; i++) {
          if (isPaused) {
            break;
          }
          await emitEvent(
            {
              progress: {
                id: i,
                activityId: "test/simulation",
                detail: `${i}/${simulationCount} (Simulated file ${i})`,
              },
            } as ActivityEvent,
            simulationDelay
          );
        }

        // Finish event
        if (!isPaused) {
          await emitEvent(
            {
              finish: {
                id: simulationCount + 1,
                activityId: "test/simulation",
              },
            } as ActivityEvent,
            simulationDelay
          );
        }

        console.log("Simulation completed");
      } catch (error) {
        console.error("Error in simulation:", error);
      } finally {
        setIsSimulating(false);
        setCurrentSimulation(null);
      }
    };

    simulateSequence();
  };

  const pauseSimulation = () => {
    setIsPaused(true);
  };

  const clearSimulation = () => {
    if (currentSimulation) {
      clearTimeout(currentSimulation);
      setCurrentSimulation(null);
    }
    setIsPaused(false);
    setIsSimulating(false);
    clearEvents();
  };

  return (
    <div className={`rounded-md border p-4 ${className}`}>
      <h2 className="mb-2 text-xl">Activity Event Simulator</h2>

      <div className="mb-4 grid grid-cols-2 gap-4">
        <div>
          <label className="mb-1 block text-sm font-medium">Delay between events (ms)</label>
          <input
            type="number"
            min="100"
            max="10000"
            step="100"
            value={simulationDelay}
            onChange={(e) => setSimulationDelay(Number(e.target.value))}
            className="w-full rounded border p-2"
            disabled={isSimulating}
          />
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium">Number of events</label>
          <input
            type="number"
            min="1"
            max="100"
            value={simulationCount}
            onChange={(e) => setSimulationCount(Number(e.target.value))}
            className="w-full rounded border p-2"
            disabled={isSimulating}
          />
        </div>
      </div>

      <div className="flex gap-2">
        <button
          onClick={simulateActivityEvents}
          className="cursor-pointer rounded bg-green-500 p-2 text-white"
          disabled={isSimulating}
        >
          {isSimulating ? "Simulating..." : "Start Simulation"}
        </button>
        <button
          onClick={pauseSimulation}
          className="cursor-pointer rounded bg-yellow-500 p-2 text-white"
          disabled={!isSimulating || isPaused}
        >
          Pause
        </button>
        <button onClick={clearSimulation} className="cursor-pointer rounded bg-red-500 p-2 text-white">
          Clear
        </button>
      </div>
    </div>
  );
};
