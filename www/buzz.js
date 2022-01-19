var instanceExports = null;
const importObject = {
    env: {
        consoleLog: function (ptr) {
            var m = new Uint8Array(instanceExports.memory.buffer);

            let s = "";
            while (m[ptr] != 0)
                s += String.fromCharCode(m[ptr++]);

            console.log('wasm:', s);
        },
        store: function (ptr) {
            //expects 128 bit ID followed by JSON string
        },
        chello: function () {
            var s = "Hello from JavaScript";
            const termLength = s.length + 1;
            var p = instanceExports.alloc(termLength);
            var m = new Uint8Array(instanceExports.memory.buffer, p, termLength);
            for (var i = 0; i < s.length; i++)
                m[i] = s.charCodeAt(i);
            m[s.length] = 0;
            return p;
        }
    }
};

async function handleErrors(response) {
    if (!response.ok) {
        let resp = await response.text();
        document.body.innerHTML = "<h1>WASM error</h1><pre>" + resp + "</pre>";
        throw new Error(resp);
    }
    return response;
}

fetch('/wasm-latest').then(handleErrors)
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, importObject))
    .then(wasm => {
        instanceExports = wasm.instance.exports;
        console.log('instance E', instanceExports);
        window.wasm = wasm;
        console.log('obj', wasm);
        console.log(wasm.instance);
        console.log(wasm.instance.exports);

        const add = wasm.instance.exports.add;
        console.log('add 2 + 3', add(2, 3));

        let h = wasm.instance.exports.get_handle()

        var s = '["b8877ff==", {name: "Book BUry", val:22}]';
        const termLength = s.length + 1;
        var p = instanceExports.alloc(termLength);
        var m = new Uint8Array(instanceExports.memory.buffer, p, termLength);
        for (var i = 0; i < s.length; i++)
            m[i] = s.charCodeAt(i);
        m[s.length] = 0;
        let hh = wasm.instance.exports.write(p, h);
        console.log("diid write", hh);
        wasm.instance.exports.drop_handle(hh);

    });

//.catch(err => console.error(err));

function write(id, index, props) {
    newId = buzz.newId();
    buzz.write(id, edgeName, props);
    return newId;
}