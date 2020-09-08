import uuid from "uuid-random"

export default class ServerCon {
    queue = []
    pendingReqs = new Map()
    onmessage = (_) => {
    }

    constructor(addr) {
        this.ws = new WebSocket(addr)

        this.ws.onerror = () => {
            alert("Websocket error")
        }
        this.ws.onopen = () => {
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
                this.onmessage(json)
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
