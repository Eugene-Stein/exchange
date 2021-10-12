import { encode, decode } from "@msgpack/msgpack";
import { order_create, Side } from "./types";


function main() {
  const buy = order_create(Side.Buy, "3.4", "49000");
  const sell = order_create(Side.Sell, "3.4", "49000");

  const buy_encoded = encode(buy);
  const sell_encoded = encode(sell);

  console.log(buy);
  console.log(buy_encoded);
  console.log(sell);
  console.log(sell_encoded);

  const buy_decoded = decode(buy_encoded);
  const sell_decoded = decode(sell_encoded);

  console.log(buy_decoded);
  console.log(sell_decoded);
}

export { main };