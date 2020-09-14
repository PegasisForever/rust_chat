import uuid from "uuid-random"

export default class ServerCon {
    queue = []
    pendingReqs = new Map()
    onMessage = (_) => {
    }
    onConnected = () => {
    }
    reconnect = true
    reconnectCount = 0

    constructor(addr) {
        this.setupWs(addr)
    }

    setupWs(addr) {
        if (this.ws) this.ws.close()
        this.ws = new WebSocket(addr)

        this.ws.onclose = (ev) => {
            if (this.reconnect && this.reconnectCount < 10) {
                console.error(ev)
                this.reconnectCount++
                this.setupWs(addr)
            } else if (this.reconnectCount >= 10) {
                alert("Failed to reconnect, please refresh.")
            }
        }
        this.ws.onerror = (ev) => {
            console.error(ev)
            this.setupWs(addr)
        }
        this.ws.onopen = () => {
            console.log("ws connected")
            this.onConnected()
            this.queue.forEach((json) => {
                this.ws.send(JSON.stringify(json))
            })
            this.queue = []
        }
        this.ws.onmessage = (e) => {
            let json = JSON.parse(e.data)
            let id = json["id"]
            let resolutionFunc = this.pendingReqs.get(id)
            if (resolutionFunc) {
                resolutionFunc(json)
            } else {
                this.onMessage(json)
            }
        }
    }

    request(json) {
        return new Promise((resolutionFunc, _) => {
            const id = uuid()
            this.pendingReqs.set(id, resolutionFunc)

            json["id"] = id
            if (this.ws.readyState !== 1) {
                this.queue.push(json)
            } else {
                this.ws.send(JSON.stringify(json))
            }
        })
    }

    disconnect() {
        this.ws.close()
    }
}
