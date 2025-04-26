import { AppLayout, RootLayout } from "@/components";
import { PageLoader } from "@/components/PageLoader";
import { usePrepareWindow } from "@/hooks/usePrepareWindow";

const App = () => {
  const { isPreparing } = usePrepareWindow();

  if (isPreparing) {
    return <PageLoader />;
  }

  return (
    <RootLayout>
      <AppLayout />
    </RootLayout>
  );
};

export default App;
