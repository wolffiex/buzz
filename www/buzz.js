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
    .then(obj => {
        instanceExports = obj.instance.exports;
        console.log('instance E', instanceExports);
        window.wasm = obj;
        console.log('obj', obj);
        console.log(obj.instance);
        console.log(obj.instance.exports);

        const add = obj.instance.exports.add;
        console.log('add 2 + 3', add(2, 3));
    });

//.catch(err => console.error(err));

function write(id, index, props) {
    newId = buzz.newId();
    buzz.write(id, edgeName, props);
    return newId;
}