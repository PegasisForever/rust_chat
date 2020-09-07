import {Component} from "preact"
import {getName} from "./tools"
import ServerCon from "./ServerCon"

export default class ChatPage extends Component {
    state = {
        users: [],
        messages: [],
        input: "",
    }

    constructor(props) {
        super(props)
        this.connection = new ServerCon("ws://localhost:8080")
        this.connection.onmessage = (json) => {
            if (json["typ"] === "users") {
                this.setState({
                    users: json["users"].sort(),
                })
            } else if (json["typ"] === "msg") {
                this.state.messages.push(json)
                this.setState({})
            }
        }

        this.connection.request({
            "typ": "name",
            "name": getName(),
        }).then((json) => {
            this.setState({
                users: json["users"].sort(),
                messages: json["messages"],
            })
        })
    }

    onSend = e => {
        e.preventDefault()
        this.connection.request({
            "typ": "msg",
            "time": Math.floor(Date.now() / 1000), //todo use milliseconds
            "text": this.state.input,
        }).then((json) => {
            this.state.messages.push(json)
            this.setState({})
        })
        this.setState({
            input: "",
        })
    }

    onInput = e => {
        this.setState({
            input: e.target.value,
        })
    }

    render() {
        return <div>
            <div>
                <h2>Online Users</h2>
                <ul>
                    {this.state.users.map((name) => <li key={name}>{name}</li>)}
                </ul>
            </div>
            <div>
                <h2>Chat</h2>
                <ul>
                    {this.state.messages.map((msg) => <li key={msg}>{msg.name + ": " + msg.text}</li>)}
                </ul>
                <form onSubmit={this.onSend}>
                    <input type="text" value={this.state.input} onInput={this.onInput}/>
                    <button type="submit">Send</button>
                </form>
            </div>
        </div>
    }
}
