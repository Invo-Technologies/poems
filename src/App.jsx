import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [message, setMessage] = useState("");

  async function runDecrypt() {
    await invoke("decrypt_command");
    setMessage("Decryption command executed!");
  }

  async function runRegister() {
    await invoke("register_command");
    setMessage("Registration command executed!");
  }

  async function runEnv() {
    await invoke("env_command");
    setMessage("Environment command executed!");
  }

  return (
    <div className="container">
      <h1>POEMS ALEO DEMO</h1>
      <button onClick={runDecrypt}>Run Decrypt</button>
      <button onClick={runRegister}>Run Register</button>
      <button onClick={runEnv}>Run Env</button>
      <p>{message}</p>
    </div>
  );
}

export default App;
