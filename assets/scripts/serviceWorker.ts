const IS_DEV = process.env.NODE_ENV === `development`;

const DEPRECATED_VERSIONS = [];
const CURRENT_VERSION = "v1";

function dev_logger(log) {
  if (IS_DEV) {
    console.log(log);
  }
}

async function addResourcesToCache(resources) {
  const cache = await caches.open(CURRENT_VERSION);
  dev_logger(`Adding resources to caches: ${resources}`);
  await cache.addAll(resources);
}

async function cacheFirst({ request, preloadResponsePromise, fallbackUrl }) {
  const responseFromCache = await caches.match(request);
  if (responseFromCache) {
    dev_logger(`Cache hit: ${responseFromCache}`);
    return responseFromCache;
  }

  // Next try to use (and cache) the preloaded response, if it's there
  const preloadResponse = await preloadResponsePromise;
  if (preloadResponse) {
    dev_logger(`Using preupload response: ${preloadResponse}`);
    putInCache(request, preloadResponse.clone());
    return preloadResponse;
  }

  try {
    const responseFromNetwork = await fetch(request);
    // response may be used only once
    // we need to save clone to put one copy in cache
    // and serve second one
    putInCache(request, responseFromNetwork.clone());
    dev_logger(`response from network: ${responseFromNetwork}`);
    return responseFromNetwork;
  } catch (error) {
    const fallbackResponse = await caches.match(fallbackUrl);
    if (fallbackResponse) {
      dev_logger(`fallback response: ${fallbackResponse}`);
      return fallbackResponse;
    }
    // when even the fallback response is not available,
    // there is nothing we can do, but we must always
    // return a Response object
    return new Response("Network error", {
      status: 408,
      headers: { "Content-Type": "text/plain" },
    });
  }
}

async function deleteCache(key) {
  dev_logger(`deleting cache key: ${key}`);
  await caches.delete(key);
}

async function deleteOldCaches() {
  const cacheKeepList = [CURRENT_VERSION];
  const keyList = await caches.keys();
  const cachesToDelete = keyList.filter((key) => !cacheKeepList.includes(key));
  await Promise.all(cachesToDelete.map(deleteCache));
}

// Enable navigation preload
async function enableNavigationPreload() {
  if (self.registration.navigationPreload) {
    await self.registration.navigationPreload.enable();
  }
}

async function putInCache(request, response) {
  const cache = await caches.open(CURRENT_VERSION);
  dev_logger(`placing in cache: \n ${request} \n ${response}`);
  await cache.put(request, response);
}

self.addEventListener("activate", (event) => {
  event.waitUntil(deleteOldCaches());
  event.waitUntil(enableNavigationPreload());
});

self.addEventListener("install", (event) => {
  event.waitUntil(addResourcesToCache(["/", "/assets/bootstrap.css", "/assets/bootstrap.js"]));
});

self.addEventListener("fetch", (event) => {
  dev_logger(`fetch event: \n${event}\n`);
  event.respondWith(
    cacheFirst({
      request: event.request,
      preloadResponsePromise: event.preloadResponse,
      fallbackUrl: `/logo.svg`,
    }),
  );
});

// https://javascript.plainenglish.io/push-notifications-using-a-service-worker-5f2e371774e
// self.addEventListener("push", (e) => {
//   console.log({ e });
//   self.registration.showNotification("Wohoo!!", { body: e.data.text() });
// });
