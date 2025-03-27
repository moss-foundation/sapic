import { cn } from "@/utils";

export const TestCollectionIcon = ({ type, className }: { type: string; className?: string }) => {
  switch (type) {
    case "folder":
      return (
        <svg
          className={cn(className, "min-h-4 min-w-4")}
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M8.10584 4.34613L8.25344 4.5H8.46667H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1333C14.5 12.9529 13.932 13.5 13.3667 13.5H2.63333C2.06804 13.5 1.5 12.9529 1.5 12.1333V3.86667C1.5 3.04707 2.06804 2.5 2.63333 2.5H6.1217C6.25792 2.5 6.38824 2.55557 6.48253 2.65387L8.10584 4.34613Z"
            fill="#EBECF0"
            stroke="#6C707E"
          />
        </svg>
      );

    case "hdr":
      return (
        <svg
          className={cn(className, "min-h-4 min-w-4")}
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect width="16" height="16" fill="white" fillOpacity="0.01" />
          <path
            d="M15 7.5V5.5C15 5.23478 14.8946 4.98043 14.7071 4.79289C14.5196 4.60536 14.2652 4.5 14 4.5H11V11.5H12V8.5H12.74L13.91 11.5H15L13.835 8.5H14C14.2652 8.5 14.5196 8.39464 14.7071 8.20711C14.8946 8.01957 15 7.76522 15 7.5ZM12 5.5H14V7.5H12V5.5Z"
            fill="#DB3B4B"
          />
          <path
            d="M8 11.5H6V4.5H8C8.53043 4.5 9.03914 4.71071 9.41421 5.08579C9.78929 5.46086 10 5.96957 10 6.5V9.5C10 10.0304 9.78929 10.5391 9.41421 10.9142C9.03914 11.2893 8.53043 11.5 8 11.5ZM7 10.5H8C8.26522 10.5 8.51957 10.3946 8.70711 10.2071C8.89464 10.0196 9 9.76522 9 9.5V6.5C9 6.23478 8.89464 5.98043 8.70711 5.79289C8.51957 5.60536 8.26522 5.5 8 5.5H7V10.5Z"
            fill="#DB3B4B"
          />
          <path d="M4 4.5V7.5H2V4.5H1V11.5H2V8.5H4V11.5H5V4.5H4Z" fill="#DB3B4B" />
        </svg>
      );

    default:
      return (
        <svg
          className={cn(className, "min-h-4 min-w-4")}
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect width="16" height="16" fill="white" fillOpacity="0.01" />
          <path
            d="M15 11.5H12C11.7349 11.4997 11.4807 11.3942 11.2932 11.2068C11.1058 11.0193 11.0003 10.7651 11 10.5V5.5C11.0003 5.23486 11.1057 4.98066 11.2932 4.79319C11.4807 4.60571 11.7349 4.50026 12 4.5H15V5.5H12V10.5H15V11.5Z"
            fill="#208A3C"
          />
          <path
            d="M9 11.5H7C6.73488 11.4997 6.4807 11.3942 6.29323 11.2068C6.10576 11.0193 6.0003 10.7651 6 10.5V5.5C6.00026 5.23486 6.10571 4.98066 6.29319 4.79319C6.48066 4.60571 6.73486 4.50026 7 4.5H9C9.26513 4.50026 9.51934 4.60571 9.70681 4.79319C9.89429 4.98066 9.99973 5.23486 10 5.5V10.5C9.9997 10.7651 9.89424 11.0193 9.70677 11.2068C9.5193 11.3942 9.26512 11.4997 9 11.5ZM7 5.5V10.5H9V5.5H7Z"
            fill="#208A3C"
          />
          <path
            d="M3 11.5H1V4.5H3C3.53025 4.5006 4.03861 4.7115 4.41356 5.08644C4.7885 5.46139 4.9994 5.96975 5 6.5V9.5C4.9994 10.0303 4.7885 10.5386 4.41356 10.9136C4.03861 11.2885 3.53025 11.4994 3 11.5ZM2 10.5H3C3.26514 10.4997 3.51934 10.3943 3.70681 10.2068C3.89429 10.0193 3.99974 9.76514 4 9.5V6.5C3.99974 6.23486 3.89429 5.98066 3.70681 5.79319C3.51934 5.60571 3.26514 5.50026 3 5.5H2V10.5Z"
            fill="#208A3C"
          />
        </svg>
      );
  }
};
