import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import React, { useState } from "react";
import {
  GitCommitDetailsResult,
  GitCommitsResult,
  GitFileChangesResult,
} from "./lib/git-types";

export function App() {
  const [activeCommitHash, setActiveCommitHash] = useState("");
  const [activeCommitFile, setActiveCommitFile] = useState("");

  const { data } = useQuery({
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

  return (
    <div className="container">
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
              {commitInfoQuery.data.diff.map((file: any) => (
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
        {data?.map((commit: any) => (
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
