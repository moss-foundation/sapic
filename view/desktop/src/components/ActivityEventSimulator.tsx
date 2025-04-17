import React, { useState, useRef } from "react";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { ActivityEvent } from "@repo/moss-workbench";

interface ActivityEventSimulatorProps {
  className?: string;
}

export const ActivityEventSimulator: React.FC<ActivityEventSimulatorProps> = ({ className = "" }) => {
  const { clearEvents } = useActivityEvents();
  const [isSimulating, setIsSimulating] = useState(false);
  const [simulationDelay, setSimulationDelay] = useState(1000);
  const [progressEventCount, setProgressEventCount] = useState(10);
  const [oneshotEventCount, setOneshotEventCount] = useState(3);
  const [concurrentProgressEvents, setConcurrentProgressEvents] = useState(2);
  const [priorityTestMode, setPriorityTestMode] = useState(false);
  const [isPaused, setIsPaused] = useState(false);

  const activeTimeoutsRef = useRef<NodeJS.Timeout[]>([]);

  const simulationStateRef = useRef({
    isActive: false,
    isPaused: false,
  });

  const simulationProgressRef = useRef({
    progressSequences: [] as Array<{
      sequenceId: number;
      currentProgress: number;
      totalEvents: number;
      activityType: (typeof activityTypes)[0];
    }>,
    oneshotProgress: 0,
    priorityTestProgress: 0,
  });

  const activityTypes = [
    { title: "Indexing Files", detailFormat: "{current}/{total} indexed" },
    { title: "Building Project", detailFormat: "{current}/{total} compiled" },
    { title: "Downloading Updates", detailFormat: "{current}/{total} MB" },
    { title: "Analyzing Code", detailFormat: "{current}/{total} modules" },
    { title: "Optimizing Assets", detailFormat: "{current}/{total} optimized" },
  ];

  const oneshotTypes = [
    { title: "Git Fetch", detail: "origin/master updated" },
    { title: "File Saved", detail: "All changes saved" },
    { title: "Connection", detail: "Connected to server" },
    { title: "Format", detail: "Document formatted" },
    { title: "Notification", detail: "New update available" },
    { title: "Linting", detail: "No issues found" },
  ];

  const clearAllTimeouts = () => {
    activeTimeoutsRef.current.forEach((timeoutId) => {
      clearTimeout(timeoutId);
    });
    activeTimeoutsRef.current = [];
  };

  const runSimulation = (isResuming = false) => {
    simulationStateRef.current.isActive = true;
    simulationStateRef.current.isPaused = false;

    if (!isResuming) {
      simulationProgressRef.current = {
        progressSequences: [],
        oneshotProgress: 0,
        priorityTestProgress: 0,
      };
    }

    const emitEvent = (event: ActivityEvent, delay: number) => {
      let eventType = "unknown";
      let eventTitle = "";
      let activityId = "";

      if ("start" in event) {
        eventType = "start";
        eventTitle = event.start.title;
        activityId = event.start.activityId;
      } else if ("progress" in event) {
        eventType = "progress";
        eventTitle = "progress";
        activityId = event.progress.activityId;
      } else if ("finish" in event) {
        eventType = "finish";
        activityId = event.finish.activityId;
      } else if ("oneshot" in event) {
        eventType = "oneshot";
        eventTitle = event.oneshot.title;
        activityId = event.oneshot.activityId;
      }

      console.log(`Emitting ${eventType} event (activityId: ${activityId}, title: ${eventTitle}), delay: ${delay}ms`);

      return new Promise<void>((resolve) => {
        if (simulationStateRef.current.isPaused) {
          resolve();
          return;
        }

        const timeoutId = setTimeout(() => {
          if (simulationStateRef.current.isActive && !simulationStateRef.current.isPaused) {
            window.dispatchEvent(
              new CustomEvent("workbench://activity-indicator", {
                detail: { payload: event },
              })
            );
          }

          activeTimeoutsRef.current = activeTimeoutsRef.current.filter((id) => id !== timeoutId);

          resolve();
        }, delay);

        activeTimeoutsRef.current.push(timeoutId);
      });
    };

    const randomDelay = (min: number, max: number) => {
      return Math.floor(Math.random() * (max - min + 1)) + min;
    };

    const simulateOneshotEvents = async (count: number, baseDelay: number) => {
      try {
        const startIndex = simulationProgressRef.current.oneshotProgress;

        for (let i = startIndex; i < count; i++) {
          if (!simulationStateRef.current.isActive) {
            break;
          }

          if (simulationStateRef.current.isPaused) {
            simulationProgressRef.current.oneshotProgress = i;
            break;
          }

          const oneshotType = oneshotTypes[Math.floor(Math.random() * oneshotTypes.length)];

          await emitEvent(
            {
              oneshot: {
                id: 9000 + i,
                activityId: `oneshot-${i}`,
                title: oneshotType.title,
                detail: oneshotType.detail,
              },
            } as ActivityEvent,
            priorityTestMode ? randomDelay(baseDelay * 0.2, baseDelay * 2) : randomDelay(baseDelay * 1.5, baseDelay * 4)
          );

          simulationProgressRef.current.oneshotProgress = i + 1;
        }
      } catch (error) {
        console.error("Error in oneshot simulation:", error);
      }
    };

    // Special function to test priority behavior by interleaving oneshot and progress events
    const simulatePriorityTest = async () => {
      try {
        const baseId = 5000;
        const activityId = "priority-test/progress";
        const activityType = activityTypes[0];

        const currentStep = simulationProgressRef.current.priorityTestProgress;

        if (currentStep === 0) {
          await emitEvent(
            {
              start: {
                id: baseId,
                activityId: activityId,
                title: activityType.title,
              },
            } as ActivityEvent,
            100
          );
        }

        for (let i = currentStep + 1; i <= 10; i++) {
          if (!simulationStateRef.current.isActive) {
            break;
          }

          if (simulationStateRef.current.isPaused) {
            simulationProgressRef.current.priorityTestProgress = i - 1;
            break;
          }

          const detail = activityType.detailFormat.replace("{current}", i.toString()).replace("{total}", "10");

          await emitEvent(
            {
              progress: {
                id: baseId + i,
                activityId: activityId,
                detail: detail,
              },
            } as ActivityEvent,
            300
          );

          if (i % 2 === 0) {
            const oneshotType = oneshotTypes[Math.floor(Math.random() * oneshotTypes.length)];
            await emitEvent(
              {
                oneshot: {
                  id: 9000 + i,
                  activityId: `oneshot-${i}`,
                  title: oneshotType.title,
                  detail: oneshotType.detail,
                },
              } as ActivityEvent,
              randomDelay(100, 300)
            );
          }

          if (simulationStateRef.current.isActive && !simulationStateRef.current.isPaused) {
            await new Promise<void>((resolve) => {
              const timeoutId = setTimeout(resolve, 500);
              activeTimeoutsRef.current.push(timeoutId);
            });
          }

          simulationProgressRef.current.priorityTestProgress = i;
        }

        if (simulationStateRef.current.isActive && !simulationStateRef.current.isPaused) {
          await emitEvent(
            {
              finish: {
                id: baseId + 11,
                activityId: activityId,
              },
            } as ActivityEvent,
            300
          );

          simulationProgressRef.current.priorityTestProgress = 0;
        }
      } catch (error) {
        console.error("Error in priority test simulation:", error);
      }
    };

    const simulateAll = async () => {
      try {
        if (priorityTestMode) {
          await simulatePriorityTest();
        } else {
          const progressPromises: Promise<void>[] = [];

          for (let i = 0; i < concurrentProgressEvents; i++) {
            const activityTypeIndex = i % activityTypes.length;

            const simulateTypedSequence = async () => {
              try {
                const sequenceId = i + 1;
                const baseId = sequenceId * 1000;
                const totalEvents = progressEventCount;
                const baseDelay = simulationDelay;

                const activityType = activityTypes[activityTypeIndex];

                let startingProgress = 0;
                let existingSequence = simulationProgressRef.current.progressSequences.find(
                  (seq) => seq.sequenceId === sequenceId
                );

                if (existingSequence) {
                  startingProgress = existingSequence.currentProgress;
                } else {
                  simulationProgressRef.current.progressSequences.push({
                    sequenceId,
                    currentProgress: 0,
                    totalEvents,
                    activityType,
                  });
                  existingSequence =
                    simulationProgressRef.current.progressSequences[
                      simulationProgressRef.current.progressSequences.length - 1
                    ];
                }

                if (startingProgress === 0) {
                  await emitEvent(
                    {
                      start: {
                        id: baseId,
                        activityId: `test/simulation-${sequenceId}`,
                        title: activityType.title,
                      },
                    } as ActivityEvent,
                    10
                  );

                  await new Promise<void>((resolve) => {
                    const timeoutId = setTimeout(resolve, 50);
                    activeTimeoutsRef.current.push(timeoutId);
                  });
                }

                for (let i = startingProgress + 1; i <= totalEvents; i++) {
                  if (!simulationStateRef.current.isActive) {
                    break;
                  }

                  if (simulationStateRef.current.isPaused) {
                    if (existingSequence) {
                      existingSequence.currentProgress = i - 1;
                    }
                    break;
                  }

                  const detail = activityType.detailFormat
                    .replace("{current}", i.toString())
                    .replace("{total}", totalEvents.toString());

                  await emitEvent(
                    {
                      progress: {
                        id: baseId + i,
                        activityId: `test/simulation-${sequenceId}`,
                        detail: detail,
                      },
                    } as ActivityEvent,
                    priorityTestMode
                      ? randomDelay(baseDelay * 0.2, baseDelay * 0.8)
                      : randomDelay(baseDelay * 0.5, baseDelay * 1.5)
                  );

                  if (existingSequence) {
                    existingSequence.currentProgress = i;
                  }
                }

                if (simulationStateRef.current.isActive && !simulationStateRef.current.isPaused) {
                  await emitEvent(
                    {
                      finish: {
                        id: baseId + totalEvents + 1,
                        activityId: `test/simulation-${sequenceId}`,
                      },
                    } as ActivityEvent,
                    randomDelay(baseDelay * 0.5, baseDelay)
                  );

                  simulationProgressRef.current.progressSequences =
                    simulationProgressRef.current.progressSequences.filter((seq) => seq.sequenceId !== sequenceId);
                }
              } catch (error) {
                console.error(`Error in typed progress simulation ${i + 1}:`, error);
              }
            };

            progressPromises.push(simulateTypedSequence());
          }

          const oneshotPromise = simulateOneshotEvents(oneshotEventCount, simulationDelay);

          await Promise.all([...progressPromises, oneshotPromise]);
        }

        console.log("All simulations completed");
      } catch (error) {
        console.error("Error in simulation:", error);
      } finally {
        if (!simulationStateRef.current.isPaused) {
          setIsSimulating(false);
          simulationStateRef.current.isActive = false;
        }
      }
    };

    simulateAll();
  };

  const simulateActivityEvents = () => {
    if (isSimulating && isPaused) {
      setIsPaused(false);
      runSimulation(true);
      return;
    }

    if (isSimulating && !isPaused) return;

    setIsSimulating(true);
    setIsPaused(false);
    clearEvents();

    runSimulation(false);
  };

  const pauseSimulation = () => {
    setIsPaused(true);
    simulationStateRef.current.isPaused = true;
    clearAllTimeouts();
  };

  const clearSimulation = () => {
    clearAllTimeouts();
    setIsPaused(false);
    setIsSimulating(false);
    simulationStateRef.current.isActive = false;
    simulationStateRef.current.isPaused = false;
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
            disabled={isSimulating || priorityTestMode}
          />
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium">Progress events per sequence</label>
          <input
            type="number"
            min="1"
            max="100"
            value={progressEventCount}
            onChange={(e) => setProgressEventCount(Number(e.target.value))}
            className="w-full rounded border p-2"
            disabled={isSimulating || priorityTestMode}
          />
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium">Number of oneshot events</label>
          <input
            type="number"
            min="0"
            max="20"
            value={oneshotEventCount}
            onChange={(e) => setOneshotEventCount(Number(e.target.value))}
            className="w-full rounded border p-2"
            disabled={isSimulating || priorityTestMode}
          />
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium">Concurrent progress sequences</label>
          <input
            type="number"
            min="1"
            max="5"
            value={concurrentProgressEvents}
            onChange={(e) => setConcurrentProgressEvents(Number(e.target.value))}
            className="w-full rounded border p-2"
            disabled={isSimulating || priorityTestMode}
          />
        </div>
        <div className="col-span-2 mt-2">
          <div className="flex items-center">
            <input
              type="checkbox"
              id="priorityTestMode"
              checked={priorityTestMode}
              onChange={(e) => setPriorityTestMode(e.target.checked)}
              className="relative h-4 w-4 cursor-pointer appearance-none rounded-sm border-2 border-gray-300 bg-white transition-all duration-200 ease-in-out after:absolute after:top-1/2 after:left-1/2 after:-translate-x-1/2 after:-translate-y-1/2 after:text-xs after:text-white after:opacity-0 after:content-['✓'] checked:border-blue-500 checked:bg-blue-500 checked:after:opacity-100 focus:ring-2 focus:ring-blue-300 focus:ring-offset-1 focus:outline-none"
              disabled={isSimulating}
            />
            <label htmlFor="priorityTestMode" className="ml-2 cursor-pointer text-sm font-medium select-none">
              Priority Test Mode
            </label>
            <span className="ml-2 text-xs text-gray-500">(Interleave oneshot events to test priority)</span>
          </div>
        </div>
      </div>

      <div className="flex gap-2">
        <button
          onClick={simulateActivityEvents}
          className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600"
          disabled={isSimulating && !isPaused}
        >
          {isSimulating ? (isPaused ? "Resume" : "Simulating...") : "Start Simulation"}
        </button>
        <button
          onClick={pauseSimulation}
          className="cursor-pointer rounded bg-yellow-500 p-2 text-white hover:bg-yellow-600"
          disabled={!isSimulating || isPaused}
        >
          Pause
        </button>
        <button onClick={clearSimulation} className="cursor-pointer rounded bg-red-500 p-2 text-white hover:bg-red-600">
          Clear
        </button>
      </div>
    </div>
  );
};
