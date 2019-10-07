addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const landing = `
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>科目n</title>
    </head>
    <body>
        <p id="output"></p>
        <script type="text/javascript">
            var count = 1;
            async function update() {
                var result = await fetch(window.location.pathname, {
                    method: 'POST',
                    headers: { 'Content-Type': 'text/plain' },
                    body: count.toString()
                });
                document.getElementById("output").innerHTML += '科目' + await result.text() + '<br />';
                count += 1;
                window.scrollTo(0,document.body.scrollHeight);
                setTimeout(function() {
                    update();
                }, 1000);
            }
            update();
        </script>
    </body>
</html>
`

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    let response;
    const { convert_s } = wasm_bindgen;
    await wasm_bindgen(wasm);
    if (request.method === 'POST') {
        response = new Response(convert_s(parseInt(await request.text())), { headers: { 'Content-Type': 'text/plain' } })
    } else {
        response = new Response(landing, { headers: { 'Content-Type': 'text/html' } })
    }
    return response;
}
