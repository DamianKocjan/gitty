import { OpenRepositoryResult } from "@/lib/git-types";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function useOpenRepositoryMutation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      return await invoke<OpenRepositoryResult>("open_repository");
    },
    async onSuccess(success) {
      if (success === false) {
        console.error("Failed to open repository");

        return;
      }

      await queryClient.invalidateQueries({
        queryKey: ["commits"],
      });
      await queryClient.invalidateQueries({
        queryKey: ["branch_list"],
      });
    },
  });
}
