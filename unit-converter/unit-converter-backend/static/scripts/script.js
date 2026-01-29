const resultContaner = document.getElementById("result")
const unitList = document.getElementById("unit-list")
const inputContainer = document.getElementById("input-section")

function resultLabel() {
    return resultContaner.children[0]
}
function resultValue() {
    return resultContaner.children[1]
}
function resultReset() {
    return resultContaner.children[2]
}
function resultCopy() {
    return resultContaner.children[3]
}

function hideAllInputs() {
    for (let i = 0; i < inputContainer.children.length; i++) {
        const input = inputContainer.children[i];
        input.style.display = "none";
    }
}

resultReset().addEventListener("click", (e) => {
    e.preventDefault()

    resultContaner.style.display = "none";
    resultLabel().innerText = "Result:";
    resultValue().innerText = "";
})

resultCopy().addEventListener("click", async (e) => {
    e.preventDefault()
    await navigator.clipboard.writeText(resultValue().innerText)
})

function unitsSetup() {
    for (let i = 0; i < unitList.children.length; i++) {
        const button = unitList.children[i].children[0];
        const name = button.innerText.toLowerCase();
        console.log(name)
        unitSetup(name)
    }
}

function unitSetup(name) {
    const formContainer = document.getElementById(name + "-form");
    const formValue = document.getElementById(name + "-convert");
    const formFrom = document.getElementById(name + "-from");
    const formTo = document.getElementById(name + "-to");

    const formButton = document.getElementById(name + "-button");
    formButton.addEventListener("click", async (e) => {
        e.preventDefault()

        const from = formFrom.value;
        const to = formTo.value;
        const value = formValue.value;

        const response = await fetch("/api/convert", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                from: from,
                to: to,
                value: parseFloat(value),
            }),
        })

        if (response.status === 200) {
            alert("Success")
        } else {
            alert("Error: " + response.status)
            return
        }

        const data = await response.json();
        resultContaner.style.display = "block";
        resultLabel().innerText = "Result:";
        resultValue().innerText = data.value;
    });

    const formItem = document.getElementById(name + "-item");
    formItem.addEventListener("click", (e) => {
        e.preventDefault()
        hideAllInputs()
        formContainer.style.display = "block";
    })
}

unitsSetup()
