import { Tangle } from "../../../tangle_ts/dist/tangle.js"

const canvas = document.getElementById("myCanvas");
const context = canvas.getContext("2d");

export const SCREEN_WIDTH = 160;
export const SCREEN_HEIGHT = 144;

let framebufferPtr = 0;

async function run() {
    const imports = {
        env: {
            log_u32: function (v) {
                console.log(v);
            },
            set_fb_ptr: function (ptr) {
                framebufferPtr = ptr;
            },
        }
    };

    if (canvas.width != SCREEN_WIDTH || canvas.height != SCREEN_HEIGHT) {
        canvas.width = SCREEN_WIDTH;
        canvas.height = SCREEN_HEIGHT;
    }

    const result = await Tangle.instantiateStreaming(fetch("juice.wasm"), imports);
    const exports = result.instance.exports;
    // const memory = result.tangle._time_machine._wasm_instance.instance.exports.memory;
    // console.log(exports);

    const keys = {
        'ArrowUp': 0,
        'ArrowDown': 1,
        'ArrowLeft': 2,
        'ArrowRight': 3,
        'KeyQ': 4,
        'KeyS': 5,
        'KeyA': 6,
        'KeyS': 7,
    }

    canvas.onkeydown = (event) => {
        let code = keys[event.code];
        console.log(code, event.code, event.key);
        if (code !== undefined) {
            exports.set_button(code, true);
        }
    }

    canvas.onkeyup = (event) => {
        let code = keys[event.code];
        if (code !== undefined) {
            exports.set_button(code, false);
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

        window.requestAnimationFrame(animation);
    }

    animation();
}
run();