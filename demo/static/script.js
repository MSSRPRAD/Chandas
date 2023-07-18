    // Load the WASM module and instantiate it
    import init, {script_swap} from "./pkg/transliterate_ferris_wasm.js";

    init().then(() => {

document.getElementById('submitBtn').addEventListener('click', async function() {
    const indic = ["devanagari", "telugu", "kannada"];
    const roman = ["itrans", "iast", "slp1", "hk"];
    const inputData = document.getElementById('inputData').value;
    const t1_select = document.getElementById("t1-select").options[document.getElementById("t1-select").selectedIndex].text;
    let conversion;

    if (indic.includes(t1_select)) {
        conversion = 1;
    } else if (roman.includes(t1_select)) {
        conversion = 2;
    }


    // Use the 'script_swap' function to perform the transliteration
    const transliterated = script_swap(inputData, t1_select, 'slp1', conversion);
    console.log(transliterated);

            fetch('/api', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ inputData: transliterated })
            })
            .then(response => response.json())
            .then(data => {
                const matraContent = document.getElementById('matraContent');
                matraContent.innerText = data.matra;

                const vrttaContent = document.getElementById('vrttaContent');
                vrttaContent.innerText = data.vrtta;

                const anushtupContent = document.getElementById('anushtupContent');
                anushtupContent.innerText = data.anushtup;
            })
            .catch(error => console.error('Request failed:', error));


        });
    });
