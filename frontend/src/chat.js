import {Component, createRef} from "preact"
import {forgetName, getName, getWsUrl} from "./tools"
import ServerCon from "./ServerCon"
import ChessBoard, {CHESS_SIZE} from "./chess"
import ChessSelect from "./chessSelect"
import ChessClear from "./chessClear"
import ChatList from "./chatList"

export default class ChatPage extends Component {
    state = {
        users: [],
        messages: [],
        chess: [],
        input: "",
        isNetworkAvailable: true,
        isBlack: true,
    }
    chatListRef = createRef()

    constructor(props) {
        super(props)
        this.connection = new ServerCon(getWsUrl())
        this.connection.onMessage = (json) => {
            if (json["typ"] === "users") {
                this.setState({
                    users: json["users"],
                })
            } else if (json["typ"] === "msg") {
                window.focus()
                this.state.messages.push(json)
                this.setState({}, this.isAtBottom() ? this.scrollToBottom : undefined)
            } else if (json["typ"] === "chess") {
                this.setState({
                    chess: json["chess"],
                })
            } else if (json["typ"] === "network") {
                this.setState({
                    isNetworkAvailable: json["available"],
                })
            }
        }

        this.connection.onConnected = () => {
            this.connection.request({
                "typ": "name",
                "name": getName(),
            }).then((json) => {
                this.setState({
                        users: json["users"],
                        messages: json["messages"],
                        chess: json["chess"],
                        isNetworkAvailable: json["is_network_available"],
                    },
                    this.scrollToBottom,
                )
            })
        }
    }

    isAtBottom = () => {
        let list = this.chatListRef.current.base
        return Math.abs(list.scrollHeight - list.clientHeight - list.scrollTop) < 5
    }

    scrollToBottom = () => {
        let list = this.chatListRef.current.base
        list.scrollTop = list.scrollHeight - list.clientHeight
    }

    onSend = e => {
        e.preventDefault()
        if (this.state.input !== "") {
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
    }

    onInput = e => {
        this.setState({
            input: e.target.value,
        })
    }

    updateChess = () => {
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
    onChessClick = (x, y) => {
        this.state.chess[y * CHESS_SIZE + x] = this.state.isBlack
        this.updateChess()
    }

    onChessClear = () => {
        for (let i = 0; i < this.state.chess.length; i++) {
            this.state.chess[i] = null
        }
        this.updateChess()
    }

    onLogout = () => {
        forgetName()
        this.props.onUpdateLogin()
    }

    componentWillUnmount() {
        this.connection.reconnect = false
        this.connection.disconnect()
    }

    render() {
        return <div class="chat-page-layout">
            <div class="online-user-column">
                <div class="network-indicator"
                     style={{backgroundColor: this.state.isNetworkAvailable ? "#008000" : "#b00020"}}>
                    {this.state.isNetworkAvailable ? "Internet Available" : "Internet Unavailable"}
                </div>
                <button onclick={this.onLogout}>
                    Logout
                </button>
                <h2>Online Users</h2>
                <div>
                    {this.state.users.map((name) => <p key={name}>{name}</p>)}
                </div>
            </div>
            <div class="chat-column" style={{flexGrow: 1}}>
                <ChatList ref={this.chatListRef} messages={this.state.messages}/>
                <form class="input-bar" onSubmit={this.onSend}>
                    <input type="text" value={this.state.input} onInput={this.onInput}/>
                    <button type="submit">Send</button>
                </form>
            </div>
            <div class="chess-board-column">
                <ChessClear onClick={this.onChessClear}/>
                <ChessBoard data={this.state.chess} onClick={this.onChessClick}/>
                <ChessSelect isBlack={this.state.isBlack}
                             onChange={(isBlack) => this.setState({isBlack: isBlack})}/>
            </div>
        </div>
    }
}
