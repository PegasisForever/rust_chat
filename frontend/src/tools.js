export function setName(name) {
    localStorage.setItem("name", name)
}

export function getName() {
    return localStorage.getItem("name")
}

export function forgetName(){
    return localStorage.removeItem("name")
}
