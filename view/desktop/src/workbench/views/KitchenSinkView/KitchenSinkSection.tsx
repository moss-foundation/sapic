import { cn } from "@/utils";

interface KitchenSinkSectionProps extends React.HTMLAttributes<HTMLDivElement> {
  header: string;
  description?: string;
}

export const KitchenSinkSection = ({ children, header, description, className, ...props }: KitchenSinkSectionProps) => {
  return (
    <section className={cn("rounded-xl bg-white p-6 shadow-md dark:bg-stone-800", className)} {...props}>
      <h2 className="mb-4 text-2xl font-bold capitalize text-gray-800 dark:text-gray-100">{header}</h2>
      {description && <p className="mb-6 text-gray-600 dark:text-gray-300">{description}</p>}

      <div className="mb-10 space-y-8">{children}</div>
    </section>
  );
};
