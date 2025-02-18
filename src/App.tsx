import React, { useState } from "react";
import { Button } from "./components/ui/button";
import { useBranchListQuery } from "./hooks/use-branch-list-query";
import { useCommitFileChangesQuery } from "./hooks/use-commit-file-changes-query";
import { useCommitQuery } from "./hooks/use-commit-query";
import { useCommitsQuery } from "./hooks/use-commits-query";
import { useOpenRepositoryMutation } from "./hooks/use-open-repository-mutation";
import { Option } from "./lib/git-types";
import { cn } from "./lib/utils";

export function App() {
  const [activeCommitHash, setActiveCommitHash] =
    useState<Option<string>>(null);
  const [activeCommitFile, setActiveCommitFile] =
    useState<Option<string>>(null);

  const commitsQuery = useCommitsQuery();

  const commitInfoQuery = useCommitQuery(activeCommitHash);

  React.useEffect(() => {
    if (commitInfoQuery.data) {
      setActiveCommitFile(commitInfoQuery.data.diff[0].file_name);
    }
  }, [commitInfoQuery.data]);

  const commitFileChangesQuery = useCommitFileChangesQuery(
    activeCommitHash,
    activeCommitFile
  );

  const branchListQuery = useBranchListQuery();

  const openRepositoryMutation = useOpenRepositoryMutation();

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
