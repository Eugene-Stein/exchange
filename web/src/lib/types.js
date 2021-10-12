// buy
// [
//   0,
//   "3.4",
//   "49000"
// ]
// sell
// [
//   1,
//   "3.4",
//   "49000"
// ]
const Side = Object.freeze({
  Buy: 0,
  Sell: 1,
});
const Order = Object.freeze({
  side: Symbol("side"),
  amount: Symbol("amount"),
  price: Symbol("price"),
});
const OrderAttrMap = Object.freeze({
  [Order.side]: 0,
  [Order.amount]: 1,
  [Order.price]: 2,
})
function order_get(order, attr) {
  return order[OrderAttrMap[attr]];
}
function order_set(order, attr, val) {
  order[OrderAttrMap[attr]] = val;
}
function order_create(side, amount, price) {
  return [side, amount, price];
}

export { Side, Order, order_get, order_set, order_create };