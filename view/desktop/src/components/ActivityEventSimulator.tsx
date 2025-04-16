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

  // Use a ref to track all active timeouts so we can clear them when pausing
  // Using NodeJS.Timeout for proper typing
  const activeTimeoutsRef = useRef<NodeJS.Timeout[]>([]);

  // Track simulation state
  const simulationStateRef = useRef({
    isActive: false,
    isPaused: false,
  });

  // Add simulation progress tracking
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

  // Sample activity types for more realistic simulations
  const activityTypes = [
    { title: "Indexing Files", detailFormat: "{current}/{total} indexed" },
    { title: "Building Project", detailFormat: "{current}/{total} compiled" },
    { title: "Downloading Updates", detailFormat: "{current}/{total} MB" },
    { title: "Analyzing Code", detailFormat: "{current}/{total} modules" },
    { title: "Optimizing Assets", detailFormat: "{current}/{total} optimized" },
  ];

  // Sample oneshot event types
  const oneshotTypes = [
    { title: "Git Fetch", detail: "origin/master updated" },
    { title: "File Saved", detail: "All changes saved" },
    { title: "Connection", detail: "Connected to server" },
    { title: "Format", detail: "Document formatted" },
    { title: "Notification", detail: "New update available" },
    { title: "Linting", detail: "No issues found" },
  ];

  // Clear all active timeouts
  const clearAllTimeouts = () => {
    activeTimeoutsRef.current.forEach((timeoutId) => {
      clearTimeout(timeoutId);
    });
    activeTimeoutsRef.current = [];
  };

  // Function to actually run the simulation
  const runSimulation = (isResuming = false) => {
    // Update refs to track simulation state
    simulationStateRef.current.isActive = true;
    simulationStateRef.current.isPaused = false;

    // If not resuming, reset progress tracking
    if (!isResuming) {
      simulationProgressRef.current = {
        progressSequences: [],
        oneshotProgress: 0,
        priorityTestProgress: 0,
      };
    }

    // Helper to emit an event with a delay
    const emitEvent = (event: ActivityEvent, delay: number) => {
      // For logging/debugging
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
        // Don't schedule new events if we're paused
        if (simulationStateRef.current.isPaused) {
          resolve();
          return;
        }

        const timeoutId = setTimeout(() => {
          // Only emit the event if we're still active and not paused
          if (simulationStateRef.current.isActive && !simulationStateRef.current.isPaused) {
            // Dispatch the event to the Tauri event system
            window.dispatchEvent(
              new CustomEvent("workbench://activity-indicator", {
                detail: { payload: event },
              })
            );
          }

          // Remove this timeout from the active list
          activeTimeoutsRef.current = activeTimeoutsRef.current.filter((id) => id !== timeoutId);

          resolve();
        }, delay);

        // Store the timeout ID so we can clear it if needed
        activeTimeoutsRef.current.push(timeoutId);
      });
    };

    // Generate a random delay between min and max values
    const randomDelay = (min: number, max: number) => {
      return Math.floor(Math.random() * (max - min + 1)) + min;
    };

    // Function to simulate a single progress event sequence
    const simulateProgressSequence = async (sequenceId: number, totalEvents: number, baseDelay: number) => {
      try {
        const baseId = sequenceId * 1000; // Use base ID to separate different sequences

        // Check if we're resuming this sequence
        let startingProgress = 0;
        let existingSequence = simulationProgressRef.current.progressSequences.find(
          (seq) => seq.sequenceId === sequenceId
        );

        // Get or create activity type for this sequence
        let activityType: (typeof activityTypes)[0];

        if (existingSequence) {
          // Resume from saved progress
          startingProgress = existingSequence.currentProgress;
          activityType = existingSequence.activityType;
        } else {
          // Start new sequence
          activityType = activityTypes[Math.floor(Math.random() * activityTypes.length)];
          // Save this sequence's progress
          simulationProgressRef.current.progressSequences.push({
            sequenceId,
            currentProgress: 0,
            totalEvents,
            activityType,
          });
          existingSequence =
            simulationProgressRef.current.progressSequences[simulationProgressRef.current.progressSequences.length - 1];
        }

        // Start event (only if not resuming or at beginning)
        if (startingProgress === 0) {
          // Ensure start event is sent first with minimal delay
          // This helps with title association for progress events
          await emitEvent(
            {
              start: {
                id: baseId,
                activityId: `test/simulation-${sequenceId}`,
                title: activityType.title,
              },
            } as ActivityEvent,
            10 // Use minimal delay for start events to ensure they're processed before progress events
          );

          // Small delay after start event to ensure it's processed
          await new Promise<void>((resolve) => {
            const timeoutId = setTimeout(resolve, 50);
            activeTimeoutsRef.current.push(timeoutId);
          });
        }

        // Progress events with configurable delay between them
        for (let i = startingProgress + 1; i <= totalEvents; i++) {
          // Check if we're still active and not paused
          if (!simulationStateRef.current.isActive) {
            break;
          }

          if (simulationStateRef.current.isPaused) {
            // Save current progress before breaking
            if (existingSequence) {
              existingSequence.currentProgress = i - 1;
            }
            break;
          }

          // Create detail following the format for this activity type
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
              ? randomDelay(baseDelay * 0.2, baseDelay * 0.8) // Faster in priority test mode
              : randomDelay(baseDelay * 0.5, baseDelay * 1.5)
          );

          // Update progress
          if (existingSequence) {
            existingSequence.currentProgress = i;
          }
        }

        // Finish event
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

          // Remove from progress tracking once finished
          simulationProgressRef.current.progressSequences = simulationProgressRef.current.progressSequences.filter(
            (seq) => seq.sequenceId !== sequenceId
          );
        }
      } catch (error) {
        console.error(`Error in progress simulation ${sequenceId}:`, error);
      }
    };

    // Function to simulate oneshot events
    const simulateOneshotEvents = async (count: number, baseDelay: number) => {
      try {
        // Start from saved progress if resuming
        const startIndex = simulationProgressRef.current.oneshotProgress;

        for (let i = startIndex; i < count; i++) {
          // Check if we're still active and not paused
          if (!simulationStateRef.current.isActive) {
            break;
          }

          if (simulationStateRef.current.isPaused) {
            simulationProgressRef.current.oneshotProgress = i;
            break;
          }

          // Get a random oneshot event type
          const oneshotType = oneshotTypes[Math.floor(Math.random() * oneshotTypes.length)];

          // Emit oneshot event - these will be displayed for exactly 1 second
          await emitEvent(
            {
              oneshot: {
                id: 9000 + i, // Use high IDs to avoid collision with progress events
                activityId: `oneshot-${i}`,
                title: oneshotType.title,
                detail: oneshotType.detail,
              },
            } as ActivityEvent,
            priorityTestMode
              ? randomDelay(baseDelay * 0.2, baseDelay * 2) // More varied timing in priority test mode
              : randomDelay(baseDelay * 1.5, baseDelay * 4)
          );

          // Update progress
          simulationProgressRef.current.oneshotProgress = i + 1;
        }
      } catch (error) {
        console.error("Error in oneshot simulation:", error);
      }
    };

    // Special function to test priority behavior by interleaving oneshot and progress events
    const simulatePriorityTest = async () => {
      try {
        // Create a progress event sequence with a consistent activity
        const baseId = 5000;
        const activityId = "priority-test/progress";
        const activityType = activityTypes[0]; // Use the first activity type consistently

        // Get current progress
        const currentStep = simulationProgressRef.current.priorityTestProgress;

        // Start the progress sequence (only if not resuming)
        if (currentStep === 0) {
          await emitEvent(
            {
              start: {
                id: baseId,
                activityId: activityId,
                title: activityType.title,
              },
            } as ActivityEvent,
            100 // Start quickly
          );
        }

        // Loop to interleave progress and oneshot events
        for (let i = currentStep + 1; i <= 10; i++) {
          // Check if we're still active and not paused
          if (!simulationStateRef.current.isActive) {
            break;
          }

          if (simulationStateRef.current.isPaused) {
            simulationProgressRef.current.priorityTestProgress = i - 1;
            break;
          }

          // Send a progress event
          const detail = activityType.detailFormat.replace("{current}", i.toString()).replace("{total}", "10");

          await emitEvent(
            {
              progress: {
                id: baseId + i,
                activityId: activityId,
                detail: detail,
              },
            } as ActivityEvent,
            300 // Quick progress updates
          );

          // Every other iteration, also send an oneshot event to interrupt
          if (i % 2 === 0) {
            const oneshotType = oneshotTypes[i % oneshotTypes.length];
            await emitEvent(
              {
                oneshot: {
                  id: 8000 + i,
                  activityId: `priority-oneshot-${i}`,
                  title: oneshotType.title,
                  detail: `Interrupting progress #${i}`,
                },
              } as ActivityEvent,
              50 // Very quick oneshot events to interrupt
            );
          }

          // Small delay between iterations
          if (simulationStateRef.current.isActive && !simulationStateRef.current.isPaused) {
            await new Promise<void>((resolve) => {
              const timeoutId = setTimeout(resolve, 500);
              activeTimeoutsRef.current.push(timeoutId);
            });
          }

          // Update progress
          simulationProgressRef.current.priorityTestProgress = i;
        }

        // Finish the progress sequence
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

          // Reset progress
          simulationProgressRef.current.priorityTestProgress = 0;
        }
      } catch (error) {
        console.error("Error in priority test simulation:", error);
      }
    };

    // Simulate both types of sequences
    const simulateAll = async () => {
      try {
        if (priorityTestMode) {
          // Run the special priority test simulation
          await simulatePriorityTest();
        } else {
          const progressPromises: Promise<void>[] = [];

          // Start multiple progress event sequences with different activity types
          for (let i = 0; i < concurrentProgressEvents; i++) {
            // Ensure we use a different activity type for each sequence
            // By using modulo, we cycle through available types if there are more sequences than types
            const activityTypeIndex = i % activityTypes.length;

            // Create a specialized sequence that uses the specific activity type
            const simulateTypedSequence = async () => {
              try {
                const sequenceId = i + 1;
                const baseId = sequenceId * 1000;
                const totalEvents = progressEventCount;
                const baseDelay = simulationDelay;

                // Use the specific activity type instead of random
                const activityType = activityTypes[activityTypeIndex];

                // Check if we're resuming this sequence
                let startingProgress = 0;
                let existingSequence = simulationProgressRef.current.progressSequences.find(
                  (seq) => seq.sequenceId === sequenceId
                );

                if (existingSequence) {
                  // Resume from saved progress
                  startingProgress = existingSequence.currentProgress;
                  // Keep the activity type consistent when resuming
                } else {
                  // Save this sequence's progress with the specific activity type
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

                // Start event (only if not resuming or at beginning)
                if (startingProgress === 0) {
                  // Ensure start event is sent first with minimal delay
                  // This helps with title association for progress events
                  await emitEvent(
                    {
                      start: {
                        id: baseId,
                        activityId: `test/simulation-${sequenceId}`,
                        title: activityType.title,
                      },
                    } as ActivityEvent,
                    10 // Use minimal delay for start events to ensure they're processed before progress events
                  );

                  // Small delay after start event to ensure it's processed
                  await new Promise<void>((resolve) => {
                    const timeoutId = setTimeout(resolve, 50);
                    activeTimeoutsRef.current.push(timeoutId);
                  });
                }

                // Progress events with configurable delay between them
                for (let i = startingProgress + 1; i <= totalEvents; i++) {
                  // Check if we're still active and not paused
                  if (!simulationStateRef.current.isActive) {
                    break;
                  }

                  if (simulationStateRef.current.isPaused) {
                    // Save current progress before breaking
                    if (existingSequence) {
                      existingSequence.currentProgress = i - 1;
                    }
                    break;
                  }

                  // Create detail following the format for this activity type
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
                      ? randomDelay(baseDelay * 0.2, baseDelay * 0.8) // Faster in priority test mode
                      : randomDelay(baseDelay * 0.5, baseDelay * 1.5)
                  );

                  // Update progress
                  if (existingSequence) {
                    existingSequence.currentProgress = i;
                  }
                }

                // Finish event
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

                  // Remove from progress tracking once finished
                  simulationProgressRef.current.progressSequences =
                    simulationProgressRef.current.progressSequences.filter((seq) => seq.sequenceId !== sequenceId);
                }
              } catch (error) {
                console.error(`Error in typed progress simulation ${i + 1}:`, error);
              }
            };

            progressPromises.push(simulateTypedSequence());
          }

          // Start oneshot events
          const oneshotPromise = simulateOneshotEvents(oneshotEventCount, simulationDelay);

          // Wait for all simulations to complete
          await Promise.all([...progressPromises, oneshotPromise]);
        }

        console.log("All simulations completed");
      } catch (error) {
        console.error("Error in simulation:", error);
      } finally {
        // Only reset if we weren't paused
        if (!simulationStateRef.current.isPaused) {
          setIsSimulating(false);
          simulationStateRef.current.isActive = false;
        }
      }
    };

    simulateAll();
  };

  // Main simulation control function
  const simulateActivityEvents = () => {
    // If already simulating and paused, resume the simulation by restarting it
    if (isSimulating && isPaused) {
      // Need to update UI state first
      setIsPaused(false);

      // Then resume the simulation (passing true to indicate resuming)
      runSimulation(true);
      return;
    }

    // Don't start a new simulation if one is already running
    if (isSimulating && !isPaused) return;

    // Start a new simulation
    setIsSimulating(true);
    setIsPaused(false);
    clearEvents();

    // Run the simulation (passing false to indicate new simulation)
    runSimulation(false);
  };

  const pauseSimulation = () => {
    // Set the paused state
    setIsPaused(true);
    simulationStateRef.current.isPaused = true;

    // Clear all pending timeouts
    clearAllTimeouts();
  };

  const clearSimulation = () => {
    // Clear all timeouts
    clearAllTimeouts();

    // Reset all state
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
