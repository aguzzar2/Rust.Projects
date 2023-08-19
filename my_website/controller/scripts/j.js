document.addEventListener("DOMContentLoaded", function() {
    // Get the current URL
    const currentURL = window.location.pathname;

    // Get the navigation container
    const navContainer = document.getElementById("nav-container");

    // Loop through each button inside the navigation container
    const navButtons = navContainer.querySelectorAll(".nav__links-button");
    const homeButtons = navContainer.querySelectorAll(".homepage__button");

    homeButtons.forEach(button => {
        // Get the href attribute of the button
        const href = button.parentNode.getAttribute("action"); // Get the form's action attribute

        // Check if the href matches the current URL
        if (href === currentURL) {
            // Add the active class to the button
            button.classList.add("active");
        }
    });

    navButtons.forEach(button => {
        // Get the href attribute of the button
        const href = button.parentNode.getAttribute("action"); // Get the form's action attribute

        // Check if the href matches the current URL
        if (href === currentURL) {
            // Add the active class to the button
            button.classList.add("active");
        }
    });
});
