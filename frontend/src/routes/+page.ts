// Since we use browser-only fetch requests and interactive libraries (like Chart.js/Dex), 
// let's disable SSR (Server Side Rendering) or prerendering on this page so it is entirely client-rendered.
export const ssr = false;
export const prerender = false;
