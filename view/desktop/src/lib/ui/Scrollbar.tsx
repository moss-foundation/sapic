import { OverlayScrollbarsComponent, OverlayScrollbarsComponentProps } from "overlayscrollbars-react";

const defaultOptions: OverlayScrollbarsComponentProps = {
  options: {
    scrollbars: {
      autoHide: "move",
    },
  },
  defer: true,
};

export const Scrollbar = ({ children, options, ...props }: OverlayScrollbarsComponentProps) => {
  const combinedOptions = {
    ...defaultOptions.options,
    ...options,
  };

  return (
    <OverlayScrollbarsComponent options={combinedOptions} defer={defaultOptions.defer} {...props}>
      {children}
    </OverlayScrollbarsComponent>
  );
};

export default Scrollbar;
