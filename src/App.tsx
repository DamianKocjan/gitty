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

  async function getCommits() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const commits = await invoke("get_commits");
    console.log(commits);
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

      <Button type="button" onClick={getCommits}>
        Get Commits
      </Button>
      <p>{greetMsg}</p>
    </div>
  );
}
