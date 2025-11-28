document.getElementById("weather-form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const city = document.getElementById("weather-city").value;

    const response = await fetch("/api/weather", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ city, timestamp: Math.floor(Date.now() / 1000) }),
    });

    const data = await response.json();

    for (let [key, value] of Object.entries(data)) {
        console.log(`${key}: ${value}`);
    }

    const result = document.getElementById("weather-result");
    result.innerHTML = `
        <h2>${city}</h2>
        <p>Temperature: ${data.temp}°C</p>
        <p>Humidity: ${data.humidity}%</p>
        <p>Wind Speed: ${data.wind_speed} m/s</p>
    `;
})
