import { h, Component, render } from "https://unpkg.com/preact?module"
document.addEventListener("DOMContentLoaded", () => {
    let i = 0;
    setInterval(async () => {
        i += 1;
        let response = await fetch("/api/cpus");
        let json = await response.json();
        //document.body.textContent = JSON.stringify(json, null, 2);
        const app = h('pre', null, JSON.stringify(json, null, 2));
        render(app, document.body);
    }, 1000);
})