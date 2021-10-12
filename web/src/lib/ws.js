class WS {
  constructor(url) {
    this.url = url;
    this.websocket = null;
    this.init();
  }
  init() {
    if (this.websocket) {
      this.websocket.close();
    }
    this.websocket = new WebSocket(this.url);
    this.websocket.addEventListener("open", this.on_open);
    this.websocket.addEventListener("message", this.on_message);
    this.websocket.addEventListener("error", this.on_error);
    this.websocket.addEventListener("close", this.on_close);
  }
  on_open(event) {
    console.log(event);
  }
  on_message(event) {
    console.log(event);
  }
  on_error(event) {
    console.error(event);
  }
  on_close(event) {
    console.log(event);
  }
  send(arg) {
    return this.websocket.send(arg);
  }

}

export { WS };