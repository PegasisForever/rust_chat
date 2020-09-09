export function setName(name) {
    localStorage.setItem("name", name)
}

export function getName() {
    return localStorage.getItem("name")
}

export function forgetName() {
    return localStorage.removeItem("name")
}

export function rndFactory(str) {
    let h = 1779033703 ^ str.length
    for (let i = 0; i < str.length; i++) {
        h = Math.imul(h ^ str.charCodeAt(i), 3432918353)
        h = h << 13 | h >>> 19
    }
    h = Math.imul(h ^ h >>> 16, 2246822507)
    h = Math.imul(h ^ h >>> 13, 3266489909)
    let seed = (h ^= h >>> 16) >>> 0

    return function () {
        let t = seed += 0x6D2B79F5
        t = Math.imul(t ^ t >>> 15, t | 1)
        t ^= t + Math.imul(t ^ t >>> 7, t | 61)
        return ((t ^ t >>> 14) >>> 0) / 4294967296
    }
}

export function getWsUrl() {
    if (!process.env.NODE_ENV || process.env.NODE_ENV === "development") {
        return "ws://localhost:8080"
    } else {
        return "ws://pega.local/ws"
    }
}
