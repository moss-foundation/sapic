import { Icons } from "@/lib/ui/Icon";

export interface StatusBarItem {
  id: number;
  icon: Icons;
  label: string;
  order: number | undefined;
}
