import { OverlayScrollbarsComponent, OverlayScrollbarsComponentProps } from "overlayscrollbars-react";

const defaultOptions: OverlayScrollbarsComponentProps = {
  options: {
    scrollbars: {
      autoHide: "move",
    },
  },

  defer: true,
};

export const Scrollbar = ({ children, options, className, ...props }: OverlayScrollbarsComponentProps) => {
  const combinedOptions = {
    ...defaultOptions.options,
    ...options,
  };

  return (
    <OverlayScrollbarsComponent className={className} options={combinedOptions} defer={defaultOptions.defer} {...props}>
      {children}
    </OverlayScrollbarsComponent>
  );
};

export default Scrollbar;
