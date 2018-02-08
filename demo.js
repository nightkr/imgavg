fetch('target/wasm32-unknown-unknown/release/imgavg.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {}))
    .then(wasm => {
        window.wasm = wasm;
        console.log(wasm);
        for(let image of document.querySelectorAll(".image")) {
            let src = image.children[0].src;
            fetch(src)
                .then(response => response.arrayBuffer())
                .then(bytesBuf => {
                    let bytes = new Uint8Array(bytesBuf);
                    let bytesPtr = wasm.instance.exports.alloc(bytesBuf.byteLength);
                    let mem = new Uint8Array(wasm.instance.exports.memory.buffer, bytesPtr, bytesBuf.byteLength);
                    for (let i = 0; i < bytesBuf.byteLength; i++) {
                        mem[i] = bytes[i];
                    }
                    let colorPtr = wasm.instance.exports.calculate_background(bytesPtr, bytesBuf.byteLength);
                    let color = copyCStr(wasm.instance.exports, colorPtr);
                    image.style.backgroundColor = color;
                });
        }
    });
