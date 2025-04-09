document.addEventListener("DOMContentLoaded", async () => {
  try {
    const response = await fetch("/api/weather");
    const data = await response.json();

    if (!response.ok) {
      throw new Error("Failed to fetch weather data");
    }

    const container = document.getElementById("weather-container");

    data.forecast.forecastday.forEach((day) => {
      const dayDiv = document.createElement("div");
      dayDiv.style.marginBottom = "10px";
      dayDiv.innerHTML = `
                <h3>${day.date}</h3>
                <p>High: ${day.day.maxtemp_c}°C / Low: ${day.day.mintemp_c}°C</p>
                <p>${day.day.condition.text}</p>
            `;
      container.appendChild(dayDiv);
    });
  } catch (error) {
    console.error("Error:", error);
  }
});
