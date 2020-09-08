import {rndFactory} from "./tools"

let nameColorMap = new Map()

function getNameColor(name) {
    if (nameColorMap.has(name)) {
        return nameColorMap.get(name)
    } else {
        let rnd = rndFactory(name)
        let color = `rgb(${rnd() * 150}, ${rnd() * 150}, ${rnd() * 150})`
        nameColorMap.set(name, color)
        return color
    }
}

export default function ChatList(props) {
    let date
    return <div ref={props.ref} class="chat-list">
        {props.messages.map((msg) => <p key={msg}>
            <span class="time-span">{(date = new Date(msg.time),
                `${date.getMonth() + 1}/${date.getDate()} ${date.getHours() < 10 ? ("0" + date.getHours()) : date.getHours()}:${date.getMinutes() < 10 ? ("0" + date.getMinutes()) : date.getMinutes()} `)}</span>
            <span style={{color: getNameColor(msg.name)}}>{msg.name}</span>
            <span>{": " + msg.text}</span>
        </p>)}
    </div>
}
