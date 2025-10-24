import { openUrl } from "@tauri-apps/plugin-opener";

type ExternalLinkProps = {
  text: string;
  href: string;
  className?: string;
};

export default function ExternalLink({ text, href, className }: ExternalLinkProps) {
  async function handleClick() {
    try {
      await openUrl(href);
    } catch (err) {
      console.log("Failed to open url in browser:", err);
    }
  }

  return (
    <div onClick={handleClick} className={className}>
      {text}
    </div>
  );
}
