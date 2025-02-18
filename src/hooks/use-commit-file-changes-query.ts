import { GitFileChangesResult, Option } from "@/lib/git-types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function useCommitFileChangesQuery(
  commitHash: Option<string>,
  commitFile: Option<string>
) {
  return useQuery({
    queryKey: ["commit_file_changes", commitHash, commitFile],
    queryFn: async () => {
      return await invoke<GitFileChangesResult>("get_commit_file_changes", {
        hash: commitHash,
        file: commitFile,
      });
    },
    enabled: commitHash !== null && commitFile !== null,
  });
}
