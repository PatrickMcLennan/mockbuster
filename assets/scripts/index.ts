// Injected via webpack
// declare const SUBSCRIPTION_PUBLIC_KEY: string;

const registerServiceWorker = async () => {
  const applicationServerKey = SUBSCRIPTION_PUBLIC_KEY;
  console.log("Encoded Application Server Key:", applicationServerKey); // Add this line for debugging
  if ("serviceWorker" in navigator && "Notification" in window) {
    try {
      Notification.requestPermission();
      const registration = await navigator.serviceWorker.register("/assets/serviceWorker.js");

      const subscription = await registration.pushManager.subscribe({
        userVisibleOnly: true,
        applicationServerKey,
      });

      console.log({ subscription });

      // fetch("/subscribe", {
      //     method: "POST",
      //     body: JSON.stringify(subscription),
      //     headers: {
      //         "Content-Type": "application/json",
      //     },
      // });
    } catch (error) {
      console.error(`Registration failed with ${error}`);
    }
  }
};

registerServiceWorker();
