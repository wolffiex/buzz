var importObject = {
    imports: {
        logOne: arg => {
            console.log("one", arg);
            return true;
        }
    }
};

async function handleErrors(response) {
    if (!response.ok) {
        let resp = await response.text();
        document.body.innerHTML = "<h1>WASM error</h1><pre>" + resp + "</pre>";
        throw new Error(response.statusText);
    }
    return response;
}

fetch('/wasm-latest').then(handleErrors)
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, importObject))
    .then(obj => {
        window.wasm = obj;
        console.log('obj', obj);
        console.log(obj.instance);
        console.log(obj.instance.exports);
        x = obj.instance.exports.main();
        console.log('x', x);

        const add = obj.instance.exports.add;
        console.log('add', add(2, 3));
    })
    .catch(err => console.error(err));

function write(id, index, props) {
    newId = buzz.newId();
    buzz.write(id, edgeName, props);
    return newId;
}