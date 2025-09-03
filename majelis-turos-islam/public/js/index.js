// src/js/init.js

// Bootstrap biasanya auto jalan kalo pake bundle.min.js
// Kalau perlu config custom taro di sini

// AOS
if (window.AOS) {
    AOS.init({
        duration: 600,
        once: true,
    });
}

// // Tambahin init library lain di bawah
// // Swiper example
// if (window.Swiper) {
//   const swiper = new Swiper(".swiper-container", {
//     loop: true,
//     autoplay: { delay: 3000 },
//   });
// }

// // Chart.js example
// if (window.Chart) {
//   const ctx = document.getElementById("myChart");
//   if (ctx) {
//     new Chart(ctx, {
//       type: "bar",
//       data: { labels: ["A", "B"], datasets: [{ data: [12, 19] }] },
//     });
//   }
// }

console.log("✅ Semua library JS sudah di-init");
