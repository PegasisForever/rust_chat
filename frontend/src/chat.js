import {Component} from "preact"
import {getName} from "./tools"

export default class ChatPage extends Component {
    render(){
        return <div>
            {getName()}
        </div>
    }
}
