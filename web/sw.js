// The service worker: serves the site from its cache first, so it works offline.
// cargo xtask web fills in VERSION, a hash of the site's files, and FILES, the files;
// a new version installs beside the old one, and replaces it once no page uses the old one.
const VERSION = "dev";
const FILES = [];
const CACHE = `grafia-${VERSION}`;

self.addEventListener("install", (e) => {
  // past the HTTP cache, which may still hold the previous version's files
  const requests = FILES.map((file) => new Request(file, { cache: "reload" }));
  e.waitUntil(caches.open(CACHE).then((cache) => cache.addAll(requests)));
});

self.addEventListener("activate", (e) => {
  // only this site's old versions: other sites on the same origin (github.io) have caches too
  e.waitUntil(caches.keys().then((keys) =>
    Promise.all(keys.filter((key) => key.startsWith("grafia-") && key !== CACHE).map((key) => caches.delete(key)))));
});

self.addEventListener("fetch", (e) => {
  // ignoreSearch: the page also opens from a link with a query string (?utm_source=...)
  e.respondWith(caches.match(e.request, { cacheName: CACHE, ignoreSearch: true })
    .then((hit) => hit || fetch(e.request)));
});
