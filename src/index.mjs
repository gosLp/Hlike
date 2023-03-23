import { h, Component, render } from 'https://unpkg.com/preact?module';
import htm from 'https://unpkg.com/htm?module';

const html = htm.bind(h);

function App(props){
    return html`
    <div class= "container"> 
        ${props.cpus.map((cpu) =>{
            return html`
                <div class="bar">
                    <div class="bar-inner" style="height: ${cpu}%"></div>
                    <label>${cpu.toFixed(2)} % usage</label> 
                    
                </div>
            `;
        })}
    </div>
    <div class= "network-container"> 
        ${props.network.map((network) =>{
            return html`
                <div class="network-bar">
                    <label>Recieved Data:</label>
                    ${network[0]}
                    
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

// let url = new URL("/realtime/cpus", window.location.href);
 let url2 = new URL("/realtime/cn", window.location.href);

//http => ws
//https => wss
// url.protocol = url.protocol.replace("http", "ws");
url2.protocol = url2.protocol.replace("http","ws");

let ws2 = new WebSocket(url2.href);

// let ws = new WebSocket(url.href);
// ws.onmessage = (ev) =>{
//     console.log(JSON.parse(ev.data));
//     let json = JSON.parse(ev.data)
//     render(html`<${App} cpus=${json}></${App}>`, document.body);
// };

ws2.onmessage = (ev)=>{
    console.log(JSON.parse(ev.data));
   
    let json = JSON.parse(ev.data);
    console.log(json.network_usage[1][0]);
    render(html`<${App} cpus=${json.cpu_u} network=${json.network_usage}></${App}>`,document.body);
}


//