window.onscroll = () => {
    updateScroll();
}

function updateScroll() {
    if (window.scrollY === 0) {
        document.getElementById("navbar").classList.add("fella-nav-no-border");
    } else {
        document.getElementById("navbar").classList.remove("fella-nav-no-border");
    }
}
