import { encode, decode } from "@msgpack/msgpack";
import { WS } from "./ws";

class Client {
  constructor(url) {
    this.ws = new WS(url);
  }
  put_order(order) {
    const encoded = encode(order);
    this.ws.send(encoded);
  }
}

export { Client };