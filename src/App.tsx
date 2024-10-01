import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Button } from "./components/ui/button";
import { Input } from "./components/ui/input";

export function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  const { data } = useQuery({
    queryKey: ["get_commits"],
    queryFn: async () => {
      return await invoke("get_commits");
    },
  });

  async function getCommitInfo(hash: string) {
    const commitInfo = await invoke("get_commit", { hash });
    console.log(commitInfo);
  }

  return (
    <div className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <Input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <Button type="submit">Greet</Button>
      </form>

      <p>{greetMsg}</p>

      <ul>
        {data?.map((commit: any) => (
          <li key={commit.hash} onClick={() => getCommitInfo(commit.hash)}>
            {commit.message}
          </li>
        ))}
      </ul>
    </div>
  );
}
