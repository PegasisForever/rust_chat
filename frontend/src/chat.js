import {Component, createRef} from "preact"
import {getName} from "./tools"
import ServerCon from "./ServerCon"
import ChessBoard, {CHESS_SIZE} from "./chess"
import ChessSelect from "./chessSelect"

export default class ChatPage extends Component {
    state = {
        users: [],
        messages: [],
        chess: [],
        input: "",
        isBlack: true,
    }
    chatListRef = createRef()

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
                this.setState({}, this.isAtBottom() ? this.scrollToBottom : undefined)
            } else if (json["typ"] === "chess") {
                this.setState({
                    chess: json["chess"],
                })
            }
        }

        this.connection.request({
            "typ": "name",
            "name": getName(),
        }).then((json) => {
            this.setState({
                    users: json["users"].sort(),
                    messages: json["messages"],
                    chess: json["chess"],
                },
                this.scrollToBottom,
            )
        })
    }

    isAtBottom = () => {
        let list = this.chatListRef.current
        return list.scrollTop === list.scrollHeight - list.clientHeight
    }

    scrollToBottom = () => {
        let list = this.chatListRef.current
        list.scrollTop = list.scrollHeight - list.clientHeight
    }

    onSend = e => {
        e.preventDefault()
        this.connection.request({
            "typ": "msg",
            "time": Date.now(),
            "text": this.state.input,
        }).then((json) => {
            this.state.messages.push(json)
            this.setState({}, this.isAtBottom() ? this.scrollToBottom : undefined)
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

    onChessClick = (x, y) => {
        this.state.chess[y * CHESS_SIZE + x] = this.state.isBlack
        this.connection.request({
            "typ": "chess",
            "time": Date.now(),
            "chess": this.state.chess,
        }).then((json) => {
            this.setState({
                chess: json["chess"],
            })
        })
    }

    render() {
        return <div class="chat-page-layout">
            <div>
                <h2>Online Users</h2>
                <div>
                    {this.state.users.map((name) => <p key={name}>{name}</p>)}
                </div>
            </div>
            <div class="chat-column" style={{flexGrow: 1}}>
                <div ref={this.chatListRef} class="chat-list">
                    {this.state.messages.map((msg) => <p key={msg}>{msg.name + ": " + msg.text}</p>)}
                </div>
                <form class="input-bar" onSubmit={this.onSend}>
                    <input type="text" value={this.state.input} onInput={this.onInput}/>
                    <button type="submit">Send</button>
                </form>
            </div>
            <div class="chess-board-column">
                <ChessBoard data={this.state.chess} onClick={this.onChessClick}/>
                <ChessSelect isBlack={this.state.isBlack}
                             onChange={(isBlack) => this.setState({isBlack: isBlack})}/>
            </div>
        </div>
    }
}
