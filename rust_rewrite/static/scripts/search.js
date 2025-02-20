document.addEventListener('DOMContentLoaded', () => {
    const searchInput = document.getElementById("search-input");
    // const searchButton = document.getElementById("search-button");

    // Focus the input field on page load so the user can immediately start typing.
    searchInput.focus();

    // Search when the user presses Enter.
    searchInput.addEventListener('keypress', (event) => {
        if (event.key === 'Enter') {
            makeSearchRequest();
        }
    });

    // Search when the user clicks the search button.
    // searchButton.addEventListener('click', makeSearchRequest);
});

async function makeSearchRequest() {
    const query = document.getElementById("search-input").value;
    const response = await fetch(`/api/v1/search?q=${encodeURIComponent(query)}`); // This supposedly also needs a 'language' parameter, but I can't find it in the legacy code. Does it mean programming language or human language?
    const searchResults = await response.json();
    console.log(searchResults);

    const resultsDiv = document.getElementById("results");
    resultsDiv.innerHTML = ''; // Clear the previous search results.

    searchResults.data.forEach(result => {
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