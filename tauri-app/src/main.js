const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;
listen("event_name", (eventPayload) => {
  console.log(eventPayload);
});
console.log(window.__TAURI__);

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function greetMore() {
  greetMsgNl.textContent = await invoke("greet_more", { name: greetInputEl.value });
  // greet_more command gets the input from {name: greetInputEl.value}
}
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  // greetMsgNl = document.querySelector("#greet-more");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
    //greetMore();
  });
});
