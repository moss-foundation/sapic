export const KitchenSinkSection = ({
  children,
  header,
  description,
}: {
  children: React.ReactNode;
  header: string;
  description: string;
}) => {
  return (
    <section className="rounded-xl bg-white p-6 shadow-md dark:bg-stone-800">
      <h2 className="mb-4 text-2xl font-bold text-gray-800 capitalize dark:text-gray-100">{header}</h2>
      <p className="mb-6 text-gray-600 dark:text-gray-300">{description}</p>

      <div className="mb-10 space-y-8">{children}</div>
    </section>
  );
};
