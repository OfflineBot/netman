const { invoke } = window.__TAURI__.tauri;

let network_box = document.querySelector('#wifi-options');
let message_box = document.querySelector('#message-box');

window.connect_network = connect_network;

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function get_network() {
    let networks = await invoke("get_network", {});
    return networks;
}

async function connect_network(network_name, id) {
    let input_id = `input_${id}`;
    console.log(input_id);
    let password = document.getElementById(`input_${id}`);
    let pass = password.value;

    let output = await invoke("connect_network", {name: network_name, password: pass});

    message_box.innerText = output;
    await sleep(2000);
    message_box.innerText = "";
}

async function fully_load_network() {
    let networks = await get_network();
    if (networks.length === 0) {
        network_box.innerHTML = '<div class="connection">Coudnt get any Network connections!</div>';
        return;
    }
    render_network(networks);
}

function get_network_template(network_name, id) {
    return `
        <div class="connection">
        <p class="network-name">${network_name}</p>
        <input id="input_${id}" placeholder="Password"/>
        <button onclick="connect_network('${network_name}', '${id}')">Connect</button>
        </div>
    `;
}

function render_network(networks) {
    network_box.innerHTML = "";
    for (let i = 0; i < networks.length; i++) {
        network_box.innerHTML += get_network_template(networks[i], i);
    }
}

window.addEventListener("DOMContentLoaded", async () => {
    document.querySelector('#wifi-search-btn').addEventListener('submit', async (e) => {
        e.preventDefault();
        await fully_load_network();
    })
});


