export const OnboardingPage = () => {
  return (
    <div className="grid h-screen w-full place-items-center" data-tauri-drag-region>
      <div className="flex flex-col gap-6">
        <h1 className="text-3xl font-bold">Welcome to Sapic</h1>

        <div className="flex flex-col gap-4">
          <p className="text-(--moss-secondary-foreground)">Let's get you started with the onboarding process.</p>

          {/* Placeholder for onboarding steps */}
          <div className="flex flex-col gap-2">{/* Onboarding content will go here */}</div>
        </div>
      </div>
    </div>
  );
};
