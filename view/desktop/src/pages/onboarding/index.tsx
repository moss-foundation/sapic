import { invokeTauriIpc } from "@/infra/ipc/tauri.ts";

export const OnboardingPage = () => {
  const handleComplete = async () => {
    const result = await invokeTauriIpc("onboarding__complete_onboarding");

    if (result.status === "error") {
      throw new Error(String(result.status));
    }
  };

  return (
    <div className="grid h-screen w-full place-items-center" data-tauri-drag-region>
      <div className="flex flex-col gap-6">
        <h1 className="text-3xl font-bold">Welcome to Sapic</h1>

        <div className="flex flex-col gap-4">
          <p className="text-(--moss-secondary-foreground)">Let's get you started with the onboarding process.</p>

          <button
            className="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-600"
            onClick={handleComplete}
          >
            Complete Onboarding
          </button>
          {/* Placeholder for onboarding steps */}
          <div className="flex flex-col gap-2">{/* Onboarding content will go here */}</div>
        </div>
      </div>
    </div>
  );
};
