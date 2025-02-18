import { GitCommitsResult } from "@/lib/git-types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function useCommitsQuery() {
  return useQuery({
    queryKey: ["commits"],
    queryFn: async () => {
      return await invoke<GitCommitsResult>("get_commits");
    },
  });
}
