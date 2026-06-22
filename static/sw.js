const CACHE = "verba-ferri-v1";

const PRECACHE = [
  "../index.html",
  "../game.html",
  "style.css",
  "manifest.json",
  "icon.svg",
  "../pkg/verba_ferri.js",
  "../pkg/verba_ferri_bg.wasm",
  "word-list.html",
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE).then((cache) => cache.addAll(PRECACHE)),
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k))),
    ),
  );
});

self.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then((cached) => cached || fetch(event.request)),
  );
});
