document.addEventListener("DOMContentLoaded", function() {

    // Fetch the fragment and inject it into the page.
    // I assume we don't need to worry about compatibility with old browsers, re: fetch.
    function loadFragment(url, targetId) {
        fetch(url)
            .then(response => response.text())
            .then(data => {
                document.getElementById(targetId).innerHTML = data;
                if (targetId === 'header-container') {
                    updateNavLinks(); // For scalability, it's probably better to use a separate function instead of hardcoding this in an if statement.
                }
            })
            .catch(error => console.error('Error loading fragment:', error));
    }

    function updateNavLinks() {
        const navLinks = document.getElementById('nav-links');
        const user = getUser(); // Function to get user info from cookie.
        // TODO: change this; needs to contact a new endpoint instead, since we use HTTP-only cookies now.

        if (user) {
            navLinks.innerHTML = `
                <a id="nav-logout" href="/api/logout">Log out [${user.username}]</a>
            `;
        } else {
            navLinks.innerHTML = `
                <a id="nav-login" href="/login">Log in</a>
                <a id="nav-register" href="/register">Register</a>
            `;
        }
    }

/*    // Function to get user info from cookie.
    // It splits the cookie string into an array, then loops through the array to find the user cookie, or returns null if it doesn't exist.
    // I haven't had to work with cookies like this before, so I'm not a hundred percent sure this is the best solution.
    function getUser() {
        const name = 'user=';
        const decodedCookie = decodeURIComponent(document.cookie);
        const ca = decodedCookie.split(';');
        for(let i = 0; i < ca.length; i++) {
            let c = ca[i];
            while (c.charAt(0) === ' ') {
                c = c.substring(1);
            }
            if (c.indexOf(name) === 0) {
                return JSON.parse(c.substring(name.length, c.length));
            }
        }
        return null;
    }

    */

    loadFragment('/rust_rewrite/static/fragments/header.html', 'header-container');
    loadFragment('/rust_rewrite/static/fragments/footer.html', 'footer-container');
});