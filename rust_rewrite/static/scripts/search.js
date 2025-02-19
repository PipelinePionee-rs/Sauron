document.addEventListener('DOMContentLoaded', () => {
    const searchInput = document.getElementById("search-input");

    // Focus the input field on page load so the user can immediately start typing.
    searchInput.focus();

    // Search when the user presses Enter.
    searchInput.addEventListener('keypress', (event) => {
        if (event.key === 'Enter') {
            makeSearchRequest();
        }
    });
});

async function makeSearchRequest() {
    const query = document.getElementById("search-input").value;
    const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`); // This supposedly also needs a 'language' parameter, but I can't find it in the legacy code. Does it mean programming language or human language?
    const searchResults = await response.json();

    const resultsDiv = document.getElementById("results");
    resultsDiv.innerHTML = ''; // Clear the previous search results.

    searchResults.forEach(result => {
        const resultDiv = document.createElement('div');
        const title = document.createElement('h2');
        const link = document.createElement('a');
        const description = document.createElement('p');

        link.href = result.url;
        link.textContent = result.title;
        link.className = 'search-result-title';

        title.appendChild(link);
        description.textContent = result.description;
        description.className = 'search-result-description';

        resultDiv.appendChild(title);
        resultDiv.appendChild(description);
        resultsDiv.appendChild(resultDiv);
    });
}