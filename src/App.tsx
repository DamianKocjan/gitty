import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import React, { useState } from "react";
import { Button } from "./components/ui/button";
import {
  GitBranchListResult,
  GitCommitDetailsResult,
  GitCommitsResult,
  GitFileChangesResult,
  OpenRepositoryResult,
} from "./lib/git-types";
import { cn } from "./lib/utils";

export function App() {
  const [activeCommitHash, setActiveCommitHash] = useState("");
  const [activeCommitFile, setActiveCommitFile] = useState("");

  const commitsQuery = useQuery({
    queryKey: ["commits"],
    queryFn: async () => {
      return await invoke<GitCommitsResult>("get_commits");
    },
  });

  const commitInfoQuery = useQuery({
    queryKey: ["commit", activeCommitHash],
    queryFn: async () => {
      return await invoke<GitCommitDetailsResult>("get_commit", {
        hash: activeCommitHash,
      });
    },
    enabled: !!activeCommitHash,
  });

  React.useEffect(() => {
    if (commitInfoQuery.data) {
      setActiveCommitFile(commitInfoQuery.data.diff[0].file_name);
    }
  }, [commitInfoQuery.data]);

  const commitFileChangesQuery = useQuery({
    queryKey: ["commit_file_changes", activeCommitHash, activeCommitFile],
    queryFn: async () => {
      return await invoke<GitFileChangesResult>("get_commit_file_changes", {
        hash: activeCommitHash,
        file: activeCommitFile,
      });
    },
    enabled: !!activeCommitHash && !!activeCommitFile,
  });

  const branchListQuery = useQuery({
    queryKey: ["branches"],
    queryFn: async () => {
      return await invoke<GitBranchListResult>("get_branch_list");
    },
  });

  const openRepositoryMutation = useMutation({
    mutationFn: async () => {
      return await invoke<OpenRepositoryResult>("open_repository");
    },
    onSuccess(success) {
      if (!success) {
        console.error("Failed to open repository");

        return;
      }

      commitsQuery.refetch();
      branchListQuery.refetch();
    },
  });

  return (
    <div className="container">
      <Button onClick={() => openRepositoryMutation.mutate()}>
        change repo
      </Button>

      <h2>Branches</h2>
      <select>
        {branchListQuery.data?.map((branch) => (
          <option
            key={branch.name}
            value={branch.name}
            className={cn(
              branch.is_current && "font-bold",
              branch.is_head && "font-italic",
              branch.is_remote && "text-red-500"
            )}
          >
            {branch.name}
          </option>
        ))}
      </select>

      <h2>Commit Info</h2>

      {!!commitInfoQuery.data && (
        <ul>
          <li>
            <strong>Author:</strong> {commitInfoQuery.data.commit.author}
          </li>
          <li>
            <strong>Message:</strong> {commitInfoQuery.data.commit.message}
          </li>
          <li>
            <strong>Files:</strong>
            <ul>
              {commitInfoQuery.data.diff.map((file) => (
                <li
                  key={file.file_name}
                  onClick={() => setActiveCommitFile(file.file_name)}
                  className={
                    activeCommitFile === file.file_name ? "font-bold" : ""
                  }
                >
                  {file.file_name}

                  {activeCommitFile === file.file_name && (
                    <p className="outline outline-1 outline-black">
                      {commitFileChangesQuery.data || "NOT WORKING"}
                    </p>
                  )}
                </li>
              ))}
            </ul>
          </li>
        </ul>
      )}

      <h2>Commits</h2>

      <ul>
        {commitsQuery.data?.map((commit) => (
          <li
            key={commit.hash}
            onClick={() => setActiveCommitHash(commit.hash)}
          >
            {commit.message}
          </li>
        ))}
      </ul>
    </div>
  );
}
