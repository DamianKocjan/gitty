type Option<T> = T | null;

export type GitCommit = {
  hash: string;
  author: string;
  timestamp: number;
  message: string;
};
export type GitCommitsResult = GitCommit[];

export type GitCommitFileDiff = {
  file_name: string;
  additions: number;
  deletions: number;
};
export type GitCommitDetails = {
  commit: GitCommit & {
    description: string;
  };
  diff: GitCommitFileDiff[];
};
export type GitCommitDetailsResult = Option<GitCommitDetails>;

export type GitFileChanges = string;
export type GitFileChangesResult = Option<GitFileChanges>;
