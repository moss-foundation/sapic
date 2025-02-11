import { useState } from "react";
import reactLogo from "./assets/react.svg";

function App() {
  const [name, setName] = useState("");


  async function greet() {
    alert("Greeting " + name + "...");
  }

  const [theme, setTheme] = useState("dark");

  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
    document.querySelector("html")!.setAttribute("data-theme", theme === "dark" ? "light" : "dark");
  };

  return (
    <>
      <div className="absolute -top-3 -right-3 p-4 flex" >
        <div className="" />
        <button onClick={toggleTheme} className="cursor-pointer">
          {theme === "light" ? (
            <svg className="size-9 text-black hover:text-gray-500 " xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" /></svg>
          ) : (
            <svg className="size-9 text-white hover:text-black/50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4" /><path d="M12 2v2" /><path d="M12 20v2" /><path d="m4.93 4.93 1.41 1.41" /><path d="m17.66 17.66 1.41 1.41" /><path d="M2 12h2" /><path d="M20 12h2" /><path d="m6.34 17.66-1.41 1.41" /><path d="m19.07 4.93-1.41 1.41" /></svg>
          )}
        </button>
      </div>

      <main className="h-full  flex flex-col justify-center text-center bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-sans transition">
        <h1 className="text-center text-2xl font-bold">Welcome to Tauri + React</h1>

        <div className="flex justify-center mt-4">
          <a href="https://vitejs.dev" target="_blank">
            <img
              src="/vite.svg"
              className="size-38 p-6  duration-700 will-change-[filter] hover:drop-shadow-[0_0_2em_#747bff]"
              alt="Vite logo"
            />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img
              src="/tauri.svg"
              className="size-38 p-6 duration-700 will-change-[filter] hover:drop-shadow-[0_0_2em_#24c8db]"
              alt="Tauri logo"
            />
          </a>
          <a href="https://reactjs.org" target="_blank">
            <img
              src={reactLogo}
              className="size-38 p-6 duration-700 will-change-[filter] hover:drop-shadow-[0_0_2em_#61dafb]"
              alt="React logo"
            />
          </a>
        </div>
        <p className="mt-4">Click on the Tauri, Vite, and React logos to learn more.</p>

        <form
          className="flex justify-center mt-4"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
            className="mr-2 border border-transparent rounded-lg px-4 py-2 text-gray-900 bg-white shadow-md duration-200 focus:outline-none focus:border-blue-500 dark:text-white dark:bg-gray-900 dark:focus:border-blue-400"
          />
          <button
            type="submit"
            className="border border-transparent rounded-lg px-4 py-2 font-medium text-gray-900 bg-white shadow-md cursor-pointer duration-200 hover:border-blue-500 active:border-blue-500 active:bg-gray-200 dark:text-white dark:bg-gray-900 dark:active:bg-gray-700"
          >
            Greet
          </button>
        </form>
      </main>
    </>
  );
}

export default App;