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

        wasm.instance.exports.init_records();
        write("a", "b");
        write("c", "d");
        let x = read("a");

        console.log("read a", x);
        console.log("read c", read("c"));
        console.log("read eee", read("eee"));
    });

//.catch(err => console.error(err));
function toCString(s) {
    const termLength = s.length + 1;
    var p = instanceExports.alloc(termLength);
    var m = new Uint8Array(instanceExports.memory.buffer, p, termLength);
    for (var i = 0; i < s.length; i++)
        m[i] = s.charCodeAt(i);
    m[s.length] = 0;
    return p;
}

function write(k, v) {
    wasm.instance.exports.write( toCString(k), toCString(v));
}

function read(k) {
    let p = wasm.instance.exports.read(toCString(k));
    return fromCString(p);
}

function fromCString(ptr) {
    var m = new Uint8Array(instanceExports.memory.buffer, ptr);
    var s = "";
    let i = 0;
    while (m[i] != 0) {
        s += String.fromCharCode(m[i++]);
    }
    wasm.instance.exports.free_cstring(ptr);
    return s;
}
