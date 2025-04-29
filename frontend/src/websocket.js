 
export class WebSocketClient {
    constructor(url, onMessage) {
      this.url = url;
      this.onMessage = onMessage;
      this.connect();
    }
  
    connect() {
      this.ws = new WebSocket(this.url);
      this.ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        this.onMessage(data);
      };
      this.ws.onclose = () => {
        setTimeout(() => this.connect(), 5000);
      };
      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };
    }
  }