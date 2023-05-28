// import { Tangle } from "../../../tangle_ts/dist/tangle.js"
import { Tangle } from "./tangle.js"

const canvas = document.getElementById("myCanvas");
const context = canvas.getContext("2d");
const peerState = document.getElementById("peerState");

export const SCREEN_WIDTH = 160;
export const SCREEN_HEIGHT = 144;

let framebufferPtr = 0;

async function run() {
    if (canvas.width != SCREEN_WIDTH || canvas.height != SCREEN_HEIGHT) {
        canvas.width = SCREEN_WIDTH;
        canvas.height = SCREEN_HEIGHT;
    }

    let input_state = {};

    const imports = {
        env: {
            log_u32: function (v) {
                console.log(v);
            },
            set_fb_ptr: function (ptr) {
                framebufferPtr = ptr;
            },
            set_input_state: function (player_id, up, down, left, right, a, b, start, select) {
                input_state[player_id] = { up, down, left, right, a, b, start, select };
                // console.log(player_id, up, down, left, right, a, b, start, select)
            },
        }
    };

    const config = {
        room_name: "tanner-tojam2023"
    }

    const result = await Tangle.instantiateStreaming(fetch("juice.wasm"), imports, config);
    const exports = result.instance.exports;


    // const set_button = exports.set_button;
    // exports.set_button = (player_id, code, is_pressed) => {
    //     console.log(player_id, code, is_pressed);
    //     if (input_state[player_id] === undefined) {
    //         input_state[player_id] = [false, false, false, false, false, false, false, false];
    //     }

    //     input_state[player_id][code] = is_pressed;
    //     set_button(result.tangle.room.my_id, code, is_pressed)
    // }
    // const memory = result.tangle._time_machine._wasm_instance.instance.exports.memory;
    // console.log(exports);

    const keys = {
        'ArrowUp': 0,
        'ArrowDown': 1,
        'ArrowLeft': 2,
        'ArrowRight': 3,
        'KeyQ': 4, // A
        'KeyS': 5, // B
        'KeyA': 6, // START
        'KeyS': 7, // SELECT
        'KeyX': 4,
        'KeyZ': 5,
        'Enter': 6,
        'Space': 6,
        'Tab': 7,
    }

    canvas.onkeydown = (event) => {
        let code = keys[event.code];
        if (code !== undefined) {
            // console.log(result.tangle._room.my_id / 10, code, true);
            exports.set_button(result.tangle._room.my_id / 10, code, true);
        }
    }

    canvas.onkeyup = (event) => {
        let code = keys[event.code];
        if (code !== undefined) {
            exports.set_button(result.tangle._room.my_id / 10, code, false);
        }
    }

    async function animation() {
        context.clearRect(0, 0, context.canvas.width, context.canvas.height);

        // context.rect(0, 0, canvas.width, canvas.height);
        // context.fillStyle = "red";
        // context.fill();

        exports.framebuffer.callAndRevert();
        if (framebufferPtr !== 0) {
            const fb = result.tangle.read_memory_clamped(framebufferPtr, SCREEN_WIDTH * SCREEN_HEIGHT * 4);
            const img = new ImageData(fb, SCREEN_WIDTH, SCREEN_HEIGHT);
            context.putImageData(img, 0, 0);
        }

        let html = "<h1>Players</h1>";
        html += "<ul>";
        for (const [player_id, state] of Object.entries(input_state)) {
            html += "<li>"
            html += `Player #${player_id}: `
            for (const [key, isPressed] of Object.entries(state)) {
                if (isPressed) {
                    html += "<span style='color:red;'>";
                }
                html += `${key}, `;
                if (isPressed) {
                    html += "</span>"
                }
            }
            html += "</li>"
        }
        html += "</ul>";
        peerState.innerHTML = html;

        window.requestAnimationFrame(animation);
    }

    animation();
}
run();