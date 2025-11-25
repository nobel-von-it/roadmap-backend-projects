const unitList = document.getElementById("unit-list");
const resultContainer = document.getElementById("result");

const inputSections = document.getElementsByClassName("unit-form");

const lengthButton = document.getElementById("length-button");
const weightButton = document.getElementById("weight-button");
const temperatureButton = document.getElementById("temperature-button");

const resetButton = document.getElementById("reset-button");
const copyButton = document.getElementById("copy-button");

let currentFormIdx = 0;

for (let i = 0; i < unitList.children.length; i++) {
    unitList.children.item(i).addEventListener("click", (e) => {
        e.preventDefault();
        updateUnit(i);
    });
}

for (let i = 0; i < inputSections.length; i++) {
    addEventListenerToButtonByType(inputSections.item(i).id.split("-")[0]);
}

resetButton.addEventListener("click", (e) => {
    e.preventDefault();
    inputSections.item(currentFormIdx).style.display = "block";
    resultContainer.style.display = "none";
    for (let i = 0; i < inputSections.length; i++) {
        inputSections.item(i).reset();
    }
    inputSections.item(currentFormIdx).style.display = "block";
})

copyButton.addEventListener("click", (e) => {
    e.preventDefault();
    navigator.clipboard.writeText(document.getElementById("result-value").innerHTML);
})

function formatResult(value, from, to, result) {
    return value.toString() + " " + from + " = " + result.toString() + " " + to;
}

function addEventListenerToButtonByType(type) {
    const form = document.getElementById(type + "-form");
    form.addEventListener("submit", (e) => {
        e.preventDefault();
        calculateAndUpdate(type);
    });
}

function calculateAndUpdate(type) {
    calculate(type);
    inputSections.item(currentFormIdx).style.display = "none";
}

function calculate(type) {
    const value = document.getElementById(type + "-convert").value;
    const from = document.getElementById(type + "-from").value;
    const to = document.getElementById(type + "-to").value;
    const result = getResult(type, value, from, to);
    updateResult(value, from, to, result);
}

function getResult(type, value, from, to) {
    switch (type) {
        case "length":
            return getLengthResult(value, from, to);
        case "weight":
            return getWeightResult(value, from, to);
        case "temperature":
            return getTemperatureResult(value, from, to);
        default:
            break;
    }
}

function getTemperatureResult(value, from, to) {
    const fromTo = from + "|" + to;
    switch (fromTo) {
        case "C|F":
            return value * 1.8 + 32;
        case "F|C":
            return (value - 32) / 1.8;
        default:
            return value;
    }
}

function getWeightResult(value, from, to) {
    const fromTo = from + "|" + to;
    switch (fromTo) {
        case "kg|lb":
            return value * 2.20462;
        case "lb|kg":
            return value / 2.20462;
        default:
            return value;
    }
}

function getLengthResult(value, from, to) {
    const fromTo = from + "|" + to;
    switch (fromTo) {
        case "cm|m":
            return value / 100;
        case "m|cm":
            return value * 100;
        case "cm|km":
            return value / 100000;
        case "km|cm":
            return value * 100000;
        case "m|km":
            return value / 1000;
        case "km|m":
            return value * 1000;
        default:
            return value;
    }
}

function updateUnit(i) {
    inputSections.item(currentFormIdx).style.display = "none";
    currentFormIdx = i;
    inputSections.item(currentFormIdx).style.display = "block";
}

function updateResult(value, from, to, result) {
    resultContainer.style.display = "block";
    const resultLabel = document.getElementById("result-label");
    const resultValue = document.getElementById("result-value");

    resultLabel.innerHTML = "Result of calculation:";
    resultValue.innerHTML = formatResult(value, from, to, result);
}
