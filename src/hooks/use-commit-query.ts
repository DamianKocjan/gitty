import { GitCommitDetailsResult, Option } from "@/lib/git-types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function useCommitQuery(commitHash: Option<string>) {
  return useQuery({
    queryKey: ["commit", commitHash],
    queryFn: async () => {
      return await invoke<GitCommitDetailsResult>("get_commit", {
        hash: commitHash,
      });
    },
    enabled: commitHash !== null,
  });
}
