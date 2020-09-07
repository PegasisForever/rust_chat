import "./style"
import LoginPage from "./login"
import {Component} from "preact"
import {getName} from "./tools"
import ChatPage from "./chat"

export default class App extends Component {
    state = {
        loggedIn: getName() !== null,
    }

    onUpdateLogin = () => {
        this.setState({
            loggedIn: getName() !== null,
        })
    }

    render(_, {loggedIn}) {
        return <div>
            {loggedIn ?
                <ChatPage onUpdateLogin={this.onUpdateLogin}/> :
                <LoginPage onUpdateLogin={this.onUpdateLogin}/>}
        </div>
    }
}
