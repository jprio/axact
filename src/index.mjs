import { h, Component, render } from "https://unpkg.com/preact?module"
import htm from "https://unpkg.com/htm?module"
const html = htm.bind(h);
function App(props) {
    return html`
    <div>
        ${props.cpus.map((cpu) => {
        return html`
        <div class="bar">
            <div class="bar-inner" style="width: ${cpu}%">${cpu.toFixed(2)}%</div>
        </div>`;
    })}
    </div>
    `;
}
document.addEventListener("DOMContentLoaded", () => {
    let i = 0;
    setInterval(async () => {
        i += 1;
        let response = await fetch("/api/cpus");
        let json = await response.json();
        console.log(json);
        render(html`<${App} cpus=${json}></${App}>`, document.body);
    }, 1000);
})