import { encode, decode } from "@msgpack/msgpack";
import App from './App.svelte';
import { main } from "./lib/codec";
import { Client } from "./lib/client";

const app = new App({
	target: document.body,
	props: {
		symbol: "BTC/USDT",
		base: "BTC",
		quote: "USDT",
		price: "41000"
	}
});

const client = new Client(`WS_URL`);
window.client = client;

window.encode = encode;
window.decode = decode;
main();
export default app;