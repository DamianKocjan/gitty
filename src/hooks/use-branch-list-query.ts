import { GitBranchListResult } from "@/lib/git-types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function useBranchListQuery() {
  return useQuery({
    queryKey: ["branch_list"],
    queryFn: async () => {
      return await invoke<GitBranchListResult>("get_branch_list");
    },
  });
}
