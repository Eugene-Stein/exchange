<script>
  export let symbol;
  export let base;
  export let quote;
  export let price;
  import * as _ from "./lib/data";
  import { init } from "klinecharts";
  import { order_create, Side } from "./lib/types";

  const chart = init("chart");

  chart.applyNewData([
    {
      close: 4976.16,
      high: 4977.99,
      low: 4970.12,
      open: 4972.89,
      timestamp: 1587660000000,
      volume: 204
    },
    {
      close: 4977.33,
      high: 4979.94,
      low: 4971.34,
      open: 4973.2,
      timestamp: 1587660060000,
      volume: 194
    },
    {
      close: 4977.93,
      high: 4977.93,
      low: 4974.2,
      open: 4976.53,
      timestamp: 1587660120000,
      volume: 197
    },
    {
      close: 4966.77,
      high: 4968.53,
      low: 4962.2,
      open: 4963.88,
      timestamp: 1587660180000,
      volume: 28
    },
    {
      close: 4961.56,
      high: 4972.61,
      low: 4961.28,
      open: 4961.28,
      timestamp: 1587660240000,
      volume: 184
    },
    {
      close: 4964.19,
      high: 4964.74,
      low: 4961.42,
      open: 4961.64,
      timestamp: 1587660300000,
      volume: 191
    },
    {
      close: 4968.93,
      high: 4972.7,
      low: 4964.55,
      open: 4966.96,
      timestamp: 1587660360000,
      volume: 105
    },
    {
      close: 4979.31,
      high: 4979.61,
      low: 4973.99,
      open: 4977.06,
      timestamp: 1587660420000,
      volume: 35
    },
    {
      close: 4977.02,
      high: 4981.66,
      low: 4975.14,
      open: 4981.66,
      timestamp: 1587660480000,
      volume: 135
    },
    {
      close: 4985.09,
      high: 4988.62,
      low: 4980.3,
      open: 4986.72,
      timestamp: 1587660540000,
      volume: 76
    }
  ]);

  let input_price = price;
  let input_amount_base = "3.14";
  $: input_amount_quote = `${input_amount_base * price}`;
  function put_order(side) {
    const order = order_create(side, input_amount_base, input_price);
    window.client.put_order(order);
  }
  function buy() {
    put_order(Side.Buy);
  }
  function sell() {
    put_order(Side.Sell);
  }
</script>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>

<main>
  <h1>Exchange</h1>
  <h3>Spot {symbol}</h3>
  <h3>Last Price {price}</h3>
  <div>
    <input type="number" placeholder="price" bind:value={input_price} />
    <input
      type="number"
      placeholder="amount in {base}"
      bind:value={input_amount_base} />
    <input
      type="number"
      placeholder="amount in {quote}"
      bind:value={input_amount_quote} />
    <button on:click={buy}>Buy</button>
    <button on:click={sell}>Sell</button>
  </div>
</main>
