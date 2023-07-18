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
    let url = new URL('/realtime/cpus', window.location.href);
    url.protocol = url.protocol.replace('http', 'ws');
    let ws = new WebSocket(url.href);
    ws.onmessage = (ev) => {
        let json = JSON.parse(ev.data);
        console.log(json);
        render(html`<${App} cpus=${json}></${App}>`, document.body);

    };
})