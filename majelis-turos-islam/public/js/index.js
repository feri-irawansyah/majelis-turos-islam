// src/js/init.js

document.addEventListener("DOMContentLoaded", function () {
    // if (window.Swiper) {
    //     const swiper = new Swiper(".mySwiper", {
    //         navigation: {
    //             nextEl: ".swiper-button-next",
    //             prevEl: ".swiper-button-prev",
    //         },
    //     });
    // }

    if (window.AOS) {
        AOS.init({
            once: false,
            mirror: true,
            duration: 800,
        });
    }

    console.log("✅ Semua library JS sudah di-init");
});
