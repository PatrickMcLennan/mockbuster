// Injected via webpack
// declare const SUBSCRIPTION_PUBLIC_KEY: string;

const registerServiceWorker = async () => {
  const applicationServerKey = SUBSCRIPTION_PUBLIC_KEY;
  if ("serviceWorker" in navigator && "Notification" in window) {
    try {
      Notification.requestPermission();
      const registration = await navigator.serviceWorker.register("/assets/serviceWorker.js");
      const existingSubscription = await registration.pushManager.getSubscription();

      if (!existingSubscription) {
        const subscription = await registration.pushManager.subscribe({
          userVisibleOnly: true,
          applicationServerKey,
        });

        const p256ArrayBuffer = subscription.getKey(`p256dh`);
        const p256Uint8Array = new Uint8Array(p256ArrayBuffer);
        const p256NumberArray = Array.from(p256Uint8Array);
        const p256Base64 = btoa(String.fromCharCode.apply(null, p256NumberArray));

        const authKeyArrayBuffer = subscription.getKey("auth");
        const authKeyUint8Array = new Uint8Array(authKeyArrayBuffer);
        const authKeyNumberArray = Array.from(authKeyUint8Array);
        const authKeyBase64 = btoa(String.fromCharCode.apply(null, authKeyNumberArray));

        fetch("/subscribe", {
          method: "POST",
          body: JSON.stringify({
            endpoint: subscription.endpoint,
            p256: p256Base64,
            auth: authKeyBase64,
          }),
          headers: {
            "Content-Type": "application/json",
          },
        });
      }
    } catch (error) {
      console.error(`Registration failed with ${error}`);
    }
  }
};

registerServiceWorker();
