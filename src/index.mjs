import { h, Component, render } from 'https://unpkg.com/preact?module';
import htm from 'https://unpkg.com/htm?module';

const html = htm.bind(h);

function App(props){
    return html`
    <div> 
        ${props.cpus.map((cpu) =>{
            return html`
                <div class="bar">
                    <div class="bar-inner" style="width: ${cpu}%"></div>
                    <label>${cpu.toFixed(2)} % usage</label> 
                    
                </div>
            `;
        })}
    </div>
    `;

    
}


// document.addEventListener("DOMContentLoaded", ()=>{
//     let i =0;
//      setInterval(async ()=>{
//         let response = await fetch('/api/cpus');
//         if (response.status != 200){
//             throw new Error(`HTTp error! status: ${response.status}`);
//         }
//         let json = await response.json();
//         // i =i+1;
        
//         //  document.body.textContent = JSON.stringify(json, null, 2);        

//         render(html`<${App} cpus=${json}></${App}>`, document.body);
//      }, 1000)
    
// });
//this is without using websockets 
// let update = async() =>{
//         let response = await fetch('/api/cpus');
//         if (response.status != 200){
//             throw new Error(`HTTp error! status: ${response.status}`);
//         }
//         let json = await response.json();
//         // i =i+1;
        
//         //  document.body.textContent = JSON.stringify(json, null, 2);        

//         render(html`<${App} cpus=${json}></${App}>`, document.body);
// };


// setInterval(update, 1000);
// update();


//this is with using websockets

let url = new URL("/realtime/cpus", window.location.href);
console.log(url);
//http => ws
//https => wss
url.protocol = url.protocol.replace("http", "ws");

console.log(url.protocol);
let ws = new WebSocket(url.href);
ws.onmessage = (ev) =>{
    console.log(ev);
};