const CACHE = "verba_ferri-v2";

const PRECACHE = [
  "index.html",
  "game.html",
  "style.css",
  "manifest.json",
  "icon.svg",
  "verba_ferri.js",
  "verba_ferri_bg.wasm",
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
